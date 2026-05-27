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

    let mut current_offset = read_checkpoint(ito_path);
    if current_offset > total_local {
        current_offset = total_local;
    }

    if current_offset >= total_local {
        return Ok(ForwardResult {
            forwarded: 0,
            duplicates: 0,
            failed_batches: 0,
            total_local,
            new_offset: current_offset,
        });
    }

    let batch_size = config.batch_size.max(1);
    let new_events = &all_events[current_offset..];
    let mut forwarded = 0usize;
    let mut duplicates = 0usize;
    let mut failed_batches = 0usize;
    let mut offset = current_offset;

    for chunk in new_events.chunks(batch_size) {
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

/// Read all audit events from the routed local audit store.
fn read_all_events(ito_path: &Path) -> Vec<AuditEvent> {
    crate::audit::default_audit_store(ito_path).read_all()
}

#[cfg(test)]
#[path = "event_forwarder_tests.rs"]
mod event_forwarder_tests;
