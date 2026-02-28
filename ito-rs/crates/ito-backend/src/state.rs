//! Shared application state for the backend API.
//!
//! [`AppState`] owns the project root and `.ito` path. Repositories are
//! constructed per-request because the filesystem-backed implementations
//! borrow from a path reference, making them cheap to create.

use std::path::PathBuf;

/// Shared state passed to all API handlers via `axum::State`.
///
/// Holds the immutable paths needed to construct domain repositories
/// on each request. Repositories borrow `&Path` and are lightweight,
/// so per-request construction is efficient.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Root directory of the project (parent of `.ito/`).
    pub project_root: PathBuf,
    /// Path to the `.ito/` directory.
    pub ito_path: PathBuf,
}

impl AppState {
    /// Construct application state from a project root directory.
    ///
    /// The `.ito/` path is derived as `project_root.join(".ito")`.
    pub fn new(project_root: PathBuf) -> Self {
        let ito_path = project_root.join(".ito");
        Self {
            project_root,
            ito_path,
        }
    }

    /// Construct application state with explicit paths.
    ///
    /// Use this when the `.ito` directory name is overridden via configuration.
    pub fn with_ito_path(project_root: PathBuf, ito_path: PathBuf) -> Self {
        Self {
            project_root,
            ito_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_derives_ito_path_from_project_root() {
        let state = AppState::new(PathBuf::from("/tmp/myproject"));
        assert_eq!(state.project_root, PathBuf::from("/tmp/myproject"));
        assert_eq!(state.ito_path, PathBuf::from("/tmp/myproject/.ito"));
    }

    #[test]
    fn with_ito_path_uses_explicit_paths() {
        let state = AppState::with_ito_path(
            PathBuf::from("/tmp/myproject"),
            PathBuf::from("/tmp/myproject/.custom_ito"),
        );
        assert_eq!(state.ito_path, PathBuf::from("/tmp/myproject/.custom_ito"));
    }
}
