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
pub use worktree::{aggregate_worktree_events, discover_worktrees};
pub use writer::FsAuditWriter;
