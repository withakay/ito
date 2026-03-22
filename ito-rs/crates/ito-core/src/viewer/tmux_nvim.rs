use std::process::Command;

use crate::errors::{CoreError, CoreResult};
use uuid::Uuid;

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

        let status = Command::new("tmux")
            .arg("display-popup")
            .arg("-E")
            .arg("nvim")
            .arg("-R")
            .arg(&temp_file)
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
    std::env::temp_dir().join(format!("ito-viewer-{}.md", Uuid::new_v4()))
}

#[cfg(test)]
mod tests {
    use super::temporary_viewer_path;

    #[test]
    fn temporary_viewer_path_is_unique() {
        let first = temporary_viewer_path();
        let second = temporary_viewer_path();

        assert_ne!(first, second);
    }
}
