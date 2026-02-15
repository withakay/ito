//! Audit log infrastructure: filesystem writer, event reader, reconciliation
//! engine, worktree discovery, and stream watcher.

pub mod reader;
pub mod reconcile;
pub mod stream;
pub mod validate;
pub mod worktree;
pub mod writer;

pub use reader::{EventFilter, read_audit_events, read_audit_events_filtered};
pub use reconcile::{ReconcileReport, build_file_state, run_reconcile};
pub use stream::{StreamConfig, StreamEvent, poll_new_events, read_initial_events};
pub use worktree::{aggregate_worktree_events, discover_worktrees, find_worktree_for_branch};
pub use writer::FsAuditWriter;

// Re-export domain audit types so adapters (ito-cli, ito-web) never need
// a direct ito-domain dependency for audit event construction.
pub use ito_domain::audit::context::{resolve_context, resolve_user_identity};
pub use ito_domain::audit::event::{
    Actor, AuditEvent, AuditEventBuilder, EntityType, EventContext, ops,
};
pub use ito_domain::audit::writer::AuditWriter;
