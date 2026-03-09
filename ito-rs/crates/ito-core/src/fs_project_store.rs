//! Filesystem-backed [`BackendProjectStore`] implementation.
//!
//! Resolves `{org}/{repo}` to a project `.ito/` path under a configurable
//! data directory and constructs domain repositories from that path.
//!
//! Directory layout: `<data_dir>/projects/{org}/{repo}/.ito/`

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use ito_domain::backend::{ArchiveResult, ArtifactBundle, BackendError, BackendProjectStore, PushResult};
use ito_domain::changes::ChangeRepository;
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

use crate::repository_runtime::{
    boxed_fs_change_repository, boxed_fs_module_repository, boxed_fs_task_mutation_port,
    boxed_fs_task_repository, boxed_fs_spec_repository,
};

fn filesystem_revision(ito_path: &std::path::Path, change_id: &str) -> String {
    let change_dir = ito_common::paths::changes_dir(ito_path).join(change_id);
    if let Ok(Some(revision)) = crate::backend_sync::read_revision_file(&change_dir)
        && !revision.trim().is_empty()
    {
        return revision;
    }

    let mut latest: Option<DateTime<Utc>> = None;
    for relative in ["proposal.md", "design.md", "tasks.md"] {
        let path = change_dir.join(relative);
        if let Ok(metadata) = std::fs::metadata(&path)
            && let Ok(modified) = metadata.modified()
        {
            let timestamp = DateTime::<Utc>::from(modified);
            latest = Some(latest.map_or(timestamp, |current| current.max(timestamp)));
        }
    }
    latest.unwrap_or_else(Utc::now).to_rfc3339()
}

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

    fn task_mutation_service(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ito_domain::tasks::TaskMutationService + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(boxed_fs_task_mutation_port(ito_path))
    }

    fn spec_repository(
        &self,
        org: &str,
        repo: &str,
    ) -> DomainResult<Box<dyn ito_domain::specs::SpecRepository + Send>> {
        let ito_path = self.ito_path_for(org, repo)?;
        Ok(boxed_fs_spec_repository(ito_path))
    }

    fn pull_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ArtifactBundle, BackendError> {
        let ito_path = self
            .ito_path_for(org, repo)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        let mut bundle = crate::backend_sync::read_local_bundle(&ito_path, change_id)
            .map_err(|err| BackendError::NotFound(err.to_string()))?;
        bundle.revision = filesystem_revision(&ito_path, change_id);
        Ok(bundle)
    }

    fn push_artifact_bundle(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> Result<PushResult, BackendError> {
        let ito_path = self
            .ito_path_for(org, repo)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        let current_revision = filesystem_revision(&ito_path, change_id);
        if !bundle.revision.trim().is_empty() && bundle.revision != current_revision {
            return Err(BackendError::RevisionConflict(ito_domain::backend::RevisionConflict {
                change_id: change_id.to_string(),
                local_revision: bundle.revision.clone(),
                server_revision: current_revision,
            }));
        }

        let new_revision = Utc::now().to_rfc3339();
        let mut next = bundle.clone();
        next.change_id = change_id.to_string();
        next.revision = new_revision.clone();
        crate::backend_sync::write_bundle_to_local(&ito_path, change_id, &next)
            .map_err(|err| BackendError::Other(err.to_string()))?;

        Ok(PushResult {
            change_id: change_id.to_string(),
            new_revision,
        })
    }

    fn archive_change(
        &self,
        org: &str,
        repo: &str,
        change_id: &str,
    ) -> Result<ArchiveResult, BackendError> {
        let ito_path = self
            .ito_path_for(org, repo)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        let module_repo = self
            .module_repository(org, repo)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        let spec_names = crate::archive::discover_change_specs(&ito_path, change_id)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        crate::archive::copy_specs_to_main(&ito_path, change_id, &spec_names)
            .map_err(|err| BackendError::Other(err.to_string()))?;
        let archive_name = crate::archive::generate_archive_name(change_id);
        crate::archive::move_to_archive(module_repo.as_ref(), &ito_path, change_id, &archive_name)
            .map_err(|err| BackendError::Other(err.to_string()))?;

        Ok(ArchiveResult {
            change_id: change_id.to_string(),
            archived_at: Utc::now().to_rfc3339(),
        })
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
