use crate::errors::CoreResult;

use super::ViewerBackend;
use super::bat::run_with_stdin;

/// Render markdown via `glow`.
pub struct GlowViewer;

impl ViewerBackend for GlowViewer {
    fn name(&self) -> &str {
        "glow"
    }

    fn description(&self) -> &str {
        "Render the proposal in the terminal with glow"
    }

    fn is_available(&self) -> bool {
        super::bat::command_on_path("glow")
    }

    fn open(&self, content: &str) -> CoreResult<()> {
        run_with_stdin("glow", &["-"], content)
    }
}
