//! Proposal viewer support.

use crate::errors::CoreResult;

/// Artifact collection helpers for proposal viewing.
pub mod collector;

/// Bat-based terminal viewer backend.
pub mod bat;

/// Glow-based terminal viewer backend.
pub mod glow;

/// Viewer registry and lookup helpers.
pub mod registry;

/// Tmux + Neovim popup viewer backend.
pub mod tmux_nvim;

pub use bat::BatViewer;
pub use collector::collect_proposal_artifacts;
pub use glow::GlowViewer;
pub use registry::ViewerRegistry;
pub use tmux_nvim::TmuxNvimViewer;

/// A pluggable backend that can render collected proposal artifacts.
pub trait ViewerBackend {
    /// Stable CLI/backend identifier.
    fn name(&self) -> &str;

    /// Human-readable summary shown in prompts and help.
    fn description(&self) -> &str;

    /// Whether the viewer can run in the current environment.
    fn is_available(&self) -> bool;

    /// Open or render the provided proposal content.
    fn open(&self, content: &str) -> CoreResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::errors::CoreResult;
    #[cfg(unix)]
    use std::sync::mpsc;
    #[cfg(unix)]
    use std::time::Duration;

    struct DummyViewer;

    impl ViewerBackend for DummyViewer {
        fn name(&self) -> &str {
            "dummy"
        }

        fn description(&self) -> &str {
            "Dummy viewer for tests"
        }

        fn is_available(&self) -> bool {
            true
        }

        fn open(&self, _content: &str) -> CoreResult<()> {
            Ok(())
        }
    }

    #[test]
    fn viewer_backend_trait_exposes_required_methods() {
        let viewer = DummyViewer;
        assert_eq!(viewer.name(), "dummy");
        assert_eq!(viewer.description(), "Dummy viewer for tests");
        assert!(viewer.is_available());
        viewer.open("hello").unwrap();
    }

    #[test]
    fn viewer_registry_filters_and_finds_available_viewers() {
        let registry =
            ViewerRegistry::new(vec![Box::new(DummyViewer), Box::new(UnavailableViewer)]);

        let available = registry.available_viewers();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].name(), "dummy");
        assert!(registry.find_by_name("dummy").is_some());
        assert!(registry.find_by_name("missing").is_none());
    }

    #[test]
    fn viewer_registry_hides_tmux_when_disabled() {
        let registry = ViewerRegistry::with_tmux_enabled(
            vec![Box::new(TmuxNvimViewer), Box::new(DummyViewer)],
            false,
        );

        let available = registry.available_viewers();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].name(), "dummy");
        assert!(!registry.is_enabled("tmux-nvim"));
        assert!(registry.find_by_name("tmux-nvim").is_some());
    }

    #[test]
    fn concrete_viewers_report_expected_names() {
        assert_eq!(BatViewer.name(), "bat");
        assert_eq!(GlowViewer.name(), "glow");
        assert_eq!(TmuxNvimViewer.name(), "tmux-nvim");
    }

    #[cfg(unix)]
    #[test]
    fn run_with_stdin_closes_pipe_after_write() {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let result = crate::viewer::bat::run_with_stdin("sh", &["-c", "cat >/dev/null"], "hi");
            tx.send(result).unwrap();
        });

        let result = rx
            .recv_timeout(Duration::from_secs(2))
            .expect("run_with_stdin should finish after writing EOF");
        assert!(result.is_ok(), "{result:?}");
    }

    struct UnavailableViewer;

    impl ViewerBackend for UnavailableViewer {
        fn name(&self) -> &str {
            "unavailable"
        }

        fn description(&self) -> &str {
            "Unavailable viewer for tests"
        }

        fn is_available(&self) -> bool {
            false
        }

        fn open(&self, _content: &str) -> CoreResult<()> {
            Ok(())
        }
    }
}
