//! Filesystem-backed [`BackendProjectStore`] implementation.
//!
//! Resolves `{org}/{repo}` to a project `.ito/` path under a configurable
//! data directory and constructs domain repositories from that path.
//!
//! Directory layout: `<data_dir>/projects/{org}/{repo}/.ito/`

use std::path::PathBuf;

use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::ChangeRepository;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

use crate::repository_runtime::{
    boxed_fs_change_repository, boxed_fs_module_repository, boxed_fs_task_repository,
};

/// Filesystem-backed project store rooted at a configurable data directory.
///
/// Projects are stored under `<data_dir>/projects/{org}/{repo}/.ito/`.
/// Repositories are constructed per-request using the resolved `.ito/` path.
#[derive(Debug, Clone)]
pub struct FsBackendProjectStore {
    data_dir: PathBuf,
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

impl FsBackendProjectStore {
    /// Create a new filesystem project store rooted at the given data directory.
    pub fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Compute the `.ito/` path for a project.
    ///
    /// Returns an error if `org` or `repo` contain path traversal characters.
    pub fn ito_path_for(&self, org: &str, repo: &str) -> DomainResult<PathBuf> {
        if !is_safe_path_segment(org) || !is_safe_path_segment(repo) {
            return Err(DomainError::io(
                "invalid path segment in org/repo",
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("invalid path segment: org={org:?}, repo={repo:?}"),
                ),
            ));
        }
        Ok(self
            .data_dir
            .join("projects")
            .join(org)
            .join(repo)
            .join(".ito"))
    }
}

impl BackendProjectStore for FsBackendProjectStore {
    fn change_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ChangeRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(boxed_fs_change_repository(ito_path))
    }

    fn module_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ModuleRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(boxed_fs_module_repository(ito_path))
    }

    fn task_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn TaskRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(boxed_fs_task_repository(ito_path))
    }

    fn ensure_project(&self, org: &str, repo: &str) -> DomainResult<()> {
        let ito_path = self.ito_path_for(org, repo)?;
        std::fs::create_dir_all(&ito_path)
            .map_err(|e| DomainError::io("creating project directory", e))
    }

    fn project_exists(&self, org: &str, repo: &str) -> bool {
        self.ito_path_for(org, repo)
            .map(|p| p.is_dir())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ito_path_resolves_correctly() {
        let store = FsBackendProjectStore::new("/data");
        let path = store.ito_path_for("withakay", "ito").unwrap();
        assert_eq!(path, PathBuf::from("/data/projects/withakay/ito/.ito"));
    }

    #[test]
    fn ito_path_rejects_path_traversal() {
        let store = FsBackendProjectStore::new("/data");
        assert!(store.ito_path_for("..", "ito").is_err());
        assert!(store.ito_path_for("org", "..").is_err());
        assert!(store.ito_path_for(".", "repo").is_err());
        assert!(store.ito_path_for("org/evil", "repo").is_err());
        assert!(store.ito_path_for("org", "repo\\evil").is_err());
        assert!(store.ito_path_for("", "repo").is_err());
    }

    #[test]
    fn project_exists_returns_false_for_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FsBackendProjectStore::new(tmp.path());
        assert!(!store.project_exists("noorg", "norepo"));
    }

    #[test]
    fn ensure_project_creates_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FsBackendProjectStore::new(tmp.path());
        store.ensure_project("acme", "widgets").unwrap();
        assert!(store.project_exists("acme", "widgets"));
        assert!(store.ito_path_for("acme", "widgets").unwrap().is_dir());
    }

    #[test]
    fn change_repository_returns_box_trait() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FsBackendProjectStore::new(tmp.path());
        store.ensure_project("org", "repo").unwrap();
        let repo = store.change_repository("org", "repo").unwrap();
        // Should return an empty list for a fresh project
        let changes = repo.list().unwrap();
        assert!(changes.is_empty());
    }

    #[test]
    fn module_repository_returns_box_trait() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FsBackendProjectStore::new(tmp.path());
        store.ensure_project("org", "repo").unwrap();
        let repo = store.module_repository("org", "repo").unwrap();
        let modules = repo.list().unwrap();
        assert!(modules.is_empty());
    }

    #[test]
    fn task_repository_returns_box_trait() {
        let tmp = tempfile::tempdir().unwrap();
        let store = FsBackendProjectStore::new(tmp.path());
        store.ensure_project("org", "repo").unwrap();
        let repo = store.task_repository("org", "repo").unwrap();
        // Loading tasks for a non-existent change should return empty
        let result = repo.load_tasks("nonexistent-change").unwrap();
        assert_eq!(result.progress.total, 0);
    }

    #[test]
    fn store_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FsBackendProjectStore>();
    }
}
