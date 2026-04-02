use crate::errors::CoreResult;

use super::ViewerBackend;
use super::util::{command_on_path, run_with_stdin};

/// Render markdown via `bat` with paging.
pub struct BatViewer;

impl ViewerBackend for BatViewer {
    fn name(&self) -> &str {
        "bat"
    }

    fn description(&self) -> &str {
        "Render the proposal in the terminal with bat"
    }

    fn is_available(&self) -> bool {
        command_on_path("bat")
    }

    fn open(&self, content: &str) -> CoreResult<()> {
        run_with_stdin("bat", &["--language=markdown", "--paging=always"], content)
    }
}
