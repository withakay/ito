//! HTML browser viewer backend.
//!
//! Converts a markdown document to standalone HTML via `pandoc` and opens it in
//! the system default browser (`open` on macOS, `xdg-open` on Linux).

use std::process::Command;

use crate::errors::{CoreError, CoreResult};

use super::ViewerBackend;
use super::util::command_on_path;

/// Render markdown as HTML in the system browser via `pandoc`.
pub struct HtmlViewer;

/// Return the platform-specific command for opening a file in the default application.
///
/// Returns `open` on macOS and `xdg-open` on Linux/other Unix systems.
/// Windows is not currently supported.
fn browser_opener() -> &'static str {
    if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    }
}

impl ViewerBackend for HtmlViewer {
    fn name(&self) -> &str {
        "html"
    }

    fn description(&self) -> &str {
        "Open the proposal as HTML in the system browser (requires pandoc)"
    }

    fn is_available(&self) -> bool {
        command_on_path("pandoc") && command_on_path(browser_opener())
    }

    fn availability_hint(&self) -> Option<String> {
        if !command_on_path("pandoc") {
            return Some(
                "pandoc is required for the HTML viewer. \
                 Install it from https://pandoc.org/installing.html"
                    .to_string(),
            );
        }
        let opener = browser_opener();
        if !command_on_path(opener) {
            return Some(format!(
                "'{opener}' is required to open the browser. \
                 Please install it or open the HTML file manually."
            ));
        }
        None
    }

    fn open(&self, content: &str) -> CoreResult<()> {
        if !command_on_path("pandoc") {
            return Err(CoreError::not_found(
                "pandoc is required for the HTML viewer. \
                 Install it from https://pandoc.org/installing.html",
            ));
        }

        let opener = browser_opener();
        if !command_on_path(opener) {
            return Err(CoreError::not_found(format!(
                "'{opener}' is required to open the browser. \
                 Please install it or open the HTML file manually.",
            )));
        }

        // Write the markdown content to a temporary file.
        let mut md_file = tempfile::Builder::new()
            .prefix("ito-viewer-")
            .suffix(".md")
            .tempfile()
            .map_err(|e| CoreError::io("creating temporary markdown file", e))?;
        std::io::Write::write_all(&mut md_file, content.as_bytes())
            .map_err(|e| CoreError::io("writing temporary markdown file", e))?;

        // Build the output HTML path alongside the markdown tempfile.
        // This HTML file intentionally outlives this function: `open`/`xdg-open`
        // returns immediately while the browser reads the file asynchronously.
        // Cleanup is left to the OS temp directory reaper.
        let html_path = md_file.path().with_extension("html");

        // Convert markdown to standalone HTML via pandoc.
        let pandoc_output = Command::new("pandoc")
            .arg("--standalone")
            .arg("--from=markdown")
            .arg("--to=html5")
            .arg("-o")
            .arg(&html_path)
            .arg(md_file.path())
            .output()
            .map_err(|e| CoreError::io("spawning pandoc", e))?;

        if !pandoc_output.status.success() {
            return Err(CoreError::process(format!(
                "pandoc failed: {}",
                String::from_utf8_lossy(&pandoc_output.stderr).trim()
            )));
        }

        // Open the HTML file in the system browser.
        let open_output = Command::new(opener)
            .arg(&html_path)
            .output()
            .map_err(|e| CoreError::io(format!("spawning {opener}"), e))?;

        if !open_output.status.success() {
            return Err(CoreError::process(format!(
                "{opener} failed: {}",
                String::from_utf8_lossy(&open_output.stderr).trim()
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "html_tests.rs"]
mod html_tests;
