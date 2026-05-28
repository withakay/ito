use super::*;
use ito_domain::audit::event::{AuditEvent, EventContext, SCHEMA_VERSION};
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
        count: 1,
        ctx: test_ctx(),
    }
}

fn write_tasks_file(root: &Path, change_id: &str, file: &str, content: &str) {
    let path = root.join(".ito/changes").join(change_id);
    std::fs::create_dir_all(&path).expect("create dirs");
    std::fs::write(path.join(file), content).expect("write tasks");
}

fn write_tasks(root: &Path, change_id: &str, content: &str) {
    write_tasks_file(root, change_id, "tasks.md", content);
}

fn write_schema_apply_tracks(root: &Path, tracking_file: &str) {
    let schema_dir = root
        .join(".ito")
        .join("templates")
        .join("schemas")
        .join("spec-driven");
    std::fs::create_dir_all(&schema_dir).expect("schema dirs");
    std::fs::write(
        schema_dir.join("schema.yaml"),
        format!(
            "name: spec-driven\nversion: 1\nartifacts: []\napply:\n  tracks: {tracking_file}\n"
        ),
    )
    .expect("write schema.yaml");
}

#[test]
fn build_file_state_from_default_tasks_md() {
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
fn build_file_state_uses_apply_tracks_when_set() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    write_schema_apply_tracks(tmp.path(), "todo.md");
    write_tasks_file(
        tmp.path(),
        "test-change",
        "todo.md",
        "# Tasks\n\n## Wave 1\n\n### Task 1.1: Test\n- **Status**: [x] complete\n",
    );
    std::fs::write(
        tmp.path().join(".ito/changes/test-change/.ito.yaml"),
        "schema: spec-driven\n",
    )
    .expect("write .ito.yaml");

    let state = build_file_state(&ito_path, "test-change");
    assert_eq!(state.len(), 1);

    let key = EntityKey {
        entity: "task".to_string(),
        entity_id: "1.1".to_string(),
        scope: Some("test-change".to_string()),
    };
    assert_eq!(state.get(&key), Some(&"complete".to_string()));
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
    let writer = default_audit_store(&ito_path);
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

    let writer = default_audit_store(&ito_path);
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

    let writer = default_audit_store(&ito_path);
    writer
        .append(&make_event("1.1", "ch", "create", Some("pending")))
        .unwrap();

    let report = run_reconcile(&ito_path, Some("ch"), true);
    assert!(report.drifts.is_empty());
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
    let writer = default_audit_store(&ito_path);
    writer
        .append(&make_event("1.1", "ch", "create", Some("pending")))
        .unwrap();

    let report = run_reconcile(&ito_path, Some("ch"), false);
    // Task in log but not in files -> Extra
    assert_eq!(report.drifts.len(), 1);
}

#[test]
fn reconcile_fix_clears_extra_task_drift() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ito_path = tmp.path().join(".ito");

    let writer = default_audit_store(&ito_path);
    writer
        .append(&make_event("1.1", "ch", "create", Some("pending")))
        .unwrap();

    let report = run_reconcile(&ito_path, Some("ch"), true);
    assert!(report.drifts.is_empty());
    assert_eq!(report.events_written, 1);

    let report = run_reconcile(&ito_path, Some("ch"), true);
    assert!(report.drifts.is_empty());
    assert_eq!(report.events_written, 0);
}
