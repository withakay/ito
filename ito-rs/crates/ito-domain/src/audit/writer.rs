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
#[path = "writer_tests.rs"]
mod writer_tests;
