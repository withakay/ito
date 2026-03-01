//! Shared application state for the multi-tenant backend API.
//!
//! [`AppState`] owns the backend server configuration including the data
//! directory, allowlist, and auth settings. Project-scoped repositories
//! are constructed per-request by resolving `{org}/{repo}` to a path
//! under the configured data directory.

use std::path::PathBuf;

use ito_config::types::{BackendAllowlistConfig, BackendAuthConfig};

/// Shared state passed to all API handlers via `axum::State`.
///
/// In multi-tenant mode, the data directory root replaces the single
/// project root. Each `{org}/{repo}` is resolved to a subdirectory
/// under `data_dir/projects/{org}/{repo}/.ito/`.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Root directory for backend-managed project storage.
    pub data_dir: PathBuf,
    /// Organization/repository allowlist policy.
    pub allowlist: BackendAllowlistConfig,
    /// Authentication configuration.
    pub auth: BackendAuthConfig,
}

impl AppState {
    /// Construct application state for a multi-tenant backend.
    pub fn new(
        data_dir: PathBuf,
        allowlist: BackendAllowlistConfig,
        auth: BackendAuthConfig,
    ) -> Self {
        Self {
            data_dir,
            allowlist,
            auth,
        }
    }

    /// Resolve the `.ito/` path for a project identified by `{org}/{repo}`.
    ///
    /// Returns `<data_dir>/projects/{org}/{repo}/.ito`.
    pub fn ito_path_for(&self, org: &str, repo: &str) -> PathBuf {
        self.data_dir
            .join("projects")
            .join(org)
            .join(repo)
            .join(".ito")
    }

    /// Ensure the project directory structure exists.
    ///
    /// Creates `<data_dir>/projects/{org}/{repo}/.ito/` if it does not exist.
    #[allow(dead_code)]
    pub fn ensure_project_dir(&self, org: &str, repo: &str) -> std::io::Result<()> {
        let ito_path = self.ito_path_for(org, repo);
        std::fs::create_dir_all(&ito_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ito_path_for_resolves_to_expected_path() {
        let state = AppState::new(
            PathBuf::from("/data"),
            BackendAllowlistConfig::default(),
            BackendAuthConfig::default(),
        );
        let path = state.ito_path_for("withakay", "ito");
        assert_eq!(path, PathBuf::from("/data/projects/withakay/ito/.ito"));
    }

    #[test]
    fn ensure_project_dir_creates_directories() {
        let tmp = tempfile::tempdir().unwrap();
        let state = AppState::new(
            tmp.path().to_path_buf(),
            BackendAllowlistConfig::default(),
            BackendAuthConfig::default(),
        );
        state.ensure_project_dir("acme", "repo1").unwrap();
        assert!(state.ito_path_for("acme", "repo1").is_dir());
    }
}
