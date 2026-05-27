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

    fn push(&self, _change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
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

    let not_found = backend_error_to_core(BackendError::NotFound("change xyz".to_string()), "pull");
    assert!(matches!(not_found, CoreError::NotFound(_)));
}
