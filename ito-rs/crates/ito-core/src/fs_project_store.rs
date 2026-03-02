//! Filesystem-backed [`BackendProjectStore`] implementation.
//!
//! Resolves `{org}/{repo}` to a project `.ito/` path under a configurable
//! data directory and constructs domain repositories from that path.
//!
//! Directory layout: `<data_dir>/projects/{org}/{repo}/.ito/`

use std::path::PathBuf;

use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::{
    Change, ChangeRepository, ChangeSummary, ChangeTargetResolution, ResolveTargetOptions,
};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::{Module, ModuleRepository, ModuleSummary};
use ito_domain::tasks::{TaskRepository, TasksParseResult};

use crate::change_repository::FsChangeRepository;
use crate::module_repository::FsModuleRepository;
use crate::task_repository::FsTaskRepository;

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
        Ok(Box::new(OwnedFsChangeRepository::new(ito_path)))
    }

    fn module_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ModuleRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(Box::new(OwnedFsModuleRepository::new(ito_path)))
    }

    fn task_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn TaskRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(Box::new(OwnedFsTaskRepository::new(ito_path)))
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

// ── Owned-path repository wrappers ─────────────────────────────────
//
// The standard `Fs*Repository` types borrow their path (`&'a Path`).
// The `BackendProjectStore` trait returns `Box<dyn ... + Send>` which
// requires `'static`. These wrappers own the `PathBuf` and delegate
// to the borrowed-path implementations by creating them on the fly.

/// Change repository that owns its `.ito/` path.
struct OwnedFsChangeRepository {
    ito_path: PathBuf,
}

impl OwnedFsChangeRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsChangeRepository<'_> {
        FsChangeRepository::new(&self.ito_path)
    }
}

impl ChangeRepository for OwnedFsChangeRepository {
    fn resolve_target_with_options(
        &self,
        input: &str,
        options: ResolveTargetOptions,
    ) -> ChangeTargetResolution {
        self.inner().resolve_target_with_options(input, options)
    }

    fn suggest_targets(&self, input: &str, max: usize) -> Vec<String> {
        self.inner().suggest_targets(input, max)
    }

    fn exists(&self, id: &str) -> bool {
        self.inner().exists(id)
    }

    fn get(&self, id: &str) -> DomainResult<Change> {
        self.inner().get(id)
    }

    fn list(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.inner().list()
    }

    fn list_by_module(&self, module_id: &str) -> DomainResult<Vec<ChangeSummary>> {
        self.inner().list_by_module(module_id)
    }

    fn list_incomplete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.inner().list_incomplete()
    }

    fn list_complete(&self) -> DomainResult<Vec<ChangeSummary>> {
        self.inner().list_complete()
    }

    fn get_summary(&self, id: &str) -> DomainResult<ChangeSummary> {
        self.inner().get_summary(id)
    }
}

/// Module repository that owns its `.ito/` path.
struct OwnedFsModuleRepository {
    ito_path: PathBuf,
}

impl OwnedFsModuleRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsModuleRepository<'_> {
        FsModuleRepository::new(&self.ito_path)
    }
}

impl ModuleRepository for OwnedFsModuleRepository {
    fn exists(&self, id: &str) -> bool {
        self.inner().exists(id)
    }

    fn get(&self, id_or_name: &str) -> DomainResult<Module> {
        self.inner().get(id_or_name)
    }

    fn list(&self) -> DomainResult<Vec<ModuleSummary>> {
        self.inner().list()
    }
}

/// Task repository that owns its `.ito/` path.
struct OwnedFsTaskRepository {
    ito_path: PathBuf,
}

impl OwnedFsTaskRepository {
    fn new(ito_path: PathBuf) -> Self {
        Self { ito_path }
    }

    fn inner(&self) -> FsTaskRepository<'_> {
        FsTaskRepository::new(&self.ito_path)
    }
}

impl TaskRepository for OwnedFsTaskRepository {
    fn load_tasks(&self, change_id: &str) -> DomainResult<TasksParseResult> {
        self.inner().load_tasks(change_id)
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
