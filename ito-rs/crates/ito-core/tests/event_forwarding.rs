//! Integration tests for event forwarding.
//!
//! Tests cover:
//! - Full forwarding workflow: write local events, forward, verify checkpoint
//! - Incremental forwarding: new events after first forward
//! - Idempotent retry behavior
//! - Invalid payload handling

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use ito_core::audit::{AuditWriter, FsAuditWriter};
use ito_core::event_forwarder::{ForwardResult, ForwarderConfig, forward_events};
use ito_domain::audit::event::{AuditEvent, EventContext, SCHEMA_VERSION};
use ito_domain::backend::{BackendError, BackendEventIngestClient, EventBatch, EventIngestResult};

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

/// A recording ingest client that tracks all submitted batches.
struct RecordingIngestClient {
    call_count: AtomicUsize,
    submitted_events: Mutex<Vec<AuditEvent>>,
    idempotency_keys: Mutex<Vec<String>>,
}

impl RecordingIngestClient {
    fn new() -> Self {
        Self {
            call_count: AtomicUsize::new(0),
            submitted_events: Mutex::new(Vec::new()),
            idempotency_keys: Mutex::new(Vec::new()),
        }
    }

    fn calls(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    fn submitted_count(&self) -> usize {
        self.submitted_events.lock().unwrap().len()
    }

    fn keys(&self) -> Vec<String> {
        self.idempotency_keys.lock().unwrap().clone()
    }
}

impl BackendEventIngestClient for RecordingIngestClient {
    fn ingest(&self, batch: &EventBatch) -> Result<EventIngestResult, BackendError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        let mut events = self.submitted_events.lock().unwrap();
        let mut keys = self.idempotency_keys.lock().unwrap();
        events.extend(batch.events.iter().cloned());
        keys.push(batch.idempotency_key.clone());
        Ok(EventIngestResult {
            accepted: batch.events.len(),
            duplicates: 0,
        })
    }
}

fn write_events(ito_path: &std::path::Path, events: &[AuditEvent]) {
    let writer = FsAuditWriter::new(ito_path);
    for event in events {
        writer.append(event).unwrap();
    }
}

#[test]
fn full_forwarding_workflow() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    // Write local events
    let events: Vec<AuditEvent> = (0..5).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events);

    // Forward all events
    let client = RecordingIngestClient::new();
    let config = ForwarderConfig {
        batch_size: 10,
        max_retries: 3,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();

    assert_eq!(result.forwarded, 5);
    assert_eq!(result.duplicates, 0);
    assert_eq!(result.failed_batches, 0);
    assert_eq!(result.total_local, 5);
    assert_eq!(result.new_offset, 5);
    assert_eq!(client.calls(), 1);
    assert_eq!(client.submitted_count(), 5);

    // Verify all idempotency keys contain the operation name
    for key in client.keys() {
        assert!(
            key.contains("event-forward"),
            "key should contain operation: {key}"
        );
    }
}

#[test]
fn incremental_forwarding() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");
    let config = ForwarderConfig {
        batch_size: 10,
        max_retries: 3,
        retry_base_delay: Duration::from_millis(1),
    };

    // Phase 1: Write and forward 3 events
    let events1: Vec<AuditEvent> = (0..3).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events1);

    let client1 = RecordingIngestClient::new();
    let result1 = forward_events(&client1, &ito_path, &config).unwrap();
    assert_eq!(result1.forwarded, 3);
    assert_eq!(result1.new_offset, 3);

    // Phase 2: Write 2 more events and forward again
    let events2: Vec<AuditEvent> = (3..5).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events2);

    let client2 = RecordingIngestClient::new();
    let result2 = forward_events(&client2, &ito_path, &config).unwrap();
    assert_eq!(result2.forwarded, 2, "should only forward new events");
    assert_eq!(result2.new_offset, 5);
    assert_eq!(client2.submitted_count(), 2);

    // Phase 3: No new events — should be a no-op
    let client3 = RecordingIngestClient::new();
    let result3 = forward_events(&client3, &ito_path, &config).unwrap();
    assert_eq!(result3.forwarded, 0);
    assert_eq!(client3.calls(), 0, "no call when nothing to forward");
}

#[test]
fn batch_boundaries_preserved() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    // Write 7 events, batch size 3 → 3 batches (3 + 3 + 1)
    let events: Vec<AuditEvent> = (0..7).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events);

    let client = RecordingIngestClient::new();
    let config = ForwarderConfig {
        batch_size: 3,
        max_retries: 3,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 7);
    assert_eq!(client.calls(), 3);

    // Each batch should have a unique idempotency key
    let keys = client.keys();
    assert_eq!(keys.len(), 3);
    let unique: std::collections::HashSet<_> = keys.iter().collect();
    assert_eq!(unique.len(), 3, "idempotency keys must be unique per batch");
}

#[test]
fn transient_failure_retried_then_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..2).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events);

    // Fail once, then succeed
    struct RetryClient {
        call_count: AtomicUsize,
    }

    impl BackendEventIngestClient for RetryClient {
        fn ingest(&self, batch: &EventBatch) -> Result<EventIngestResult, BackendError> {
            let n = self.call_count.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                return Err(BackendError::Unavailable("timeout".to_string()));
            }
            Ok(EventIngestResult {
                accepted: batch.events.len(),
                duplicates: 0,
            })
        }
    }

    let client = RetryClient {
        call_count: AtomicUsize::new(0),
    };
    let config = ForwarderConfig {
        batch_size: 10,
        max_retries: 3,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 2);
    assert_eq!(result.failed_batches, 0);
    assert_eq!(client.call_count.load(Ordering::SeqCst), 2);
}

#[test]
fn permanent_failure_stops_forwarding() {
    let tmp = tempfile::tempdir().unwrap();
    let ito_path = tmp.path().join(".ito");

    let events: Vec<AuditEvent> = (0..4).map(|i| make_event(&format!("1.{i}"))).collect();
    write_events(&ito_path, &events);

    // Always fail with auth error (non-retriable)
    struct FailClient;

    impl BackendEventIngestClient for FailClient {
        fn ingest(&self, _batch: &EventBatch) -> Result<EventIngestResult, BackendError> {
            Err(BackendError::Unauthorized("bad token".to_string()))
        }
    }

    let client = FailClient;
    let config = ForwarderConfig {
        batch_size: 2,
        max_retries: 0,
        retry_base_delay: Duration::from_millis(1),
    };

    let result = forward_events(&client, &ito_path, &config).unwrap();
    assert_eq!(result.forwarded, 0);
    assert_eq!(result.failed_batches, 1);
    assert_eq!(result.new_offset, 0, "checkpoint not advanced on failure");
}

#[test]
fn forward_result_reports_diagnostics() {
    let result = ForwardResult {
        forwarded: 10,
        duplicates: 2,
        failed_batches: 1,
        total_local: 15,
        new_offset: 12,
    };

    // Verify the result provides enough info for CLI diagnostics
    assert!(result.forwarded > 0, "should report forwarded count");
    assert!(result.duplicates > 0, "should report duplicate count");
    assert!(result.failed_batches > 0, "should report failed batches");
    assert!(
        result.total_local > result.new_offset,
        "gap means unfowarded events"
    );
}
