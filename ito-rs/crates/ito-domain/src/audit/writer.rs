//! Audit writer trait and no-op implementation.
//!
//! The `AuditWriter` trait is object-safe for dynamic dispatch and defines
//! the single `append` method for recording audit events. The concrete
//! filesystem writer lives in `ito-core`; this module provides only the
//! trait and a no-op stub for testing.

use super::event::AuditEvent;

/// Trait for appending audit events to a log.
///
/// Implementations must be `Send + Sync` for use across async boundaries.
/// The trait is object-safe to allow `dyn AuditWriter`.
pub trait AuditWriter: Send + Sync {
    /// Append a single event to the audit log.
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// A no-op writer that silently discards all events.
///
/// Used when audit logging is disabled or unavailable (e.g., during `ito init`
/// before the `.ito/` directory exists).
pub struct NoopAuditWriter;

impl AuditWriter for NoopAuditWriter {
    fn append(&self, _event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::event::{EventContext, SCHEMA_VERSION};

    fn test_event() -> AuditEvent {
        AuditEvent {
            v: SCHEMA_VERSION,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
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

    #[test]
    fn noop_writer_returns_ok() {
        let writer = NoopAuditWriter;
        let event = test_event();
        assert!(writer.append(&event).is_ok());
    }

    #[test]
    fn noop_writer_is_object_safe() {
        let writer: Box<dyn AuditWriter> = Box::new(NoopAuditWriter);
        let event = test_event();
        assert!(writer.append(&event).is_ok());
    }

    #[test]
    fn noop_writer_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NoopAuditWriter>();
    }

    #[test]
    fn trait_is_object_safe_for_dyn_dispatch() {
        fn takes_writer(_w: &dyn AuditWriter) {}
        let noop = NoopAuditWriter;
        takes_writer(&noop);
    }
}
