//! Integration tests for backend client mode.
//!
//! Tests cover: claim success/conflict, allocate no-work, pull/push success,
//! stale revision conflict, backend repository adapters, and config resolution.

use ito_config::types::{BackendApiConfig, BackendProjectConfig};
use ito_core::backend_change_repository::BackendChangeRepository;
use ito_core::backend_client::{is_retriable_status, resolve_backend_runtime};
use ito_core::backend_coordination;
use ito_core::backend_sync;
use ito_core::backend_task_repository::BackendTaskRepository;
use ito_core::errors::CoreError;
use ito_domain::backend::{
    AllocateResult, ArtifactBundle, BackendChangeReader, BackendError, BackendLeaseClient,
    BackendSyncClient, BackendTaskReader, ClaimResult, LeaseConflict, PushResult, ReleaseResult,
    RevisionConflict,
};
use ito_domain::changes::{Change, ChangeRepository, ChangeSummary};
use ito_domain::errors::{DomainError, DomainResult};
use ito_domain::tasks::TaskRepository;
use tempfile::TempDir;

// ── Fake implementations ────────────────────────────────────────────

struct FakeLeaseClient {
    claim_result: Result<ClaimResult, BackendError>,
    release_result: Result<ReleaseResult, BackendError>,
    allocate_result: Result<AllocateResult, BackendError>,
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

struct FakeSyncClient {
    pull_result: Result<ArtifactBundle, BackendError>,
    push_result: Result<PushResult, BackendError>,
}

impl BackendSyncClient for FakeSyncClient {
    fn pull(&self, _change_id: &str) -> Result<ArtifactBundle, BackendError> {
        self.pull_result.clone()
    }

    fn push(&self, _change_id: &str, _bundle: &ArtifactBundle) -> Result<PushResult, BackendError> {
        self.push_result.clone()
    }
}

struct FakeChangeReader {
    summaries: Vec<ChangeSummary>,
}

impl BackendChangeReader for FakeChangeReader {
    fn list_changes(
        &self,
        _filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Vec<ChangeSummary>> {
        Ok(self.summaries.clone())
    }

    fn get_change(
        &self,
        change_id: &str,
        _filter: ito_domain::changes::ChangeLifecycleFilter,
    ) -> DomainResult<Change> {
        Err(DomainError::not_found("change", change_id))
    }
}

struct FakeTaskReader {
    content: Option<String>,
}

impl BackendTaskReader for FakeTaskReader {
    fn load_tasks_content(&self, _change_id: &str) -> DomainResult<Option<String>> {
        Ok(self.content.clone())
    }
}

fn make_summary(id: &str, completed: u32, total: u32) -> ChangeSummary {
    ChangeSummary {
        id: id.to_string(),
        module_id: None,
        sub_module_id: None,
        completed_tasks: completed,
        shelved_tasks: 0,
        in_progress_tasks: 0,
        pending_tasks: total - completed,
        total_tasks: total,
        last_modified: chrono::Utc::now(),
        has_proposal: true,
        has_design: false,
        has_specs: true,
        has_tasks: true,
        orchestrate: ito_domain::changes::ChangeOrchestrateMetadata::default(),
    }
}

fn make_bundle(change_id: &str, revision: &str) -> ArtifactBundle {
    ArtifactBundle {
        change_id: change_id.to_string(),
        proposal: Some("# Proposal".to_string()),
        design: None,
        tasks: Some("- [x] Task 1\n- [ ] Task 2\n".to_string()),
        specs: vec![("auth".to_string(), "## ADDED Requirements\n".to_string())],
        revision: revision.to_string(),
    }
}

fn setup_local_change(ito_path: &std::path::Path, change_id: &str) {
    let change_dir = ito_path.join("changes").join(change_id);
    std::fs::create_dir_all(change_dir.join("specs/auth")).unwrap();
    std::fs::write(change_dir.join("proposal.md"), "# Proposal").unwrap();
    std::fs::write(change_dir.join("tasks.md"), "- [ ] Task 1\n").unwrap();
    std::fs::write(change_dir.join("specs/auth/spec.md"), "## ADDED").unwrap();
}

// ── Claim tests ─────────────────────────────────────────────────────

#[test]
fn claim_success_returns_holder_info() {
    let client = FakeLeaseClient {
        claim_result: Ok(ClaimResult {
            change_id: "024-01".to_string(),
            holder: "agent-1".to_string(),
            expires_at: Some("2026-03-01T12:00:00Z".to_string()),
        }),
        release_result: Err(BackendError::Other("unused".to_string())),
        allocate_result: Err(BackendError::Other("unused".to_string())),
    };

    let result = backend_coordination::claim_change(&client, "024-01").unwrap();
    assert_eq!(result.change_id, "024-01");
    assert_eq!(result.holder, "agent-1");
    assert!(result.expires_at.is_some());
}

#[test]
fn claim_conflict_returns_holder_error() {
    let client = FakeLeaseClient {
        claim_result: Err(BackendError::LeaseConflict(LeaseConflict {
            change_id: "024-01".to_string(),
            holder: "agent-2".to_string(),
            expires_at: None,
        })),
        release_result: Err(BackendError::Other("unused".to_string())),
        allocate_result: Err(BackendError::Other("unused".to_string())),
    };

    let err = backend_coordination::claim_change(&client, "024-01").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("agent-2"), "should mention holder: {msg}");
    assert!(
        msg.contains("Lease conflict"),
        "should mention conflict: {msg}"
    );
}

// ── Allocate tests ──────────────────────────────────────────────────

#[test]
fn allocate_no_work_returns_none() {
    let client = FakeLeaseClient {
        claim_result: Err(BackendError::Other("unused".to_string())),
        release_result: Err(BackendError::Other("unused".to_string())),
        allocate_result: Ok(AllocateResult { claim: None }),
    };

    let result = backend_coordination::allocate_change(&client).unwrap();
    assert!(result.claim.is_none());
}

#[test]
fn allocate_returns_claimed_change() {
    let client = FakeLeaseClient {
        claim_result: Err(BackendError::Other("unused".to_string())),
        release_result: Err(BackendError::Other("unused".to_string())),
        allocate_result: Ok(AllocateResult {
            claim: Some(ClaimResult {
                change_id: "024-02".to_string(),
                holder: "agent-3".to_string(),
                expires_at: None,
            }),
        }),
    };

    let result = backend_coordination::allocate_change(&client).unwrap();
    let claim = result.claim.unwrap();
    assert_eq!(claim.change_id, "024-02");
    assert_eq!(claim.holder, "agent-3");
}

// ── Pull/push tests ─────────────────────────────────────────────────

#[test]
fn pull_writes_artifacts_and_revision() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    std::fs::create_dir_all(&ito_path).unwrap();

    let bundle = make_bundle("test-change", "rev-42");
    let client = FakeSyncClient {
        pull_result: Ok(bundle),
        push_result: Err(BackendError::Other("unused".to_string())),
    };

    let result =
        backend_sync::pull_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap();
    assert_eq!(result.revision, "rev-42");

    // Verify local files
    let change_dir = ito_path.join("changes").join("test-change");
    assert!(change_dir.join("proposal.md").is_file());
    assert!(change_dir.join("tasks.md").is_file());
    assert!(change_dir.join("specs/auth/spec.md").is_file());

    // Verify revision metadata
    let rev = std::fs::read_to_string(change_dir.join(".backend-revision")).unwrap();
    assert_eq!(rev, "rev-42");
}

#[test]
fn push_success_updates_local_revision() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    setup_local_change(&ito_path, "test-change");

    let client = FakeSyncClient {
        pull_result: Err(BackendError::Other("unused".to_string())),
        push_result: Ok(PushResult {
            change_id: "test-change".to_string(),
            new_revision: "rev-99".to_string(),
        }),
    };

    let result =
        backend_sync::push_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap();
    assert_eq!(result.new_revision, "rev-99");

    let rev = std::fs::read_to_string(
        ito_path
            .join("changes")
            .join("test-change")
            .join(".backend-revision"),
    )
    .unwrap();
    assert_eq!(rev, "rev-99");
}

#[test]
fn push_stale_revision_gives_actionable_error() {
    let tmp = TempDir::new().unwrap();
    let ito_path = tmp.path().join(".ito");
    let backup_dir = tmp.path().join("backups");
    setup_local_change(&ito_path, "test-change");

    let client = FakeSyncClient {
        pull_result: Err(BackendError::Other("unused".to_string())),
        push_result: Err(BackendError::RevisionConflict(RevisionConflict {
            change_id: "test-change".to_string(),
            local_revision: "rev-1".to_string(),
            server_revision: "rev-5".to_string(),
        })),
    };

    let err =
        backend_sync::push_artifacts(&client, &ito_path, "test-change", &backup_dir).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("Revision conflict"),
        "should contain 'Revision conflict': {msg}"
    );
    assert!(msg.contains("rev-1"), "should contain local rev: {msg}");
    assert!(msg.contains("rev-5"), "should contain server rev: {msg}");
    assert!(
        msg.contains("ito tasks sync pull"),
        "should suggest pull: {msg}"
    );
}

// ── Backend repository adapter tests ────────────────────────────────

#[test]
fn backend_change_repo_lists_and_filters() {
    let reader = FakeChangeReader {
        summaries: vec![
            make_summary("001-01_done", 3, 3),
            make_summary("001-02_wip", 1, 3),
            make_summary("002-01_new", 0, 0),
        ],
    };
    let repo = BackendChangeRepository::new(reader);

    let all = repo.list().unwrap();
    assert_eq!(all.len(), 3);

    let incomplete = repo.list_incomplete().unwrap();
    assert_eq!(incomplete.len(), 2); // wip and new (0 tasks = incomplete)

    let complete = repo.list_complete().unwrap();
    assert_eq!(complete.len(), 1);
    assert_eq!(complete[0].id, "001-01_done");
}

#[test]
fn backend_task_repo_parses_from_content() {
    let reader = FakeTaskReader {
        content: Some("# Tasks\n- [x] Done\n- [ ] Pending\n- [ ] Also pending\n".to_string()),
    };
    let repo = BackendTaskRepository::new(reader);

    let (completed, total) = repo.get_task_counts("any").unwrap();
    assert_eq!(completed, 1);
    assert_eq!(total, 3);
}

#[test]
fn backend_task_repo_missing_returns_zero() {
    let reader = FakeTaskReader { content: None };
    let repo = BackendTaskRepository::new(reader);

    let (completed, total) = repo.get_task_counts("any").unwrap();
    assert_eq!(completed, 0);
    assert_eq!(total, 0);
}

// ── Config resolution tests ─────────────────────────────────────────

#[test]
fn config_disabled_returns_none() {
    let config = BackendApiConfig::default();
    let result = resolve_backend_runtime(&config).unwrap();
    assert!(result.is_none());
}

#[test]
fn config_enabled_with_token_resolves() {
    let config = BackendApiConfig {
        enabled: true,
        url: "http://localhost:9999".to_string(),
        token: Some("my-token".to_string()),
        project: BackendProjectConfig {
            org: Some("test-org".to_string()),
            repo: Some("test-repo".to_string()),
        },
        ..BackendApiConfig::default()
    };

    let runtime = resolve_backend_runtime(&config).unwrap().unwrap();
    assert_eq!(runtime.base_url, "http://localhost:9999");
    assert_eq!(runtime.token, "my-token");
    assert_eq!(runtime.org, "test-org");
    assert_eq!(runtime.repo, "test-repo");
}

#[test]
fn config_enabled_missing_token_fails_with_clear_message() {
    let config = BackendApiConfig {
        enabled: true,
        token: None,
        token_env_var: "ITO_INTEG_TEST_NONEXISTENT_VAR".to_string(),
        ..BackendApiConfig::default()
    };

    let err = resolve_backend_runtime(&config).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("ITO_INTEG_TEST_NONEXISTENT_VAR"));
    assert!(
        msg.contains("not set") || msg.contains("empty"),
        "should mention missing: {msg}"
    );
}

// ── Retriable status tests ──────────────────────────────────────────

#[test]
fn retriable_status_codes() {
    assert!(is_retriable_status(429));
    assert!(is_retriable_status(500));
    assert!(is_retriable_status(502));
    assert!(is_retriable_status(503));
    assert!(!is_retriable_status(200));
    assert!(!is_retriable_status(400));
    assert!(!is_retriable_status(401));
    assert!(!is_retriable_status(404));
}

// ── Backend unavailable detection ───────────────────────────────────

#[test]
fn backend_unavailable_detection() {
    assert!(backend_coordination::is_backend_unavailable(
        &CoreError::process("Backend unavailable during pull: timeout")
    ));
    assert!(!backend_coordination::is_backend_unavailable(
        &CoreError::validation("some other error")
    ));
    assert!(!backend_coordination::is_backend_unavailable(
        &CoreError::not_found("missing")
    ));
}
