//! Reconciliation engine: compare audit log against file-on-disk state,
//! detect drift, and optionally emit compensating events.

use std::path::Path;

use ito_domain::audit::context::resolve_context;
use ito_domain::audit::materialize::{EntityKey, materialize_state};
use ito_domain::audit::reconcile::{Drift, FileState, compute_drift, generate_compensating_events};
use ito_domain::audit::writer::AuditWriter;
use ito_domain::tasks::{TaskStatus, parse_tasks_tracking_file, tasks_path};

use super::reader::read_audit_events;
use super::writer::FsAuditWriter;

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

/// Build file state from tasks.md for a specific change.
///
/// Reads the tasks file and produces a `FileState` map of task statuses.
pub fn build_file_state(ito_path: &Path, change_id: &str) -> FileState {
    let path = tasks_path(ito_path, change_id);
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
    let drifts = compute_drift(&audit_state.entities, &file_state);

    let events_written = if fix && !drifts.is_empty() {
        let ctx = resolve_context(ito_path);
        let compensating = generate_compensating_events(&drifts, Some(change_id), &ctx);
        let writer = FsAuditWriter::new(ito_path);
        let mut written = 0;
        for event in &compensating {
            if writer.append(event).is_ok() {
                written += 1;
            }
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
mod tests {
    use super::*;
    use ito_domain::audit::event::{AuditEvent, EventContext, SCHEMA_VERSION};
    use ito_domain::audit::writer::AuditWriter;

    fn test_ctx() -> EventContext {
        EventContext {
            session_id: "test".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        }
    }

    fn make_event(entity_id: &str, scope: &str, op: &str, to: Option<&str>) -> AuditEvent {
        AuditEvent {
            v: SCHEMA_VERSION,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "task".to_string(),
            entity_id: entity_id.to_string(),
            scope: Some(scope.to_string()),
            op: op.to_string(),
            from: None,
            to: to.map(String::from),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: test_ctx(),
        }
    }

    fn write_tasks(root: &Path, change_id: &str, content: &str) {
        let path = root.join(".ito/changes").join(change_id);
        std::fs::create_dir_all(&path).expect("create dirs");
        std::fs::write(path.join("tasks.md"), content).expect("write tasks");
    }

    #[test]
    fn build_file_state_from_tasks_md() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        write_tasks(
            tmp.path(),
            "test-change",
            "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [x] complete\n\n### Task 1.2: Test2\n- **Status**: [ ] pending\n",
        );

        let state = build_file_state(&ito_path, "test-change");
        assert_eq!(state.len(), 2);

        let key1 = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("test-change".to_string()),
        };
        assert_eq!(state.get(&key1), Some(&"complete".to_string()));
    }

    #[test]
    fn reconcile_no_drift() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        write_tasks(
            tmp.path(),
            "ch",
            "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [ ] pending\n",
        );

        // Write a matching audit event
        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("1.1", "ch", "create", Some("pending")))
            .unwrap();

        let report = run_reconcile(&ito_path, Some("ch"), false);
        assert!(report.drifts.is_empty());
        assert_eq!(report.events_written, 0);
    }

    #[test]
    fn reconcile_detects_drift() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        // File says complete, log says pending
        write_tasks(
            tmp.path(),
            "ch",
            "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [x] complete\n",
        );

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("1.1", "ch", "create", Some("pending")))
            .unwrap();

        let report = run_reconcile(&ito_path, Some("ch"), false);
        assert_eq!(report.drifts.len(), 1);
        assert_eq!(report.events_written, 0);
    }

    #[test]
    fn reconcile_fix_writes_compensating_events() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        write_tasks(
            tmp.path(),
            "ch",
            "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [x] complete\n",
        );

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("1.1", "ch", "create", Some("pending")))
            .unwrap();

        let report = run_reconcile(&ito_path, Some("ch"), true);
        assert_eq!(report.drifts.len(), 1);
        assert_eq!(report.events_written, 1);

        // Read events to verify compensating event was written
        let events = read_audit_events(&ito_path);
        assert_eq!(events.len(), 2);
        assert_eq!(events[1].op, "reconciled");
        assert_eq!(events[1].actor, "reconcile");
    }

    #[test]
    fn reconcile_empty_log() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        write_tasks(
            tmp.path(),
            "ch",
            "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [ ] pending\n",
        );

        // No audit log at all
        let report = run_reconcile(&ito_path, Some("ch"), false);
        assert_eq!(report.drifts.len(), 1); // Missing
    }

    #[test]
    fn reconcile_missing_tasks_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        // No tasks.md but has events
        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("1.1", "ch", "create", Some("pending")))
            .unwrap();

        let report = run_reconcile(&ito_path, Some("ch"), false);
        // Task in log but not in files -> Extra
        assert_eq!(report.drifts.len(), 1);
    }
}
