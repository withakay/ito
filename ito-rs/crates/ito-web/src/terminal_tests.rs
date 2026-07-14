use super::{TerminalState, handle_socket_with_shell, ws_handler};
use axum::{
    Router,
    extract::{State, ws::WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use futures::{SinkExt, StreamExt};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_tungstenite::{WebSocketStream, client_async, tungstenite::Message};

async fn failing_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<TerminalState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_socket_with_shell(socket, state, "/definitely/missing/ito-shell".to_string())
    })
}

async fn connect(
    root: PathBuf,
    fail_shell_spawn: bool,
) -> (WebSocketStream<tokio::net::TcpStream>, JoinHandle<()>) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = if fail_shell_spawn {
        Router::new().route("/ws/terminal", get(failing_ws_handler))
    } else {
        Router::new().route("/ws/terminal", get(ws_handler))
    }
    .with_state(Arc::new(TerminalState { root }));
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let (socket, response) = client_async(format!("ws://{address}/ws/terminal"), stream)
        .await
        .unwrap();
    assert_eq!(response.status(), 101);
    (socket, server)
}

async fn output_until(socket: &mut WebSocketStream<tokio::net::TcpStream>, marker: &str) -> String {
    tokio::time::timeout(Duration::from_secs(10), async {
        let mut output = Vec::new();
        while let Some(message) = socket.next().await {
            match message.unwrap() {
                Message::Binary(data) => output.extend_from_slice(&data),
                Message::Text(text) => output.extend_from_slice(text.as_bytes()),
                Message::Close(_) => break,
                _ => {}
            }
            if String::from_utf8_lossy(&output).contains(marker) {
                break;
            }
        }
        String::from_utf8_lossy(&output).into_owned()
    })
    .await
    .expect("terminal output timed out")
}

#[tokio::test]
async fn websocket_resizes_pty_and_forwards_binary_input_to_the_shell() {
    let (mut socket, server) = connect(std::env::temp_dir(), false).await;
    socket
        .send(Message::Text(r#"{"resize":{"cols":100,"rows":40}}"#.into()))
        .await
        .unwrap();
    socket
        .send(Message::Binary(
            b"printf '__ITO_TERMINAL_OK__\\n'; exit\n".to_vec().into(),
        ))
        .await
        .unwrap();

    let output = output_until(&mut socket, "__ITO_TERMINAL_OK__").await;
    assert!(
        output.contains("__ITO_TERMINAL_OK__"),
        "output was {output:?}"
    );
    let _ = socket.close(None).await;
    server.abort();
}

#[tokio::test]
async fn websocket_reports_shell_spawn_failures() {
    let (mut socket, server) = connect(std::env::temp_dir(), true).await;

    let output = output_until(&mut socket, "Failed to spawn shell").await;
    assert!(
        output.contains("Failed to spawn shell"),
        "output was {output:?}"
    );
    server.abort();
}
