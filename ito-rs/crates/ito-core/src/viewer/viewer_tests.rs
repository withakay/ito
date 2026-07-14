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
    let registry = ViewerRegistry::new(vec![Box::new(DummyViewer), Box::new(UnavailableViewer)]);

    let available = registry.available_viewers();
    assert_eq!(available.len(), 1);
    assert_eq!(available[0].name(), "dummy");
    assert!(registry.find_by_name("dummy").is_some());
    assert!(registry.find_by_name("missing").is_none());
}

#[test]
fn default_registry_does_not_register_removed_tmux_viewer() {
    let registry = ViewerRegistry::for_proposals();
    assert!(registry.find_by_name("tmux-nvim").is_none());
}

#[test]
fn concrete_viewers_report_expected_names() {
    assert_eq!(BatViewer.name(), "bat");
    assert_eq!(GlowViewer.name(), "glow");
    assert_eq!(HtmlViewer.name(), "html");
}

#[test]
fn default_registry_includes_html_viewer() {
    let registry = ViewerRegistry::for_proposals();
    assert!(
        registry.find_by_name("html").is_some(),
        "html viewer should be registered in the default proposal registry"
    );
}

#[cfg(unix)]
#[test]
fn run_with_stdin_closes_pipe_after_write() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let result = crate::viewer::util::run_with_stdin("sh", &["-c", "cat >/dev/null"], "hi");
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
