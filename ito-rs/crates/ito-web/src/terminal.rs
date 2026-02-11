//! WebSocket-to-PTY bridge for in-browser terminal emulation.
//!
//! A client connects via `GET /ws/terminal`, which upgrades to a WebSocket.
//! The server spawns a PTY running the user's `$SHELL` (falling back to
//! `/bin/bash`) rooted at the project directory. Input from the WebSocket is
//! written to the PTY master; output from the PTY master is streamed back as
//! binary WebSocket frames.
//!
//! The client may send a JSON `{"resize":{"cols":N,"rows":M}}` text frame to
//! resize the PTY on the fly.

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Shared state for the terminal WebSocket handler.
#[derive(Clone)]
pub struct TerminalState {
    /// Project root directory used as the PTY's initial working directory.
    pub root: PathBuf,
}

/// Accept a WebSocket upgrade and spawn a PTY-backed terminal session.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<TerminalState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<TerminalState>) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Create PTY
    let pty_system = NativePtySystem::default();
    let pair = match pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }) {
        Ok(pair) => pair,
        Err(e) => {
            let _ = ws_sender
                .send(Message::Text(format!("Failed to open PTY: {e}").into()))
                .await;
            return;
        }
    };

    // Spawn shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let mut cmd = CommandBuilder::new(&shell);
    cmd.cwd(&state.root);

    // Set a nice prompt
    cmd.env("PS1", "\\[\\033[1;34m\\]\\w\\[\\033[0m\\] $ ");

    let mut child = match pair.slave.spawn_command(cmd) {
        Ok(child) => child,
        Err(e) => {
            let _ = ws_sender
                .send(Message::Text(format!("Failed to spawn shell: {e}").into()))
                .await;
            return;
        }
    };

    // Get reader/writer for PTY master
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut writer = pair.master.take_writer().unwrap();

    // Channel for PTY output -> WebSocket
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);

    // Task: Read from PTY, send to channel
    let read_handle = std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if tx.blocking_send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Task: Send PTY output to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            if ws_sender.send(Message::Binary(data.into())).await.is_err() {
                break;
            }
        }
    });

    // Receive from WebSocket, write to PTY
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                // Handle resize messages: { "resize": { "cols": N, "rows": M } }
                if text.starts_with("{")
                    && let Ok(json) = serde_json::from_str::<serde_json::Value>(&text)
                    && let Some(resize) = json.get("resize")
                {
                    let cols = resize.get("cols").and_then(|v| v.as_u64()).unwrap_or(80) as u16;
                    let rows = resize.get("rows").and_then(|v| v.as_u64()).unwrap_or(24) as u16;
                    let _ = pair.master.resize(PtySize {
                        rows,
                        cols,
                        pixel_width: 0,
                        pixel_height: 0,
                    });
                    continue;
                }
                // Regular input
                if writer.write_all(text.as_bytes()).is_err() {
                    break;
                }
            }
            Message::Binary(data) => {
                if writer.write_all(&data).is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup
    let _ = child.kill();
    drop(writer);
    let _ = read_handle.join();
    send_task.abort();
}
