//! Shared application state for the multi-tenant backend API.
//!
//! [`AppState`] owns the backend server configuration including the project
//! store, allowlist, and auth settings. Project-scoped repositories are
//! obtained from the project store, which abstracts over filesystem and
//! SQLite backends.

use std::path::PathBuf;
use std::sync::Arc;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig};
use ito_core::BackendProjectStore;

/// Shared state passed to all API handlers via `axum::State`.
///
/// In multi-tenant mode, the project store resolves `{org}/{repo}` to
/// domain repository instances. The store implementation is selected at
/// server startup based on configuration.
#[derive(Clone)]
pub struct AppState {
    /// Root directory for backend-managed project storage.
    ///
    /// Retained for health/ready checks and event ingest (audit log writes).
    pub data_dir: PathBuf,
    /// Swappable project store (filesystem or SQLite).
    pub store: Arc<dyn BackendProjectStore>,
    /// Organization/repository allowlist policy.
    pub allowlist: BackendAllowlistConfig,
    /// Authentication configuration.
    pub auth: BackendAuthConfig,
}

/// Check that a path segment is safe for use in filesystem paths.
///
/// Rejects empty strings, `.`, `..`, and values containing `/` or `\`.
fn is_safe_path_segment(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
}

impl AppState {
    /// Construct application state for a multi-tenant backend.
    pub fn new(
        data_dir: PathBuf,
        store: Arc<dyn BackendProjectStore>,
        allowlist: BackendAllowlistConfig,
        auth: BackendAuthConfig,
    ) -> Self {
        Self {
            data_dir,
            store,
            allowlist,
            auth,
        }
    }

    /// Resolve the `.ito/` path for a project identified by `{org}/{repo}`.
    ///
    /// Returns `<data_dir>/projects/{org}/{repo}/.ito`.
    ///
    /// This path is used for operations that always target the filesystem
    /// (e.g., audit log writes for event ingest).
    ///
    /// Returns an error if `org` or `repo` contain path traversal characters.
    pub fn ito_path_for(&self, org: &str, repo: &str) -> std::io::Result<PathBuf> {
        if !is_safe_path_segment(org) || !is_safe_path_segment(repo) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid path segment: org={org:?}, repo={repo:?}"),
            ));
        }
        Ok(self
            .data_dir
            .join("projects")
            .join(org)
            .join(repo)
            .join(".ito"))
    }

    /// Ensure the project directory structure exists on the filesystem.
    ///
    /// Creates `<data_dir>/projects/{org}/{repo}/.ito/` if it does not exist.
    /// Used for audit log writes which always go to the filesystem.
    #[allow(dead_code)]
    pub fn ensure_project_dir(&self, org: &str, repo: &str) -> std::io::Result<()> {
        let ito_path = self.ito_path_for(org, repo)?;
        std::fs::create_dir_all(&ito_path)
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("data_dir", &self.data_dir)
            .field("store", &"<dyn BackendProjectStore>")
            .field("allowlist", &self.allowlist)
            .field("auth", &"<redacted>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_core::fs_project_store::FsBackendProjectStore;

    #[test]
    fn ito_path_for_resolves_to_expected_path() {
        let store = Arc::new(FsBackendProjectStore::new("/data"));
        let state = AppState::new(
            PathBuf::from("/data"),
            store,
            BackendAllowlistConfig::default(),
            BackendAuthConfig::default(),
        );
        let path = state.ito_path_for("withakay", "ito").unwrap();
        assert_eq!(path, PathBuf::from("/data/projects/withakay/ito/.ito"));
    }

    #[test]
    fn ito_path_for_rejects_path_traversal() {
        let store = Arc::new(FsBackendProjectStore::new("/data"));
        let state = AppState::new(
            PathBuf::from("/data"),
            store,
            BackendAllowlistConfig::default(),
            BackendAuthConfig::default(),
        );
        assert!(state.ito_path_for("..", "ito").is_err());
        assert!(state.ito_path_for("org", "..").is_err());
        assert!(state.ito_path_for(".", "repo").is_err());
        assert!(state.ito_path_for("org/evil", "repo").is_err());
        assert!(state.ito_path_for("org", "repo\\evil").is_err());
        assert!(state.ito_path_for("", "repo").is_err());
    }

    #[test]
    fn ensure_project_dir_creates_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let store = Arc::new(FsBackendProjectStore::new(tmp.path()));
        let state = AppState::new(
            tmp.path().to_path_buf(),
            store,
            BackendAllowlistConfig::default(),
            BackendAuthConfig::default(),
        );
        state.ensure_project_dir("acme", "repo1").unwrap();
        assert!(state.ito_path_for("acme", "repo1").unwrap().is_dir());
    }
}
