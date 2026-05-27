use super::*;
use ito_domain::backend::{ArchiveResult, BackendError, ClaimResult, LeaseConflict, ReleaseResult};

struct FakeLeaseClient {
    claim_result: Result<ClaimResult, BackendError>,
    release_result: Result<ReleaseResult, BackendError>,
    allocate_result: Result<AllocateResult, BackendError>,
}

impl FakeLeaseClient {
    fn success() -> Self {
        Self {
            claim_result: Ok(ClaimResult {
                change_id: "test".to_string(),
                holder: "me".to_string(),
                expires_at: None,
            }),
            release_result: Ok(ReleaseResult {
                change_id: "test".to_string(),
            }),
            allocate_result: Ok(AllocateResult {
                claim: Some(ClaimResult {
                    change_id: "test".to_string(),
                    holder: "me".to_string(),
                    expires_at: None,
                }),
            }),
        }
    }

    fn conflict() -> Self {
        Self {
            claim_result: Err(BackendError::LeaseConflict(LeaseConflict {
                change_id: "test".to_string(),
                holder: "other".to_string(),
                expires_at: None,
            })),
            release_result: Ok(ReleaseResult {
                change_id: "test".to_string(),
            }),
            allocate_result: Ok(AllocateResult { claim: None }),
        }
    }
}

impl BackendLeaseClient for FakeLeaseClient {
    fn claim(&self, _change_id: &str) -> Result<ClaimResult, BackendError> {
        self.claim_result.clone()
    }

    fn release(&self, _change_id: &str) -> Result<ReleaseResult, BackendError> {
        self.release_result.clone()
    }

    fn allocate(&self) -> Result<AllocateResult, BackendError> {
        self.allocate_result.clone()
    }
}

#[test]
fn claim_success() {
    let client = FakeLeaseClient::success();
    let result = claim_change(&client, "test").unwrap();
    assert_eq!(result.change_id, "test");
    assert_eq!(result.holder, "me");
}

#[test]
fn claim_conflict() {
    let client = FakeLeaseClient::conflict();
    let err = claim_change(&client, "test").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Lease conflict"), "msg: {msg}");
    assert!(msg.contains("other"), "msg: {msg}");
}

#[test]
fn release_success() {
    let client = FakeLeaseClient::success();
    let result = release_change(&client, "test").unwrap();
    assert_eq!(result.change_id, "test");
}

#[test]
fn allocate_with_work() {
    let client = FakeLeaseClient::success();
    let result = allocate_change(&client).unwrap();
    assert!(result.claim.is_some());
    assert_eq!(result.claim.unwrap().change_id, "test");
}

#[test]
fn allocate_no_work() {
    let client = FakeLeaseClient::conflict();
    let result = allocate_change(&client).unwrap();
    assert!(result.claim.is_none());
}

#[test]
fn is_backend_unavailable_detects_process_error() {
    let err = CoreError::process("Backend unavailable during pull: timeout");
    assert!(is_backend_unavailable(&err));

    let err = CoreError::validation("some other error");
    assert!(!is_backend_unavailable(&err));
}

// ── Archive orchestration tests ────────────────────────────────

use ito_domain::backend::BackendSyncClient;
use std::cell::Cell;

struct FakeSyncClient {
    bundle: ArtifactBundle,
}

impl FakeSyncClient {
    fn new(change_id: &str) -> Self {
        Self {
            bundle: ArtifactBundle {
                change_id: change_id.to_string(),
                proposal: Some("# Proposal\nTest content".to_string()),
                design: None,
                tasks: Some("- [x] Task 1\n".to_string()),
                specs: vec![(
                    "test-cap".to_string(),
                    "## ADDED Requirements\n".to_string(),
                )],
                revision: "rev-final".to_string(),
            },
        }
    }
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, _change_id: &str) -> Result<ArtifactBundle, BackendError> {
        Ok(self.bundle.clone())
    }

    fn push(&self, _change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        Ok(PushResult {
            change_id: self.bundle.change_id.clone(),
            new_revision: "rev-new".to_string(),
        })
    }
}

struct FakeArchiveClient {
    should_fail: bool,
    call_count: Cell<usize>,
}

impl FakeArchiveClient {
    fn success() -> Self {
        Self {
            should_fail: false,
            call_count: Cell::new(0),
        }
    }

    fn failing() -> Self {
        Self {
            should_fail: true,
            call_count: Cell::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.call_count.get()
    }
}

impl BackendArchiveClient for FakeArchiveClient {
    fn mark_archived(&self, change_id: &str) -> Result<ArchiveResult, BackendError> {
        self.call_count.set(self.call_count.get() + 1);
        if self.should_fail {
            return Err(BackendError::Unavailable(
                "backend offline during archive".to_string(),
            ));
        }
        Ok(ArchiveResult {
            change_id: change_id.to_string(),
            archived_at: "2026-02-28T12:00:00Z".to_string(),
        })
    }
}

fn setup_change_on_disk(ito_path: &std::path::Path, change_id: &str) {
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(change_dir.join("specs/test-cap")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Proposal").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [x] Done").unwrap();
    std::fs::write(
        change_dir.join("specs/test-cap/spec.md"),
        "## ADDED Requirements\n",
    )
    .unwrap();
}

#[test]
fn archive_with_backend_happy_path() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "test-change";
    setup_change_on_disk(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    let outcome = archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        false,
    )
    .unwrap();

    // Verify specs were updated
    assert_eq!(outcome.specs_updated, vec!["test-cap"]);

    // Verify archive name contains the change id
    assert!(outcome.archive_name.contains(change_id));

    // Verify backend was called
    assert_eq!(archive_client.calls(), 1);
    assert_eq!(outcome.backend_result.change_id, change_id);

    // Verify the change is in the archive directory
    let archive_dir = ito_path
        .join("changes")
        .join("archive")
        .join(&outcome.archive_name);
    assert!(archive_dir.exists(), "archive directory should exist");

    // Verify original change dir is gone
    let original_dir = ito_path.join("changes").join(change_id);
    assert!(!original_dir.exists(), "original change dir should be gone");

    // Verify spec was copied to main specs tree
    let main_spec = ito_path.join("specs").join("test-cap").join("spec.md");
    assert!(main_spec.exists(), "main spec should exist");
}

#[test]
fn archive_with_backend_skip_specs() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "test-change";
    setup_change_on_disk(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    let outcome = archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        true, // skip_specs
    )
    .unwrap();

    // Specs should not have been updated
    assert!(outcome.specs_updated.is_empty());

    // But archive should still succeed
    let archive_dir = ito_path
        .join("changes")
        .join("archive")
        .join(&outcome.archive_name);
    assert!(archive_dir.exists());

    // Main spec should NOT exist
    let main_spec = ito_path.join("specs").join("test-cap").join("spec.md");
    assert!(
        !main_spec.exists(),
        "main spec should not be created when skip_specs is true"
    );
}

#[test]
fn archive_with_backend_backend_unavailable() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "test-change";
    setup_change_on_disk(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::failing();

    let err = archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        false,
    )
    .unwrap_err();

    let msg = err.to_string();
    assert!(
        msg.contains("Backend unavailable"),
        "should report backend unavailability: {msg}"
    );

    // The local archive still happened (the move completed before mark_archived failed)
    // This is by design — the local state is correct; backend can be retried.
}
