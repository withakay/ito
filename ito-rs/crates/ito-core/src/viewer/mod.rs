//! Proposal viewer support.

use crate::errors::CoreResult;

/// Artifact collection helpers for proposal viewing.
pub mod collector;

/// Bat-based terminal viewer backend.
pub mod bat;

/// Glow-based terminal viewer backend.
pub mod glow;

/// HTML browser viewer backend (pandoc + system browser).
pub mod html;

/// Viewer registry and lookup helpers.
pub mod registry;

/// Shared utilities for viewer backends.
pub(crate) mod util;

pub use bat::BatViewer;
pub use collector::collect_proposal_artifacts;
pub use glow::GlowViewer;
pub use html::HtmlViewer;
pub use registry::ViewerRegistry;

/// A pluggable backend that can render collected proposal artifacts.
pub trait ViewerBackend {
    /// Stable CLI/backend identifier.
    fn name(&self) -> &str;

    /// Human-readable summary shown in prompts and help.
    fn description(&self) -> &str;

    /// Whether the viewer can run in the current environment.
    fn is_available(&self) -> bool;

    /// Detailed hint shown when the viewer is unavailable.
    ///
    /// Returns `None` if `is_available()` is true or no specific guidance exists.
    fn availability_hint(&self) -> Option<String> {
        None
    }

    /// Open or render the provided proposal content.
    fn open(&self, content: &str) -> CoreResult<()>;
}

#[cfg(test)]
mod viewer_tests;
