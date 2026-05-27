use super::*;
use crate::audit::materialize::EntityKey;

fn test_ctx() -> EventContext {
    EventContext {
        session_id: "test-session".to_string(),
        harness_session_id: None,
        branch: None,
        worktree: None,
        commit: None,
    }
}

fn task_key(id: &str, scope: &str) -> EntityKey {
    EntityKey {
        entity: "task".to_string(),
        entity_id: id.to_string(),
        scope: Some(scope.to_string()),
    }
}

#[test]
fn no_drift_when_states_match() {
    let mut audit = HashMap::new();
    audit.insert(task_key("1.1", "ch"), "complete".to_string());
    audit.insert(task_key("1.2", "ch"), "pending".to_string());

    let mut files = HashMap::new();
    files.insert(task_key("1.1", "ch"), "complete".to_string());
    files.insert(task_key("1.2", "ch"), "pending".to_string());

    let drifts = compute_drift(&audit, &files);
    assert!(drifts.is_empty());
}

#[test]
fn detect_missing_entity_in_log() {
    let audit: HashMap<EntityKey, String> = HashMap::new();
    let mut files = HashMap::new();
    files.insert(task_key("1.1", "ch"), "complete".to_string());

    let drifts = compute_drift(&audit, &files);
    assert_eq!(drifts.len(), 1);
    match &drifts[0] {
        Drift::Missing { key, file_status } => {
            assert_eq!(key.entity_id, "1.1");
            assert_eq!(file_status, "complete");
        }
        other => panic!("Expected Missing, got {other:?}"),
    }
}

#[test]
fn detect_diverged_status() {
    let mut audit = HashMap::new();
    audit.insert(task_key("1.1", "ch"), "pending".to_string());

    let mut files = HashMap::new();
    files.insert(task_key("1.1", "ch"), "complete".to_string());

    let drifts = compute_drift(&audit, &files);
    assert_eq!(drifts.len(), 1);
    match &drifts[0] {
        Drift::Diverged {
            log_status,
            file_status,
            ..
        } => {
            assert_eq!(log_status, "pending");
            assert_eq!(file_status, "complete");
        }
        other => panic!("Expected Diverged, got {other:?}"),
    }
}

#[test]
fn detect_extra_in_log() {
    let mut audit = HashMap::new();
    audit.insert(task_key("1.1", "ch"), "in-progress".to_string());

    let files: HashMap<EntityKey, String> = HashMap::new();

    let drifts = compute_drift(&audit, &files);
    assert_eq!(drifts.len(), 1);
    match &drifts[0] {
        Drift::Extra { key, log_status } => {
            assert_eq!(key.entity_id, "1.1");
            assert_eq!(log_status, "in-progress");
        }
        other => panic!("Expected Extra, got {other:?}"),
    }
}

#[test]
fn multiple_drift_types_detected() {
    let mut audit = HashMap::new();
    audit.insert(task_key("1.1", "ch"), "pending".to_string()); // diverged
    audit.insert(task_key("1.3", "ch"), "complete".to_string()); // extra

    let mut files = HashMap::new();
    files.insert(task_key("1.1", "ch"), "complete".to_string()); // diverged
    files.insert(task_key("1.2", "ch"), "pending".to_string()); // missing

    let drifts = compute_drift(&audit, &files);
    assert_eq!(drifts.len(), 3);
}

#[test]
fn display_drift_items() {
    let drift = Drift::Diverged {
        key: task_key("1.1", "ch"),
        log_status: "pending".to_string(),
        file_status: "complete".to_string(),
    };
    let s = drift.to_string();
    assert!(s.contains("Diverged"));
    assert!(s.contains("1.1"));
    assert!(s.contains("pending"));
    assert!(s.contains("complete"));
}

#[test]
fn generate_compensating_events_for_missing() {
    let drifts = vec![Drift::Missing {
        key: task_key("1.1", "ch"),
        file_status: "complete".to_string(),
    }];

    let events = generate_compensating_events(&drifts, Some("ch"), &test_ctx());
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].op, "reconciled");
    assert_eq!(events[0].actor, "reconcile");
    assert_eq!(events[0].to, Some("complete".to_string()));
    assert!(events[0].meta.is_some());
}

#[test]
fn generate_compensating_events_for_diverged() {
    let drifts = vec![Drift::Diverged {
        key: task_key("1.1", "ch"),
        log_status: "pending".to_string(),
        file_status: "complete".to_string(),
    }];

    let events = generate_compensating_events(&drifts, Some("ch"), &test_ctx());
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].from, Some("pending".to_string()));
    assert_eq!(events[0].to, Some("complete".to_string()));
}

#[test]
fn generate_compensating_events_for_extra() {
    let drifts = vec![Drift::Extra {
        key: task_key("1.1", "ch"),
        log_status: "in-progress".to_string(),
    }];

    let events = generate_compensating_events(&drifts, Some("ch"), &test_ctx());
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].from, Some("in-progress".to_string()));
    assert!(events[0].to.is_none());
}

#[test]
fn compensating_events_use_scope_from_drift_key() {
    let drifts = vec![Drift::Missing {
        key: task_key("1.1", "my-change"),
        file_status: "pending".to_string(),
    }];

    // Pass None for scope — should use the key's scope
    let events = generate_compensating_events(&drifts, None, &test_ctx());
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].scope, Some("my-change".to_string()));
}
