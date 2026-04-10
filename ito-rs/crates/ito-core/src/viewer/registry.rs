use super::{BatViewer, GlowViewer, HtmlViewer, TmuxNvimViewer, ViewerBackend};

/// Registry of known proposal viewer backends.
pub struct ViewerRegistry {
    viewers: Vec<Box<dyn ViewerBackend>>,
    tmux_enabled: bool,
}

impl ViewerRegistry {
    /// Create a registry from a fixed set of backends.
    pub fn new(viewers: Vec<Box<dyn ViewerBackend>>) -> Self {
        Self {
            viewers,
            tmux_enabled: true,
        }
    }

    /// Create a registry that respects the resolved tmux preference.
    pub fn with_tmux_enabled(viewers: Vec<Box<dyn ViewerBackend>>, tmux_enabled: bool) -> Self {
        Self {
            viewers,
            tmux_enabled,
        }
    }

    /// Create the default proposal viewer registry.
    pub fn for_proposals(tmux_enabled: bool) -> Self {
        Self::with_tmux_enabled(
            vec![
                Box::new(TmuxNvimViewer),
                Box::new(BatViewer),
                Box::new(GlowViewer),
                Box::new(HtmlViewer),
            ],
            tmux_enabled,
        )
    }

    /// Whether a registered viewer is enabled by config before availability checks.
    pub fn is_enabled(&self, name: &str) -> bool {
        self.tmux_enabled || name != "tmux-nvim"
    }

    /// Return viewers that are currently runnable.
    pub fn available_viewers(&self) -> Vec<&dyn ViewerBackend> {
        let mut available = Vec::new();
        for viewer in &self.viewers {
            let viewer = viewer.as_ref();
            if self.is_enabled(viewer.name()) && viewer.is_available() {
                available.push(viewer);
            }
        }
        available
    }

    /// Find a registered viewer by its stable name.
    ///
    /// This only checks registration; callers must still enforce config enablement.
    pub fn find_by_name(&self, name: &str) -> Option<&dyn ViewerBackend> {
        for viewer in &self.viewers {
            let viewer = viewer.as_ref();
            if viewer.name() == name {
                return Some(viewer);
            }
        }
        None
    }
}
