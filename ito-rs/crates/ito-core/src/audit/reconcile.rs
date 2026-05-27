//! Reconciliation engine: compare routed audit history against file-on-disk
//! state, detect drift, and optionally emit compensating events.

use std::path::Path;

use ito_domain::audit::context::resolve_context;
use ito_domain::audit::event::AuditEvent;
use ito_domain::audit::materialize::{EntityKey, materialize_state};
use ito_domain::audit::reconcile::{Drift, FileState, compute_drift, generate_compensating_events};
use ito_domain::tasks::{TaskStatus, parse_tasks_tracking_file};

use super::reader::read_audit_events;
use super::store::default_audit_store;

/// Result of a reconciliation run.
#[derive(Debug)]
pub struct ReconcileReport {
    /// Drift items detected.
    pub drifts: Vec<Drift>,
    /// Number of compensating events written (0 if dry-run).
    pub events_written: usize,
    /// Scope of the reconciliation (change ID or "project").
    pub scoped_to: String,
}

/// Build file state from a change's tracking file for a specific change.
///
/// Reads the tasks file and produces a `FileState` map of task statuses.
pub fn build_file_state(ito_path: &Path, change_id: &str) -> FileState {
    let Ok(path) = crate::tasks::tracking_file_path(ito_path, change_id) else {
        return FileState::new();
    };
    let Ok(contents) = ito_common::io::read_to_string_std(&path) else {
        return FileState::new();
    };

    let parsed = parse_tasks_tracking_file(&contents);
    let mut state = FileState::new();

    for task in &parsed.tasks {
        let key = EntityKey {
            entity: "task".to_string(),
            entity_id: task.id.clone(),
            scope: Some(change_id.to_string()),
        };

        let status_str = match task.status {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in-progress",
            TaskStatus::Complete => "complete",
            TaskStatus::Shelved => "shelved",
        };

        state.insert(key, status_str.to_string());
    }

    state
}

/// Run reconciliation: compare audit log against file state, report drift,
/// and optionally write compensating events.
///
/// If `change_id` is Some, reconciles only tasks for that change.
/// If `fix` is true, writes compensating events to the log.
pub fn run_reconcile(ito_path: &Path, change_id: Option<&str>, fix: bool) -> ReconcileReport {
    let Some(change_id) = change_id else {
        // Project-wide reconciliation: iterate all active changes
        return run_project_reconcile(ito_path, fix);
    };

    // Read all events and filter to this change's scope
    let all_events = read_audit_events(ito_path);
    let mut scoped_events = Vec::new();
    for event in &all_events {
        if event.scope.as_deref() == Some(change_id) && event.entity == "task" {
            scoped_events.push(event.clone());
        }
    }

    let audit_state = materialize_state(&scoped_events);
    let file_state = build_file_state(ito_path, change_id);
    let mut drifts = compute_drift(&audit_state.entities, &file_state);

    let events_written = if fix && !drifts.is_empty() {
        let ctx = resolve_context(ito_path);
        let compensating = generate_compensating_events(&drifts, Some(change_id), &ctx);
        let writer = default_audit_store(ito_path);
        let mut written = 0;
        for event in &compensating {
            if has_equivalent_compensating_event(&scoped_events, event) {
                continue;
            }
            if writer.append(event).is_ok() {
                written += 1;
            }
        }
        if written > 0 {
            let all_events = read_audit_events(ito_path);
            let mut scoped_events = Vec::new();
            for event in &all_events {
                if event.scope.as_deref() == Some(change_id) && event.entity == "task" {
                    scoped_events.push(event.clone());
                }
            }
            let audit_state = materialize_state(&scoped_events);
            let file_state = build_file_state(ito_path, change_id);
            drifts = compute_drift(&audit_state.entities, &file_state);
        }
        written
    } else {
        0
    };

    ReconcileReport {
        drifts,
        events_written,
        scoped_to: change_id.to_string(),
    }
}

fn has_equivalent_compensating_event(events: &[AuditEvent], event: &AuditEvent) -> bool {
    events.iter().any(|existing| {
        existing.entity == event.entity
            && existing.entity_id == event.entity_id
            && existing.scope == event.scope
            && existing.op == event.op
            && existing.from == event.from
            && existing.to == event.to
            && existing.actor == event.actor
            && existing.by == event.by
    })
}

/// Project-wide reconciliation across all active changes.
fn run_project_reconcile(ito_path: &Path, fix: bool) -> ReconcileReport {
    let changes_dir = ito_common::paths::changes_dir(ito_path);

    let Ok(entries) = std::fs::read_dir(&changes_dir) else {
        return ReconcileReport {
            drifts: Vec::new(),
            events_written: 0,
            scoped_to: "project".to_string(),
        };
    };

    let mut all_drifts = Vec::new();
    let mut total_written = 0;

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        // Skip the archive directory
        if name == "archive" {
            continue;
        }

        let report = run_reconcile(ito_path, Some(name), fix);
        all_drifts.extend(report.drifts);
        total_written += report.events_written;
    }

    ReconcileReport {
        drifts: all_drifts,
        events_written: total_written,
        scoped_to: "project".to_string(),
    }
}

#[cfg(test)]
#[path = "reconcile_tests.rs"]
mod reconcile_tests;
