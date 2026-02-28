//! Client-side forwarding of locally produced audit events to the backend.
//!
//! The forwarder reads new events from the local JSONL audit log, batches
//! them, and submits each batch to the backend event ingest endpoint with
//! an idempotency key so retries are safe. A checkpoint file under
//! `.ito/.state/` tracks the last forwarded line offset to avoid
//! re-sending the entire log on each invocation.

use std::path::Path;
use std::thread;
use std::time::Duration;

use ito_domain::audit::event::AuditEvent;
use ito_domain::backend::{BackendError, BackendEventIngestClient, EventBatch};

use crate::backend_client::{idempotency_key, is_retriable_status};
use crate::errors::{CoreError, CoreResult};

/// Maximum events per batch submission.
const DEFAULT_BATCH_SIZE: usize = 100;

/// Checkpoint file name within `.ito/.state/`.
const CHECKPOINT_FILE: &str = "event-forward-offset";

/// Result of a forwarding run, used for CLI diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardResult {
    /// Total events forwarded in this run.
    pub forwarded: usize,
    /// Total events skipped as duplicates by the backend.
    pub duplicates: usize,
    /// Number of batches that failed after exhausting retries.
    pub failed_batches: usize,
    /// Total events in the local log.
    pub total_local: usize,
    /// Offset after forwarding (line count forwarded so far).
    pub new_offset: usize,
}

/// Configuration for the event forwarder.
#[derive(Debug, Clone)]
pub struct ForwarderConfig {
    /// Maximum events per batch.
    pub batch_size: usize,
    /// Maximum retry attempts per batch on transient failure.
    pub max_retries: u32,
    /// Base delay between retries (doubles on each attempt).
    pub retry_base_delay: Duration,
}

impl Default for ForwarderConfig {
    fn default() -> Self {
        Self {
            batch_size: DEFAULT_BATCH_SIZE,
            max_retries: 3,
            retry_base_delay: Duration::from_millis(500),
        }
    }
}

/// Forward new local audit events to the backend.
///
/// Reads the audit log, skips events already forwarded (tracked by offset),
/// batches new events, and submits each batch with an idempotency key.
/// Updates the checkpoint file on success.
///
/// This function is designed to be called best-effort from the CLI after
/// command completion in backend mode.
pub fn forward_events(
    ingest_client: &dyn BackendEventIngestClient,
    ito_path: &Path,
    config: &ForwarderConfig,
) -> CoreResult<ForwardResult> {
    let all_events = read_all_events(ito_path);
    let total_local = all_events.len();
    let current_offset = read_checkpoint(ito_path);

    if current_offset >= total_local {
        return Ok(ForwardResult {
            forwarded: 0,
            duplicates: 0,
            failed_batches: 0,
            total_local,
            new_offset: current_offset,
        });
    }

    let new_events = &all_events[current_offset..];
    let mut forwarded = 0usize;
    let mut duplicates = 0usize;
    let mut failed_batches = 0usize;
    let mut offset = current_offset;

    for chunk in new_events.chunks(config.batch_size) {
        let batch = EventBatch {
            events: chunk.to_vec(),
            idempotency_key: idempotency_key("event-forward"),
        };

        match submit_with_retry(ingest_client, &batch, config) {
            Ok(result) => {
                forwarded += result.accepted;
                duplicates += result.duplicates;
                offset += chunk.len();
                // Persist offset after each successful batch
                if let Err(e) = write_checkpoint(ito_path, offset) {
                    tracing::warn!("failed to write forwarding checkpoint: {e}");
                }
            }
            Err(e) => {
                tracing::warn!("event forwarding batch failed: {e}");
                failed_batches += 1;
                // Stop forwarding on failure to preserve ordering
                break;
            }
        }
    }

    Ok(ForwardResult {
        forwarded,
        duplicates,
        failed_batches,
        total_local,
        new_offset: offset,
    })
}

/// Submit a batch with bounded retries on transient failures.
fn submit_with_retry(
    client: &dyn BackendEventIngestClient,
    batch: &EventBatch,
    config: &ForwarderConfig,
) -> CoreResult<ito_domain::backend::EventIngestResult> {
    let mut attempts = 0u32;
    loop {
        match client.ingest(batch) {
            Ok(result) => return Ok(result),
            Err(err) => {
                attempts += 1;
                if !is_retriable_backend_error(&err) || attempts > config.max_retries {
                    return Err(backend_ingest_error_to_core(err));
                }
                // Exponential backoff
                let delay = config.retry_base_delay * 2u32.saturating_pow(attempts - 1);
                thread::sleep(delay);
            }
        }
    }
}

/// Check if a backend error is transient and worth retrying.
fn is_retriable_backend_error(err: &BackendError) -> bool {
    match err {
        BackendError::Unavailable(_) => true,
        BackendError::Other(msg) => {
            // Check for HTTP status codes embedded in the message
            if let Some(code_str) = msg.strip_prefix("HTTP ")
                && let Ok(code) = code_str
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u16>()
            {
                return is_retriable_status(code);
            }
            false
        }
        BackendError::Unauthorized(_) => false,
        BackendError::NotFound(_) => false,
        BackendError::LeaseConflict(_) => false,
        BackendError::RevisionConflict(_) => false,
    }
}

/// Convert a backend ingest error to a `CoreError`.
fn backend_ingest_error_to_core(err: BackendError) -> CoreError {
    match err {
        BackendError::Unavailable(msg) => CoreError::process(format!(
            "Backend unavailable during event forwarding: {msg}"
        )),
        BackendError::Unauthorized(msg) => CoreError::validation(format!(
            "Backend auth failed during event forwarding: {msg}"
        )),
        BackendError::NotFound(msg) => {
            CoreError::not_found(format!("Backend ingest endpoint not found: {msg}"))
        }
        BackendError::Other(msg) => {
            CoreError::process(format!("Backend error during event forwarding: {msg}"))
        }
        BackendError::LeaseConflict(c) => CoreError::process(format!(
            "Unexpected lease conflict during event forwarding: {}",
            c.change_id
        )),
        BackendError::RevisionConflict(c) => CoreError::process(format!(
            "Unexpected revision conflict during event forwarding: {}",
            c.change_id
        )),
    }
}

// ── Checkpoint persistence ─────────────────────────────────────────

/// Path to the forwarding checkpoint file.
fn checkpoint_path(ito_path: &Path) -> std::path::PathBuf {
    ito_path.join(".state").join(CHECKPOINT_FILE)
}

/// Read the current forwarding offset from the checkpoint file.
///
/// Returns 0 if the file does not exist or is malformed.
fn read_checkpoint(ito_path: &Path) -> usize {
    let path = checkpoint_path(ito_path);
    let Ok(content) = std::fs::read_to_string(&path) else {
        return 0;
    };
    content.trim().parse::<usize>().unwrap_or(0)
}

/// Write the forwarding offset to the checkpoint file.
fn write_checkpoint(ito_path: &Path, offset: usize) -> CoreResult<()> {
    let path = checkpoint_path(ito_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| CoreError::io("creating checkpoint directory", e))?;
    }
    std::fs::write(&path, offset.to_string())
        .map_err(|e| CoreError::io("writing forwarding checkpoint", e))
}

// ── Event reading ──────────────────────────────────────────────────

/// Read all audit events from the local JSONL log.
///
/// Returns an empty vec if the log does not exist. Malformed lines are
/// skipped (same behavior as `audit::reader::read_audit_events`).
fn read_all_events(ito_path: &Path) -> Vec<AuditEvent> {
    crate::audit::read_audit_events(ito_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
    use ito_domain::backend::{BackendError, EventIngestResult};
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

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
        let writer = crate::audit::FsAuditWriter::new(ito_path);
        for event in events {
            crate::audit::AuditWriter::append(&writer, event).unwrap();
        }
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
}
