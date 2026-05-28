use super::*;
use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
use ito_domain::backend::{BackendError, EventIngestResult};
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git_repo(repo: &Path) {
    run_git(repo, &["init"]);
    run_git(repo, &["config", "user.email", "test@example.com"]);
    run_git(repo, &["config", "user.name", "Test User"]);
    run_git(repo, &["config", "commit.gpgsign", "false"]);
    std::fs::write(repo.join("README.md"), "hi\n").expect("write readme");
    run_git(repo, &["add", "README.md"]);
    run_git(repo, &["commit", "-m", "initial"]);
}

fn make_event(id: &str) -> AuditEvent {
    AuditEvent {
        v: SCHEMA_VERSION,
        ts: "2026-02-28T10:00:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: id.to_string(),
        scope: Some("test-change".to_string()),
        op: "create".to_string(),
        from: None,
        to: Some("pending".to_string()),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        count: 1,
        ctx: EventContext {
            session_id: "test-sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    }
}

/// A fake ingest client that records calls and returns configurable results.
struct FakeIngestClient {
    call_count: AtomicUsize,
    results: Mutex<Vec<Result<EventIngestResult, BackendError>>>,
}

impl FakeIngestClient {
    fn always_ok() -> Self {
        Self {
            call_count: AtomicUsize::new(0),
            results: Mutex::new(Vec::new()),
        }
    }

    fn with_results(results: Vec<Result<EventIngestResult, BackendError>>) -> Self {
        Self {
            call_count: AtomicUsize::new(0),
            results: Mutex::new(results),
        }
    }

    fn calls(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl BackendEventIngestClient for FakeIngestClient {
    fn ingest(&self, batch: &EventBatch) -> Result<EventIngestResult, BackendError> {
        let idx = self.call_count.fetch_add(1, Ordering::SeqCst);
        let results = self.results.lock().unwrap();
        if idx < results.len() {
            return results[idx].clone();
        }
        // Default: accept all
        Ok(EventIngestResult {
            accepted: batch.events.len(),
            duplicates: 0,
        })
    }
}

fn write_events_to_log(ito_path: &Path, events: &[AuditEvent]) {
    let writer = crate::audit::default_audit_store(ito_path);
    for event in events {
        crate::audit::AuditWriter::append(writer.as_ref(), event).unwrap();
    }
}

#[test]
fn forward_reads_events_from_routed_local_store() {
    let tmp = tempfile::tempdir().unwrap();
    init_git_repo(tmp.path());
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();

    let store = crate::audit::default_audit_store(&ito_path);
    crate::audit::AuditWriter::append(store.as_ref(), &make_event("1.1")).unwrap();

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig::default();
    let result = forward_events(&client, &ito_path, &config).unwrap();

    assert_eq!(result.forwarded, 1);
    assert_eq!(result.total_local, 1);
    assert_eq!(client.calls(), 1);
}

#[test]
fn forward_no_events_returns_zero() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");
    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig::default();

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 0);
    assert_eq!(result.total_local, 0);
    assert_eq!(result.new_offset, 0);
    assert_eq!(result.failed_batches, 0);
    assert_eq!(client.calls(), 0);
}

#[test]
fn forward_sends_all_new_events() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..5).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig {
        batch_size: 10,
        ..ForwarderConfig::default()
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 5);
    assert_eq!(result.total_local, 5);
    assert_eq!(result.new_offset, 5);
    assert_eq!(result.failed_batches, 0);
    assert_eq!(client.calls(), 1); // All in one batch
}

#[test]
fn forward_respects_checkpoint() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..5).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    // Pre-set checkpoint at 3 (already forwarded 3 events)
    write_checkpoint(&ito_path, 3).unwrap();

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig::default();

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 2); // Only events 3 and 4
    assert_eq!(result.new_offset, 5);
}

#[test]
fn forward_skips_when_fully_forwarded() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..3).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);
    write_checkpoint(&ito_path, 3).unwrap();

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig::default();

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 0);
    assert_eq!(result.new_offset, 3);
    assert_eq!(client.calls(), 0);
}

#[test]
fn forward_batches_correctly() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..7).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig {
        batch_size: 3,
        ..ForwarderConfig::default()
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 7);
    assert_eq!(client.calls(), 3); // 3 + 3 + 1
}

#[test]
fn forward_stops_on_permanent_failure() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..6).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::with_results(vec![
        Ok(EventIngestResult {
            accepted: 3,
            duplicates: 0,
        }),
        Err(BackendError::Unauthorized("bad token".to_string())),
    ]);
    let config = ForwarderConfig {
        batch_size: 3,
        max_retries: 0,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 3); // First batch succeeded
    assert_eq!(result.failed_batches, 1);
    assert_eq!(result.new_offset, 3); // Checkpoint at first batch end
}

#[test]
fn forward_retries_transient_failure() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..2).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::with_results(vec![
        Err(BackendError::Unavailable("timeout".to_string())),
        Ok(EventIngestResult {
            accepted: 2,
            duplicates: 0,
        }),
    ]);
    let config = ForwarderConfig {
        batch_size: 10,
        max_retries: 3,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 2);
    assert_eq!(result.failed_batches, 0);
    assert_eq!(client.calls(), 2); // 1 retry + 1 success
}

#[test]
fn forward_reports_duplicates() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..3).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::with_results(vec![Ok(EventIngestResult {
        accepted: 1,
        duplicates: 2,
    })]);
    let config = ForwarderConfig::default();

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 1);
    assert_eq!(result.duplicates, 2);
}

#[test]
fn checkpoint_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    assert_eq!(read_checkpoint(&ito_path), 0);

    write_checkpoint(&ito_path, 42).unwrap();
    assert_eq!(read_checkpoint(&ito_path), 42);

    write_checkpoint(&ito_path, 100).unwrap();
    assert_eq!(read_checkpoint(&ito_path), 100);
}

#[test]
fn checkpoint_missing_returns_zero() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");
    assert_eq!(read_checkpoint(&ito_path), 0);
}

#[test]
fn is_retriable_backend_error_checks() {
    assert!(is_retriable_backend_error(&BackendError::Unavailable(
        "timeout".to_string()
    )));
    assert!(!is_retriable_backend_error(&BackendError::Unauthorized(
        "bad".to_string()
    )));
    assert!(!is_retriable_backend_error(&BackendError::NotFound(
        "nope".to_string()
    )));
}

#[test]
fn forward_persists_checkpoint_per_batch() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..4).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events_to_log(&ito_path, &events);

    let client = FakeIngestClient::always_ok();
    let config = ForwarderConfig {
        batch_size: 2,
        ..ForwarderConfig::default()
    };

    forward_events(&client, &ito_path, &config).unwrap();
    // Checkpoint should be at 4 after all batches
    assert_eq!(read_checkpoint(&ito_path), 4);
}

#[test]
fn forward_result_equality() {
    let a = ForwardResult {
        forwarded: 5,
        duplicates: 0,
        failed_batches: 0,
        total_local: 5,
        new_offset: 5,
    };
    let b = a.clone();
    assert_eq!(a, b);
}
