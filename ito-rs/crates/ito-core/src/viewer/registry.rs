use super::{BatViewer, GlowViewer, HtmlViewer, ViewerBackend};

/// Registry of known proposal viewer backends.
pub struct ViewerRegistry {
    viewers: Vec<Box<dyn ViewerBackend>>,
}

impl ViewerRegistry {
    /// Create a registry from a fixed set of backends.
    pub fn new(viewers: Vec<Box<dyn ViewerBackend>>) -> Self {
        Self { viewers }
    }

    /// Create the default proposal viewer registry.
    pub fn for_proposals() -> Self {
        Self::new(vec![
            Box::new(BatViewer),
            Box::new(GlowViewer),
            Box::new(HtmlViewer),
        ])
    }

    /// Return viewers that are currently runnable.
    pub fn available_viewers(&self) -> Vec<&dyn ViewerBackend> {
        let mut available = Vec::new();
        for viewer in &self.viewers {
            let viewer = viewer.as_ref();
            if viewer.is_available() {
                available.push(viewer);
            }
        }
        available
    }

    /// Find a registered viewer by its stable name.
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
