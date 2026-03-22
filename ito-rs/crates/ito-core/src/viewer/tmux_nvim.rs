use std::process::Command;

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

        let mut temp_file = tempfile::Builder::new()
            .prefix("ito-viewer-")
            .suffix(".md")
            .tempfile()
            .map_err(|e| CoreError::io("creating temporary viewer file", e))?;
        std::io::Write::write_all(&mut temp_file, content.as_bytes())
            .map_err(|e| CoreError::io("writing temporary viewer file", e))?;

        // temp_file is deleted when it goes out of scope; `status()` waits for
        // nvim to exit, so the file remains valid for the entire nvim session.
        let status = Command::new("tmux")
            .arg("display-popup")
            .arg("-E")
            .arg("nvim")
            .arg("-R")
            .arg(temp_file.path())
            .status()
            .map_err(|e| CoreError::io("spawning tmux display-popup", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(CoreError::process(format!(
                "tmux display-popup exited with status {status}"
            )))
        }
    }
}
