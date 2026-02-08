//! Audit event domain types and pure functions.
//!
//! This module defines the append-only audit event model, state materialization,
//! reconciliation diff logic, and the writer trait. All types are storage-agnostic;
//! concrete writers live in `ito-core`.

pub mod context;
pub mod event;
pub mod materialize;
pub mod reconcile;
pub mod writer;

pub use context::{GitContext, resolve_user_identity};
pub use event::{
    Actor, AuditEvent, AuditEventBuilder, EntityType, EventContext, TaggedAuditEvent, WorktreeInfo,
    ops,
};
pub use materialize::{AuditState, materialize_state};
pub use reconcile::{Drift, FileState, compute_drift, generate_compensating_events};
pub use writer::{AuditWriter, NoopAuditWriter};
