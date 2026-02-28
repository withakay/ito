//! Artifact synchronization service for backend mode.
//!
//! Orchestrates pull (backend → local) and push (local → backend) flows for
//! change artifacts, including revision metadata tracking and timestamped
//! local backup snapshots.

use std::path::Path;

use crate::errors::{CoreError, CoreResult};
use chrono::Utc;
use ito_common::paths;
use ito_domain::backend::{ArtifactBundle, BackendError, BackendSyncClient, PushResult};

/// Metadata written alongside pulled artifacts to track the backend revision.
const REVISION_FILE: &str = ".backend-revision";

/// Directory under a change for spec delta files.
const SPECS_DIR: &str = "specs";

/// Validate that a string is safe to use as a path component.
///
/// Rejects strings containing path traversal sequences (`..`), path
/// separators (`/`, `\`), or null bytes. This prevents untrusted values
/// from the backend from escaping the intended directory.
fn validate_path_component(name: &str, label: &str) -> CoreResult<()> {
    if name.is_empty() {
        return Err(CoreError::Validation(format!("{label} must not be empty")));
    }
    if name.contains("..") || name.contains('/') || name.contains('\\') || name.contains('\0') {
        return Err(CoreError::Validation(format!(
            "{label} contains unsafe path characters: {name:?}"
        )));
    }
    Ok(())
}

// ── Pull ────────────────────────────────────────────────────────────

/// Pull artifacts from the backend for a change and write them locally.
///
/// Creates a timestamped backup snapshot under `backup_dir` before writing.
/// Returns the pulled artifact bundle.
pub fn pull_artifacts<S: BackendSyncClient + ?Sized>(
    sync_client: &S,
    ito_path: &Path,
    change_id: &str,
    backup_dir: &Path,
) -> CoreResult<ArtifactBundle> {
    validate_path_component(change_id, "change_id")?;

    let bundle = sync_client
        .pull(change_id)
        .map_err(|e| backend_error_to_core(e, "pull"))?;

    // Create backup snapshot before writing
    create_backup_snapshot(ito_path, change_id, backup_dir, "pull")?;

    // Write artifacts to the local change directory
    write_bundle_to_local(ito_path, change_id, &bundle)?;

    Ok(bundle)
}

/// Push local artifacts to the backend with revision conflict detection.
///
/// Creates a timestamped backup snapshot before attempting the push.
/// Returns the push result on success or a conflict error.
pub fn push_artifacts<S: BackendSyncClient + ?Sized>(
    sync_client: &S,
    ito_path: &Path,
    change_id: &str,
    backup_dir: &Path,
) -> CoreResult<PushResult> {
    validate_path_component(change_id, "change_id")?;

    // Create backup snapshot before push
    create_backup_snapshot(ito_path, change_id, backup_dir, "push")?;

    // Read local artifacts into a bundle
    let bundle = read_local_bundle(ito_path, change_id)?;

    // Push to backend
    let result = sync_client
        .push(change_id, &bundle)
        .map_err(|e| backend_error_to_core(e, "push"))?;

    // Update local revision metadata
    let change_dir = paths::changes_dir(ito_path).join(change_id);
    write_revision_file(&change_dir, &result.new_revision)?;

    Ok(result)
}

// ── Local I/O helpers ───────────────────────────────────────────────

/// Write a pulled artifact bundle to the local change directory.
fn write_bundle_to_local(
    ito_path: &Path,
    change_id: &str,
    bundle: &ArtifactBundle,
) -> CoreResult<()> {
    let change_dir = paths::changes_dir(ito_path).join(change_id);
    std::fs::create_dir_all(&change_dir)
        .map_err(|e| CoreError::io("creating change directory", e))?;

    if let Some(proposal) = &bundle.proposal {
        let path = change_dir.join("proposal.md");
        std::fs::write(&path, proposal).map_err(|e| CoreError::io("writing proposal.md", e))?;
    }

    if let Some(design) = &bundle.design {
        let path = change_dir.join("design.md");
        std::fs::write(&path, design).map_err(|e| CoreError::io("writing design.md", e))?;
    }

    if let Some(tasks) = &bundle.tasks {
        let path = change_dir.join("tasks.md");
        std::fs::write(&path, tasks).map_err(|e| CoreError::io("writing tasks.md", e))?;
    }

    // Write spec delta files
    let specs_dir = change_dir.join(SPECS_DIR);
    for (capability, content) in &bundle.specs {
        validate_path_component(capability, "capability")?;
        let cap_dir = specs_dir.join(capability);
        std::fs::create_dir_all(&cap_dir)
            .map_err(|e| CoreError::io("creating spec directory", e))?;
        std::fs::write(cap_dir.join("spec.md"), content)
            .map_err(|e| CoreError::io("writing spec delta", e))?;
    }

    // Store revision metadata
    write_revision_file(&change_dir, &bundle.revision)?;

    Ok(())
}

/// Read local change artifacts into an artifact bundle for pushing.
fn read_local_bundle(ito_path: &Path, change_id: &str) -> CoreResult<ArtifactBundle> {
    let change_dir = paths::changes_dir(ito_path).join(change_id);
    if !change_dir.is_dir() {
        return Err(CoreError::not_found(format!(
            "Change directory not found: {change_id}"
        )));
    }

    let proposal = read_optional_file(&change_dir.join("proposal.md"))?;
    let design = read_optional_file(&change_dir.join("design.md"))?;
    let tasks = read_optional_file(&change_dir.join("tasks.md"))?;

    let mut specs = Vec::new();
    let specs_dir = change_dir.join(SPECS_DIR);
    if specs_dir.is_dir() {
        let entries =
            std::fs::read_dir(&specs_dir).map_err(|e| CoreError::io("reading specs dir", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| CoreError::io("reading spec entry", e))?;
            let cap_dir = entry.path();
            if cap_dir.is_dir() {
                let spec_file = cap_dir.join("spec.md");
                if spec_file.is_file() {
                    let content = std::fs::read_to_string(&spec_file)
                        .map_err(|e| CoreError::io("reading spec file", e))?;
                    let cap_name = entry.file_name().to_string_lossy().to_string();
                    specs.push((cap_name, content));
                }
            }
        }
    }
    specs.sort_by(|a, b| a.0.cmp(&b.0));

    let revision = read_revision_file(&change_dir)?.unwrap_or_default();

    Ok(ArtifactBundle {
        change_id: change_id.to_string(),
        proposal,
        design,
        tasks,
        specs,
        revision,
    })
}

/// Read a file if it exists, returning `None` if absent.
fn read_optional_file(path: &Path) -> CoreResult<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let content =
        std::fs::read_to_string(path).map_err(|e| CoreError::io("reading artifact file", e))?;
    Ok(Some(content))
}

/// Write the backend revision to a metadata file in the change directory.
fn write_revision_file(change_dir: &Path, revision: &str) -> CoreResult<()> {
    let path = change_dir.join(REVISION_FILE);
    std::fs::write(&path, revision).map_err(|e| CoreError::io("writing revision file", e))
}

/// Read the backend revision from a metadata file in the change directory.
fn read_revision_file(change_dir: &Path) -> CoreResult<Option<String>> {
    let path = change_dir.join(REVISION_FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let content =
        std::fs::read_to_string(&path).map_err(|e| CoreError::io("reading revision file", e))?;
    Ok(Some(content.trim().to_string()))
}

// ── Backup ──────────────────────────────────────────────────────────

/// Create a timestamped backup snapshot of local change artifacts.
fn create_backup_snapshot(
    ito_path: &Path,
    change_id: &str,
    backup_dir: &Path,
    operation: &str,
) -> CoreResult<()> {
    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let snapshot_dir = backup_dir.join(format!("{change_id}_{operation}_{timestamp}"));
    std::fs::create_dir_all(&snapshot_dir)
        .map_err(|e| CoreError::io("creating backup directory", e))?;

    let change_dir = paths::changes_dir(ito_path).join(change_id);
    if !change_dir.is_dir() {
        return Ok(()); // Nothing to back up
    }

    // Copy key artifact files
    for name in ["proposal.md", "design.md", "tasks.md"] {
        let src = change_dir.join(name);
        if src.is_file() {
            let dst = snapshot_dir.join(name);
            std::fs::copy(&src, &dst).map_err(|e| CoreError::io("backing up artifact", e))?;
        }
    }

    // Copy spec files
    let specs_src = change_dir.join(SPECS_DIR);
    if specs_src.is_dir() {
        copy_dir_recursive(&specs_src, &snapshot_dir.join(SPECS_DIR))?;
    }

    Ok(())
}

/// Recursively copy a directory.
fn copy_dir_recursive(src: &Path, dst: &Path) -> CoreResult<()> {
    std::fs::create_dir_all(dst).map_err(|e| CoreError::io("creating backup subdir", e))?;
    let entries =
        std::fs::read_dir(src).map_err(|e| CoreError::io("reading backup source dir", e))?;
    for entry in entries {
        let entry = entry.map_err(|e| CoreError::io("reading dir entry", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .map_err(|e| CoreError::io("copying backup file", e))?;
        }
    }
    Ok(())
}

// ── Error mapping ───────────────────────────────────────────────────

/// Convert a backend-specific error into a `CoreError`.
fn backend_error_to_core(err: BackendError, operation: &str) -> CoreError {
    match err {
        BackendError::LeaseConflict(c) => CoreError::validation(format!(
            "Lease conflict during {operation}: change '{}' is claimed by '{}'",
            c.change_id, c.holder
        )),
        BackendError::RevisionConflict(c) => CoreError::validation(format!(
            "Revision conflict during {operation} for '{}': \
             local revision '{}' is stale (server has '{}'). \
             Run 'ito tasks sync pull {}' first, then retry.",
            c.change_id, c.local_revision, c.server_revision, c.change_id
        )),
        BackendError::Unavailable(msg) => {
            CoreError::process(format!("Backend unavailable during {operation}: {msg}"))
        }
        BackendError::Unauthorized(msg) => {
            CoreError::validation(format!("Backend auth failed during {operation}: {msg}"))
        }
        BackendError::NotFound(msg) => CoreError::not_found(format!(
            "Backend resource not found during {operation}: {msg}"
        )),
        BackendError::Other(msg) => {
            CoreError::process(format!("Backend error during {operation}: {msg}"))
        }
    }
}

/// Convert a `BackendError` to a `CoreError` (public API for CLI use).
pub fn map_backend_error(err: BackendError, operation: &str) -> CoreError {
    backend_error_to_core(err, operation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_domain::backend::{BackendError, RevisionConflict};
    use tempfile::TempDir;

    /// Fake sync client that returns preconfigured results.
    struct FakeSyncClient {
        pull_result: Result<ArtifactBundle, BackendError>,
        push_result: Result<PushResult, BackendError>,
    }

    impl FakeSyncClient {
        fn success_pull(bundle: ArtifactBundle) -> Self {
            Self {
                pull_result: Ok(bundle),
                push_result: Ok(PushResult {
                    change_id: String::new(),
                    new_revision: String::new(),
                }),
            }
        }

        fn success_push(new_revision: &str) -> Self {
            Self {
                pull_result: Err(BackendError::Other("not configured".to_string())),
                push_result: Ok(PushResult {
                    change_id: String::new(),
                    new_revision: new_revision.to_string(),
                }),
            }
        }

        fn conflict_push(local: &str, server: &str) -> Self {
            Self {
                pull_result: Err(BackendError::Other("not configured".to_string())),
                push_result: Err(BackendError::RevisionConflict(RevisionConflict {
                    change_id: "test".to_string(),
                    local_revision: local.to_string(),
                    server_revision: server.to_string(),
                })),
            }
        }
    }

    impl BackendSyncClient for FakeSyncClient {
        fn pull(&self, _change_id: &str) -> Result<ArtifactBundle, BackendError> {
            self.pull_result.clone()
        }

        fn push(
            &self,
            _change_id: &str,
            _bundle: &ArtifactBundle,
        ) -> Result<PushResult, BackendError> {
            self.push_result.clone()
        }
    }

    fn test_bundle(change_id: &str) -> ArtifactBundle {
        ArtifactBundle {
            change_id: change_id.to_string(),
            proposal: Some("# Proposal\nTest".to_string()),
            design: None,
            tasks: Some("- [ ] Task 1\n".to_string()),
            specs: vec![("auth".to_string(), "## ADDED Requirements\n".to_string())],
            revision: "rev-1".to_string(),
        }
    }

    #[test]
    fn pull_writes_artifacts_locally() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");
        std::fs::create_dir_all(&ito_path).unwrap();

        let bundle = test_bundle("test-change");
        let client = FakeSyncClient::success_pull(bundle);

        let result = pull_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap();
        assert_eq!(result.change_id, "test-change");
        assert_eq!(result.revision, "rev-1");

        // Verify files were written
        let change_dir = ito_path.join("changes").join("test-change");
        assert!(change_dir.join("proposal.md").is_file());
        assert!(change_dir.join("tasks.md").is_file());
        assert!(!change_dir.join("design.md").exists());
        assert!(change_dir.join("specs/auth/spec.md").is_file());
        assert!(change_dir.join(REVISION_FILE).is_file());

        // Verify revision content
        let rev = std::fs::read_to_string(change_dir.join(REVISION_FILE)).unwrap();
        assert_eq!(rev, "rev-1");
    }

    #[test]
    fn pull_creates_backup() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");

        // Create existing local artifacts to back up
        let change_dir = ito_path.join("changes").join("test-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "old proposal").unwrap();

        let bundle = test_bundle("test-change");
        let client = FakeSyncClient::success_pull(bundle);

        pull_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap();

        // Verify backup was created
        assert!(backup_dir.is_dir());
        let entries: Vec<_> = std::fs::read_dir(&backup_dir).unwrap().collect();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn push_sends_local_bundle() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");

        // Create local artifacts
        let change_dir = ito_path.join("changes").join("test-change");
        std::fs::create_dir_all(change_dir.join("specs/auth")).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Test Proposal").unwrap();
        std::fs::write(change_dir.join("tasks.md"), "- [ ] Task").unwrap();
        std::fs::write(change_dir.join("specs/auth/spec.md"), "## ADDED").unwrap();
        std::fs::write(change_dir.join(REVISION_FILE), "rev-1").unwrap();

        let client = FakeSyncClient::success_push("rev-2");

        let result = push_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap();
        assert_eq!(result.new_revision, "rev-2");

        // Verify revision was updated locally
        let rev = std::fs::read_to_string(change_dir.join(REVISION_FILE)).unwrap();
        assert_eq!(rev, "rev-2");
    }

    #[test]
    fn push_conflict_returns_actionable_error() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");

        // Create minimal local artifacts
        let change_dir = ito_path.join("changes").join("test-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Test").unwrap();

        let client = FakeSyncClient::conflict_push("rev-1", "rev-3");

        let err = push_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Revision conflict"), "msg: {msg}");
        assert!(msg.contains("rev-1"), "msg: {msg}");
        assert!(msg.contains("rev-3"), "msg: {msg}");
        assert!(msg.contains("ito tasks sync pull"), "msg: {msg}");
    }

    #[test]
    fn push_missing_change_dir_fails() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");
        std::fs::create_dir_all(&ito_path).unwrap();

        let client = FakeSyncClient::success_push("rev-2");

        let err = push_artifacts(&client, &ito_path, "nonexistent", &backup_dir).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("not found"), "msg: {msg}");
    }

    #[test]
    fn read_local_bundle_sorts_specs() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let change_dir = ito_path.join("changes").join("test-change");

        // Create specs in reverse order
        std::fs::create_dir_all(change_dir.join("specs/z-spec")).unwrap();
        std::fs::create_dir_all(change_dir.join("specs/a-spec")).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Proposal").unwrap();
        std::fs::write(change_dir.join("specs/z-spec/spec.md"), "z content").unwrap();
        std::fs::write(change_dir.join("specs/a-spec/spec.md"), "a content").unwrap();

        let bundle = read_local_bundle(&ito_path, "test-change").unwrap();
        assert_eq!(bundle.specs.len(), 2);
        assert_eq!(bundle.specs[0].0, "a-spec");
        assert_eq!(bundle.specs[1].0, "z-spec");
    }

    #[test]
    fn path_traversal_in_change_id_rejected() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");
        std::fs::create_dir_all(&ito_path).unwrap();

        let client = FakeSyncClient::success_push("rev-1");

        let err = push_artifacts(&client, &ito_path, "../escape", &backup_dir).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));

        let err = push_artifacts(&client, &ito_path, "foo/bar", &backup_dir).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));

        let err = push_artifacts(&client, &ito_path, "", &backup_dir).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));
    }

    #[test]
    fn path_traversal_in_capability_rejected() {
        let tmp = TempDir::new().unwrap();
        let ito_path = tmp.path().join(".ito");
        let backup_dir = tmp.path().join("backups");
        std::fs::create_dir_all(&ito_path).unwrap();

        let bundle = ArtifactBundle {
            change_id: "safe-change".to_string(),
            proposal: None,
            design: None,
            tasks: None,
            specs: vec![("../escape".to_string(), "content".to_string())],
            revision: "rev-1".to_string(),
        };
        let client = FakeSyncClient::success_pull(bundle);

        let err = pull_artifacts(&client, &ito_path, "safe-change", &backup_dir).unwrap_err();
        assert!(matches!(err, CoreError::Validation(_)));
    }

    #[test]
    fn backend_error_mapping_produces_correct_error_types() {
        let unavailable =
            backend_error_to_core(BackendError::Unavailable("timeout".to_string()), "pull");
        assert!(matches!(unavailable, CoreError::Process(_)));

        let auth = backend_error_to_core(
            BackendError::Unauthorized("invalid token".to_string()),
            "push",
        );
        assert!(matches!(auth, CoreError::Validation(_)));

        let not_found =
            backend_error_to_core(BackendError::NotFound("change xyz".to_string()), "pull");
        assert!(matches!(not_found, CoreError::NotFound(_)));
    }
}
