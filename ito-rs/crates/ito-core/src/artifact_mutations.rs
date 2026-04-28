//! Active change artifact mutation services.

use std::path::{Path, PathBuf};

use chrono::Utc;
use diffy::{Patch, apply};

use crate::backend_sync;
use crate::errors::CoreError;
use crate::repository_runtime::SqliteRuntime;
use ito_common::id::parse_change_id;
use ito_common::paths;
use ito_domain::backend::{ArtifactBundle, BackendError, BackendProjectStore, BackendSyncClient};
use ito_domain::changes::{
    ChangeArtifactKind, ChangeArtifactMutationError, ChangeArtifactMutationResult,
    ChangeArtifactMutationService, ChangeArtifactMutationServiceResult, ChangeArtifactRef,
};

use crate::change_repository::FsChangeRepository;
use crate::sqlite_project_store::SqliteBackendProjectStore;

/// Filesystem-backed change artifact mutation service.
#[derive(Debug, Clone)]
pub struct FsChangeArtifactMutationService {
    ito_path: PathBuf,
}

impl FsChangeArtifactMutationService {
    /// Create a filesystem-backed change artifact mutation service.
    pub fn new(ito_path: impl Into<PathBuf>) -> Self {
        Self {
            ito_path: ito_path.into(),
        }
    }
}

impl ChangeArtifactMutationService for FsChangeArtifactMutationService {
    fn load_artifact(
        &self,
        target: &ChangeArtifactRef,
    ) -> ChangeArtifactMutationServiceResult<Option<String>> {
        let path = artifact_path(&self.ito_path, target)?;
        if !path.is_file() {
            return Ok(None);
        }
        let contents = ito_common::io::read_to_string_std(&path)
            .map_err(|err| ChangeArtifactMutationError::io("reading change artifact", err))?;
        Ok(Some(contents))
    }

    fn write_artifact(
        &self,
        target: &ChangeArtifactRef,
        content: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult> {
        ensure_change_exists(&self.ito_path, &target.change_id)?;
        let path = artifact_path(&self.ito_path, target)?;
        let existed = path.is_file();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                ChangeArtifactMutationError::io("creating parent directories for artifact", err)
            })?;
        }
        ito_common::io::write_std(&path, content)
            .map_err(|err| ChangeArtifactMutationError::io("writing change artifact", err))?;
        Ok(ChangeArtifactMutationResult {
            target: target.clone(),
            existed,
            revision: None,
        })
    }

    fn patch_artifact(
        &self,
        target: &ChangeArtifactRef,
        patch: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult> {
        ensure_change_exists(&self.ito_path, &target.change_id)?;
        let path = artifact_path(&self.ito_path, target)?;
        let Some(current) = self.load_artifact(target)? else {
            return Err(ChangeArtifactMutationError::not_found(format!(
                "Artifact '{}' not found for patching",
                target.label()
            )));
        };
        let updated = apply_unified_patch(&current, patch, target)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| {
                ChangeArtifactMutationError::io("creating parent directories for artifact", err)
            })?;
        }
        ito_common::io::write_std(&path, &updated)
            .map_err(|err| ChangeArtifactMutationError::io("writing patched artifact", err))?;
        Ok(ChangeArtifactMutationResult {
            target: target.clone(),
            existed: true,
            revision: None,
        })
    }
}

/// Generic bundle-backed change artifact mutation service.
#[derive(Debug, Clone)]
pub struct BundleBackedChangeArtifactMutationService<C> {
    client: C,
}

impl<C> BundleBackedChangeArtifactMutationService<C> {
    /// Create a bundle-backed mutation service.
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<C> ChangeArtifactMutationService for BundleBackedChangeArtifactMutationService<C>
where
    C: ChangeArtifactBundleClient + Send + Sync,
{
    fn load_artifact(
        &self,
        target: &ChangeArtifactRef,
    ) -> ChangeArtifactMutationServiceResult<Option<String>> {
        let bundle = self.client.pull_bundle(&target.change_id)?;
        Ok(bundle_artifact_content(&bundle, &target.artifact))
    }

    fn write_artifact(
        &self,
        target: &ChangeArtifactRef,
        content: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult> {
        let mut bundle = self.client.pull_bundle(&target.change_id)?;
        let existed = bundle_artifact_content(&bundle, &target.artifact).is_some();
        set_bundle_artifact(&mut bundle, &target.artifact, content.to_string());
        let revision = self.client.push_bundle(&target.change_id, &bundle)?;
        Ok(ChangeArtifactMutationResult {
            target: target.clone(),
            existed,
            revision: Some(revision),
        })
    }

    fn patch_artifact(
        &self,
        target: &ChangeArtifactRef,
        patch: &str,
    ) -> ChangeArtifactMutationServiceResult<ChangeArtifactMutationResult> {
        let mut bundle = self.client.pull_bundle(&target.change_id)?;
        let Some(current) = bundle_artifact_content(&bundle, &target.artifact) else {
            return Err(ChangeArtifactMutationError::not_found(format!(
                "Artifact '{}' not found for patching",
                target.label()
            )));
        };
        let updated = apply_unified_patch(&current, patch, target)?;
        set_bundle_artifact(&mut bundle, &target.artifact, updated);
        let revision = self.client.push_bundle(&target.change_id, &bundle)?;
        Ok(ChangeArtifactMutationResult {
            target: target.clone(),
            existed: true,
            revision: Some(revision),
        })
    }
}

/// Client abstraction for bundle-backed artifact mutation services.
pub trait ChangeArtifactBundleClient: std::fmt::Debug + Clone {
    /// Pull the latest artifact bundle for a change.
    fn pull_bundle(&self, change_id: &str) -> ChangeArtifactMutationServiceResult<ArtifactBundle>;

    /// Push an updated artifact bundle and return the resulting revision.
    fn push_bundle(
        &self,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> ChangeArtifactMutationServiceResult<String>;
}

/// Local SQLite-backed bundle client.
#[derive(Debug, Clone)]
pub struct SqliteChangeArtifactBundleClient {
    db_path: PathBuf,
    org: String,
    repo: String,
}

impl SqliteChangeArtifactBundleClient {
    /// Create a SQLite-backed bundle client from resolved runtime settings.
    pub fn new(runtime: &SqliteRuntime) -> Self {
        Self {
            db_path: runtime.db_path.clone(),
            org: runtime.org.clone(),
            repo: runtime.repo.clone(),
        }
    }

    fn open_store(&self) -> ChangeArtifactMutationServiceResult<SqliteBackendProjectStore> {
        SqliteBackendProjectStore::open(&self.db_path).map_err(change_artifact_error_from_core)
    }
}

impl ChangeArtifactBundleClient for SqliteChangeArtifactBundleClient {
    fn pull_bundle(&self, change_id: &str) -> ChangeArtifactMutationServiceResult<ArtifactBundle> {
        let store = self.open_store()?;
        store
            .pull_artifact_bundle(&self.org, &self.repo, change_id)
            .map_err(change_artifact_error_from_backend)
    }

    fn push_bundle(
        &self,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> ChangeArtifactMutationServiceResult<String> {
        let store = self.open_store()?;
        let result = store
            .push_artifact_bundle(&self.org, &self.repo, change_id, bundle)
            .map_err(change_artifact_error_from_backend)?;
        Ok(result.new_revision)
    }
}

/// Filesystem-backed bundle client for active-change bundles.
#[derive(Debug, Clone)]
pub struct FsChangeArtifactBundleClient {
    ito_path: PathBuf,
}

impl FsChangeArtifactBundleClient {
    /// Create a filesystem-backed bundle client.
    pub fn new(ito_path: impl Into<PathBuf>) -> Self {
        Self {
            ito_path: ito_path.into(),
        }
    }
}

impl ChangeArtifactBundleClient for FsChangeArtifactBundleClient {
    fn pull_bundle(&self, change_id: &str) -> ChangeArtifactMutationServiceResult<ArtifactBundle> {
        backend_sync::read_local_bundle(&self.ito_path, change_id)
            .map_err(change_artifact_error_from_core)
    }

    fn push_bundle(
        &self,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> ChangeArtifactMutationServiceResult<String> {
        let revision = Utc::now().to_rfc3339();
        let mut next = bundle.clone();
        next.change_id = change_id.to_string();
        next.revision = revision.clone();
        backend_sync::write_bundle_to_local(&self.ito_path, change_id, &next)
            .map_err(change_artifact_error_from_core)?;
        Ok(revision)
    }
}

/// Remote/backend-backed bundle client.
#[derive(Debug, Clone)]
pub struct RemoteChangeArtifactBundleClient<S> {
    client: S,
}

impl<S> RemoteChangeArtifactBundleClient<S> {
    /// Create a remote bundle client from an existing sync client.
    pub fn new(client: S) -> Self {
        Self { client }
    }
}

impl<S> ChangeArtifactBundleClient for RemoteChangeArtifactBundleClient<S>
where
    S: BackendSyncClient + Clone + std::fmt::Debug,
{
    fn pull_bundle(&self, change_id: &str) -> ChangeArtifactMutationServiceResult<ArtifactBundle> {
        self.client
            .pull(change_id)
            .map_err(change_artifact_error_from_backend)
    }

    fn push_bundle(
        &self,
        change_id: &str,
        bundle: &ArtifactBundle,
    ) -> ChangeArtifactMutationServiceResult<String> {
        let result = self
            .client
            .push(change_id, bundle)
            .map_err(change_artifact_error_from_backend)?;
        Ok(result.new_revision)
    }
}

fn ensure_change_exists(
    ito_path: &Path,
    change_id: &str,
) -> ChangeArtifactMutationServiceResult<()> {
    if parse_change_id(change_id).is_err() {
        return Err(ChangeArtifactMutationError::validation(format!(
            "Invalid change id '{change_id}'"
        )));
    }
    if !FsChangeRepository::new(ito_path).exists(change_id) {
        return Err(ChangeArtifactMutationError::not_found(format!(
            "Change '{change_id}' not found"
        )));
    }
    Ok(())
}

fn artifact_path(
    ito_path: &Path,
    target: &ChangeArtifactRef,
) -> ChangeArtifactMutationServiceResult<PathBuf> {
    if parse_change_id(&target.change_id).is_err() {
        return Err(ChangeArtifactMutationError::validation(format!(
            "Invalid change id '{}'",
            target.change_id
        )));
    }

    let base = paths::change_dir(ito_path, &target.change_id);
    match &target.artifact {
        ChangeArtifactKind::Proposal => Ok(base.join("proposal.md")),
        ChangeArtifactKind::Design => Ok(base.join("design.md")),
        ChangeArtifactKind::Tasks => crate::tasks::tracking_file_path(ito_path, &target.change_id)
            .map_err(change_artifact_error_from_core),
        ChangeArtifactKind::SpecDelta { capability } => {
            validate_path_component(capability, "capability")?;
            Ok(paths::change_specs_dir(ito_path, &target.change_id)
                .join(capability)
                .join("spec.md"))
        }
    }
}

fn validate_path_component(value: &str, label: &str) -> ChangeArtifactMutationServiceResult<()> {
    if value.trim().is_empty() {
        return Err(ChangeArtifactMutationError::validation(format!(
            "{label} must not be empty"
        )));
    }
    if value.contains("..") || value.contains('/') || value.contains('\\') || value.contains('\0') {
        return Err(ChangeArtifactMutationError::validation(format!(
            "{label} contains unsafe path characters: {value:?}"
        )));
    }
    Ok(())
}

fn apply_unified_patch(
    current: &str,
    patch_text: &str,
    target: &ChangeArtifactRef,
) -> ChangeArtifactMutationServiceResult<String> {
    let patch = Patch::from_str(patch_text).map_err(|err| {
        ChangeArtifactMutationError::validation(format!(
            "Invalid patch for '{}': {err}",
            target.label()
        ))
    })?;
    apply(current, &patch).map_err(|err| {
        ChangeArtifactMutationError::validation(format!(
            "Patch did not apply cleanly for '{}': {err}",
            target.label()
        ))
    })
}

fn bundle_artifact_content(
    bundle: &ArtifactBundle,
    artifact: &ChangeArtifactKind,
) -> Option<String> {
    match artifact {
        ChangeArtifactKind::Proposal => bundle.proposal.clone(),
        ChangeArtifactKind::Design => bundle.design.clone(),
        ChangeArtifactKind::Tasks => bundle.tasks.clone(),
        ChangeArtifactKind::SpecDelta { capability } => bundle
            .specs
            .iter()
            .find(|(name, _)| name == capability)
            .map(|(_, content)| content.clone()),
    }
}

fn set_bundle_artifact(
    bundle: &mut ArtifactBundle,
    artifact: &ChangeArtifactKind,
    content: String,
) {
    match artifact {
        ChangeArtifactKind::Proposal => bundle.proposal = Some(content),
        ChangeArtifactKind::Design => bundle.design = Some(content),
        ChangeArtifactKind::Tasks => bundle.tasks = Some(content),
        ChangeArtifactKind::SpecDelta { capability } => {
            if let Some((_, existing)) =
                bundle.specs.iter_mut().find(|(name, _)| name == capability)
            {
                *existing = content;
            } else {
                bundle.specs.push((capability.clone(), content));
                bundle.specs.sort_by(|left, right| left.0.cmp(&right.0));
            }
        }
    }
}

fn change_artifact_error_from_core(err: CoreError) -> ChangeArtifactMutationError {
    match err {
        CoreError::Domain(domain) => ChangeArtifactMutationError::other(domain.to_string()),
        CoreError::Io { context, source } => ChangeArtifactMutationError::io(context, source),
        CoreError::Validation(message) => ChangeArtifactMutationError::validation(message),
        CoreError::Parse(message) => ChangeArtifactMutationError::validation(message),
        CoreError::Process(message) => ChangeArtifactMutationError::other(message),
        CoreError::Sqlite(message) => {
            ChangeArtifactMutationError::other(format!("sqlite error: {message}"))
        }
        CoreError::NotFound(message) => ChangeArtifactMutationError::not_found(message),
        CoreError::Serde { context, message } => {
            ChangeArtifactMutationError::other(format!("{context}: {message}"))
        }
    }
}

fn change_artifact_error_from_backend(err: BackendError) -> ChangeArtifactMutationError {
    match err {
        BackendError::LeaseConflict(conflict) => ChangeArtifactMutationError::validation(format!(
            "Lease conflict while mutating '{}': change is claimed by '{}'",
            conflict.change_id, conflict.holder
        )),
        BackendError::RevisionConflict(conflict) => {
            ChangeArtifactMutationError::validation(format!(
                "Revision conflict for '{}': local revision '{}' is stale (server has '{}'). Retry after re-reading the current artifact state.",
                conflict.change_id, conflict.local_revision, conflict.server_revision
            ))
        }
        BackendError::Unavailable(message) => {
            ChangeArtifactMutationError::other(format!("backend unavailable: {message}"))
        }
        BackendError::Unauthorized(message) => {
            ChangeArtifactMutationError::validation(format!("backend auth failed: {message}"))
        }
        BackendError::NotFound(message) => {
            ChangeArtifactMutationError::not_found(format!("backend resource not found: {message}"))
        }
        BackendError::Other(message) => ChangeArtifactMutationError::other(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    #[test]
    fn fs_service_writes_and_patches_proposal() {
        let tmp = tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");
        std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

        let service = FsChangeArtifactMutationService::new(&ito_path);
        let target = ChangeArtifactRef {
            change_id: "025-11_demo".to_string(),
            artifact: ChangeArtifactKind::Proposal,
        };

        let result = service
            .write_artifact(&target, "# Proposal\n")
            .expect("write proposal");
        assert!(!result.existed);
        assert_eq!(
            std::fs::read_to_string(
                ito_path
                    .join("changes")
                    .join("025-11_demo")
                    .join("proposal.md")
            )
            .expect("read proposal"),
            "# Proposal\n"
        );

        let patch = "--- proposal\n+++ proposal\n@@ -1 +1 @@\n-# Proposal\n+# Updated Proposal\n";
        let result = service
            .patch_artifact(&target, patch)
            .expect("patch proposal");
        assert!(result.existed);
        assert_eq!(
            std::fs::read_to_string(
                ito_path
                    .join("changes")
                    .join("025-11_demo")
                    .join("proposal.md")
            )
            .expect("read patched proposal"),
            "# Updated Proposal\n"
        );
    }

    #[test]
    fn fs_service_creates_spec_delta_directory_on_write() {
        let tmp = tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");
        std::fs::create_dir_all(ito_path.join("changes").join("025-11_demo")).expect("change dir");

        let service = FsChangeArtifactMutationService::new(&ito_path);
        let target = ChangeArtifactRef {
            change_id: "025-11_demo".to_string(),
            artifact: ChangeArtifactKind::SpecDelta {
                capability: "backend-agent-instructions".to_string(),
            },
        };

        service
            .write_artifact(&target, "## ADDED Requirements\n")
            .expect("write spec delta");

        let spec_path = ito_path
            .join("changes")
            .join("025-11_demo")
            .join("specs")
            .join("backend-agent-instructions")
            .join("spec.md");
        assert_eq!(
            std::fs::read_to_string(spec_path).expect("read spec delta"),
            "## ADDED Requirements\n"
        );
    }

    #[derive(Debug, Clone)]
    struct FakeBundleClient {
        bundle: Arc<Mutex<ArtifactBundle>>,
        revision: Arc<Mutex<u32>>,
    }

    impl ChangeArtifactBundleClient for FakeBundleClient {
        fn pull_bundle(
            &self,
            _change_id: &str,
        ) -> ChangeArtifactMutationServiceResult<ArtifactBundle> {
            Ok(self.bundle.lock().expect("bundle lock").clone())
        }

        fn push_bundle(
            &self,
            _change_id: &str,
            bundle: &ArtifactBundle,
        ) -> ChangeArtifactMutationServiceResult<String> {
            *self.bundle.lock().expect("bundle lock") = bundle.clone();
            let mut revision = self.revision.lock().expect("revision lock");
            *revision += 1;
            Ok(format!("rev-{}", *revision))
        }
    }

    #[test]
    fn bundle_service_patches_design_and_returns_revision() {
        let client = FakeBundleClient {
            bundle: Arc::new(Mutex::new(ArtifactBundle {
                change_id: "025-11_demo".to_string(),
                proposal: Some("# Proposal\n".to_string()),
                design: Some("# Design\n".to_string()),
                tasks: None,
                specs: vec![],
                revision: "rev-1".to_string(),
            })),
            revision: Arc::new(Mutex::new(1)),
        };

        let service = BundleBackedChangeArtifactMutationService::new(client.clone());
        let target = ChangeArtifactRef {
            change_id: "025-11_demo".to_string(),
            artifact: ChangeArtifactKind::Design,
        };
        let patch = "--- design\n+++ design\n@@ -1 +1 @@\n-# Design\n+# Updated Design\n";

        let result = service
            .patch_artifact(&target, patch)
            .expect("patch design");
        assert_eq!(result.revision.as_deref(), Some("rev-2"));
        assert_eq!(
            client.bundle.lock().expect("bundle lock").design.as_deref(),
            Some("# Updated Design\n")
        );
    }
}
