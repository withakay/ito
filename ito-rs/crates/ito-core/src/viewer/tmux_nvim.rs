use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::{CoreError, CoreResult};

use super::ViewerBackend;
use super::bat::command_on_path;

/// Render markdown inside a tmux popup running Neovim in read-only mode.
pub struct TmuxNvimViewer;

impl ViewerBackend for TmuxNvimViewer {
    fn name(&self) -> &str {
        "tmux-nvim"
    }

    fn description(&self) -> &str {
        "Open the proposal in a tmux popup with Neovim"
    }

    fn is_available(&self) -> bool {
        std::env::var_os("TMUX").is_some() && command_on_path("tmux") && command_on_path("nvim")
    }

    fn open(&self, content: &str) -> CoreResult<()> {
        if std::env::var_os("TMUX").is_none() {
            return Err(CoreError::validation(
                "tmux-nvim viewer requires an active tmux session",
            ));
        }
        if !command_on_path("nvim") {
            return Err(CoreError::not_found("nvim is not installed or not on PATH"));
        }
        if !command_on_path("tmux") {
            return Err(CoreError::not_found("tmux is not installed or not on PATH"));
        }

        let temp_file = temporary_viewer_path();
        std::fs::write(&temp_file, content)
            .map_err(|e| CoreError::io("writing temporary viewer file", e))?;

        let popup_command = format!("nvim -R {}", shell_escape(&temp_file.to_string_lossy()));
        let status = Command::new("tmux")
            .args(["display-popup", "-E", &popup_command])
            .status();
        let _ = std::fs::remove_file(&temp_file);
        let status = status.map_err(|e| CoreError::io("spawning tmux display-popup", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(CoreError::process(format!(
                "tmux display-popup exited with status {status}"
            )))
        }
    }
}

fn temporary_viewer_path() -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("ito-viewer-{nanos}.md"))
}

fn shell_escape(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
