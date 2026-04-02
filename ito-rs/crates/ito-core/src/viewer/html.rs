//! HTML browser viewer backend.
//!
//! Converts a markdown document to standalone HTML via `pandoc` and opens it in
//! the system default browser (`open` on macOS, `xdg-open` on Linux).

use std::process::Command;

use crate::errors::{CoreError, CoreResult};

use super::ViewerBackend;
use super::bat::command_on_path;

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
        let pandoc_status = Command::new("pandoc")
            .arg("--standalone")
            .arg("--from=markdown")
            .arg("--to=html5")
            .arg("-o")
            .arg(&html_path)
            .arg(md_file.path())
            .status()
            .map_err(|e| CoreError::io("spawning pandoc", e))?;

        if !pandoc_status.success() {
            return Err(CoreError::process(format!(
                "pandoc exited with status {pandoc_status}"
            )));
        }

        // Open the HTML file in the system browser.
        let open_status = Command::new(opener)
            .arg(&html_path)
            .status()
            .map_err(|e| CoreError::io(format!("spawning {opener}"), e))?;

        if !open_status.success() {
            return Err(CoreError::process(format!(
                "{opener} exited with status {open_status}"
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_viewer_reports_expected_name() {
        assert_eq!(HtmlViewer.name(), "html");
    }

    #[test]
    fn html_viewer_reports_expected_description() {
        let desc = HtmlViewer.description();
        assert!(
            desc.contains("HTML"),
            "description should mention HTML: {desc}"
        );
        assert!(
            desc.contains("pandoc"),
            "description should mention pandoc: {desc}"
        );
    }

    #[test]
    fn html_viewer_availability_depends_on_pandoc() {
        // This test validates the code path rather than the environment.
        // If pandoc is not on PATH, is_available() returns false.
        let viewer = HtmlViewer;
        let pandoc_present = command_on_path("pandoc");
        let opener_present = command_on_path(browser_opener());
        assert_eq!(viewer.is_available(), pandoc_present && opener_present);
    }

    #[test]
    fn html_viewer_open_errors_when_pandoc_missing() {
        // Only run when pandoc is genuinely absent.
        if command_on_path("pandoc") {
            return;
        }
        let result = HtmlViewer.open("# Test");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("pandoc"), "error should mention pandoc: {err}");
        assert!(
            err.contains("https://pandoc.org"),
            "error should include install hint: {err}"
        );
    }
}
