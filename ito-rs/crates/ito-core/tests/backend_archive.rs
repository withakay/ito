//! Integration tests for backend-mode archive orchestration.
//!
//! Tests cover: happy-path archive with backend sync, skip-specs mode,
//! backend unavailability, and verification that local repo state is
//! committable after archive.

use std::cell::Cell;
use std::path::Path;

use ito_core::backend_coordination;
use ito_domain::backend::{
    ArchiveResult, ArtifactBundle, BackendArchiveClient, BackendError, BackendSyncClient,
    PushResult,
};
use tempfile::TempDir;

// ── Fake implementations ────────────────────────────────────────────

struct FakeSyncClient {
    bundle: ArtifactBundle,
}

impl FakeSyncClient {
    fn new(change_id: &str) -> Self {
        Self {
            bundle: ArtifactBundle {
                change_id: change_id.to_string(),
                proposal: Some("# Proposal\nFinal version from backend".to_string()),
                design: Some("## Design\nArchitecture notes".to_string()),
                tasks: Some("- [x] Task 1\n- [x] Task 2\n".to_string()),
                specs: vec![
                    (
                        "backend-archive-sync".to_string(),
                        "## ADDED Requirements\n### Requirement: Backend archive\n".to_string(),
                    ),
                    (
                        "cli-archive".to_string(),
                        "## ADDED Requirements\n### Requirement: CLI archive backend\n".to_string(),
                    ),
                ],
                revision: "rev-final".to_string(),
            },
        }
    }

    fn unavailable() -> Self {
        Self {
            bundle: ArtifactBundle {
                change_id: String::new(),
                proposal: None,
                design: None,
                tasks: None,
                specs: Vec::new(),
                revision: String::new(),
            },
        }
    }
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, change_id: &str) -> Result<ArtifactBundle, BackendError> {
        if self.bundle.change_id.is_empty() {
            return Err(BackendError::Unavailable(format!(
                "backend offline for change '{change_id}'"
            )));
        }
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

fn setup_change(ito_path: &Path, change_id: &str) {
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(change_dir.join("specs/backend-archive-sync")).unwrap();
    std::fs::create_dir_all(change_dir.join("specs/cli-archive")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Proposal").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [x] Done").unwrap();
    std::fs::write(
        change_dir.join("specs/backend-archive-sync/spec.md"),
        "## ADDED Requirements\n",
    )
    .unwrap();
    std::fs::write(
        change_dir.join("specs/cli-archive/spec.md"),
        "## ADDED Requirements\n",
    )
    .unwrap();
}

// ── Tests ───────────────────────────────────────────────────────────

#[test]
fn backend_archive_happy_path_produces_committable_state() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_test-archive";
    setup_change(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    let outcome = backend_coordination::archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        false,
    )
    .unwrap();

    // 1. Archive directory exists with all artifacts
    let archive_dir = ito_path
        .join("changes")
        .join("archive")
        .join(&outcome.archive_name);
    assert!(archive_dir.exists(), "archive dir must exist");
    assert!(
        archive_dir.join("proposal.md").exists(),
        "archived proposal must exist"
    );
    assert!(
        archive_dir.join("tasks.md").exists(),
        "archived tasks must exist"
    );
    assert!(
        archive_dir.join("design.md").exists(),
        "archived design must exist (pulled from backend)"
    );

    // 2. Original change directory is gone
    let original_dir = ito_path.join("changes").join(change_id);
    assert!(
        !original_dir.exists(),
        "original change dir must be removed"
    );

    // 3. Specs were copied to main specs tree
    assert!(
        ito_path.join("specs/backend-archive-sync/spec.md").exists(),
        "backend-archive-sync spec must be in main tree"
    );
    assert!(
        ito_path.join("specs/cli-archive/spec.md").exists(),
        "cli-archive spec must be in main tree"
    );
    assert_eq!(outcome.specs_updated.len(), 2);

    // 4. Backend was notified
    assert_eq!(archive_client.calls(), 1);
    assert_eq!(outcome.backend_result.change_id, change_id);
    assert_eq!(outcome.backend_result.archived_at, "2026-02-28T12:00:00Z");
}

#[test]
fn backend_archive_does_not_mutate_local_module_markdown() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_remote-safe";
    setup_change(&ito_path, change_id);

    let module_dir = ito_path.join("modules").join("024_demo");
    std::fs::create_dir_all(&module_dir).unwrap();
    std::fs::write(
        module_dir.join("module.md"),
        format!("# Demo\n\n## Changes\n- [ ] {change_id}\n"),
    )
    .unwrap();

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    let outcome = backend_coordination::archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        false,
    )
    .unwrap();

    let module_md = std::fs::read_to_string(module_dir.join("module.md")).unwrap();
    assert!(
        module_md.contains(&format!("- [ ] {change_id}")),
        "backend archive should not mutate local module markdown"
    );

    let archive_dir = ito_path
        .join("changes")
        .join("archive")
        .join(outcome.archive_name);
    assert!(archive_dir.exists(), "archive dir must still exist");
}

#[test]
fn backend_archive_with_skip_specs_does_not_copy_specs() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_skip-specs";
    setup_change(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    let outcome = backend_coordination::archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        true, // skip_specs
    )
    .unwrap();

    // Archive exists
    let archive_dir = ito_path
        .join("changes")
        .join("archive")
        .join(&outcome.archive_name);
    assert!(archive_dir.exists());

    // Specs NOT copied to main tree
    assert!(outcome.specs_updated.is_empty());
    assert!(!ito_path.join("specs/backend-archive-sync/spec.md").exists());

    // Backend was still notified
    assert_eq!(archive_client.calls(), 1);
}

#[test]
fn backend_archive_fails_when_backend_unavailable_for_mark_archived() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_unavailable";
    setup_change(&ito_path, change_id);

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::failing();

    let err = backend_coordination::archive_with_backend(
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
        "error should indicate backend is unavailable: {msg}"
    );

    // The local archive DID complete (move happened before mark_archived),
    // so the change directory should be in the archive.
    let original_dir = ito_path.join("changes").join(change_id);
    assert!(
        !original_dir.exists(),
        "original change should be moved even when backend fails"
    );
}

#[test]
fn backend_archive_fails_when_pull_unavailable() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_pull-fail";
    setup_change(&ito_path, change_id);

    let sync_client = FakeSyncClient::unavailable();
    let archive_client = FakeArchiveClient::success();

    let err = backend_coordination::archive_with_backend(
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
        msg.contains("Backend unavailable") || msg.contains("backend offline"),
        "error should indicate backend pull failure: {msg}"
    );

    // The change should NOT have been moved — pull failed before archive
    let original_dir = ito_path.join("changes").join(change_id);
    assert!(
        original_dir.exists(),
        "original change should remain when pull fails"
    );

    // Backend mark_archived was never called
    assert_eq!(archive_client.calls(), 0);
}

#[test]
fn backend_archive_creates_backup_before_overwriting() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let change_id = "024-05_backup-test";
    setup_change(&ito_path, change_id);

    // Write distinctive local content that should be backed up
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::write(
        change_dir.join("proposal.md"),
        "# LOCAL PROPOSAL BEFORE PULL",
    )
    .unwrap();

    let sync_client = FakeSyncClient::new(change_id);
    let archive_client = FakeArchiveClient::success();

    backend_coordination::archive_with_backend(
        &sync_client,
        &archive_client,
        &ito_path,
        change_id,
        &backup_dir,
        false,
    )
    .unwrap();

    // Backup directory should have been created by the pull step
    assert!(backup_dir.exists(), "backup directory should exist");

    let entries: Vec<_> = std::fs::read_dir(&backup_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        !entries.is_empty(),
        "at least one backup snapshot should exist"
    );

    // Verify the backup contains the original local proposal
    let snapshot_dir = &entries[0].path();
    let backed_up_proposal = std::fs::read_to_string(snapshot_dir.join("proposal.md")).unwrap();
    assert_eq!(backed_up_proposal, "# LOCAL PROPOSAL BEFORE PULL");
}
