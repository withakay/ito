use super::*;
use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};

fn test_ctx() -> EventContext {
    EventContext {
        session_id: "test".to_string(),
        harness_session_id: None,
        branch: None,
        worktree: None,
        commit: None,
    }
}

fn make_event(
    entity: &str,
    entity_id: &str,
    scope: Option<&str>,
    op: &str,
    from: Option<&str>,
    to: Option<&str>,
    ts: &str,
) -> AuditEvent {
    AuditEvent {
        v: SCHEMA_VERSION,
        ts: ts.to_string(),
        entity: entity.to_string(),
        entity_id: entity_id.to_string(),
        scope: scope.map(String::from),
        op: op.to_string(),
        from: from.map(String::from),
        to: to.map(String::from),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        count: 1,
        ctx: test_ctx(),
    }
}

#[test]
fn no_issues_for_valid_sequence() {
    let events = vec![
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:00:00Z",
        ),
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "status_change",
            Some("pending"),
            Some("in-progress"),
            "2026-01-01T00:01:00Z",
        ),
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "status_change",
            Some("in-progress"),
            Some("complete"),
            "2026-01-01T00:02:00Z",
        ),
    ];

    let issues = run_checks(&events);
    assert!(issues.is_empty());
}

#[test]
fn detect_duplicate_create() {
    let events = vec![
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:00:00Z",
        ),
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:01:00Z",
        ),
    ];

    let issues = run_checks(&events);
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("Duplicate"));
}

#[test]
fn detect_status_transition_mismatch() {
    let events = vec![
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:00:00Z",
        ),
        // from="in-progress" but last known is "pending"
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "status_change",
            Some("in-progress"),
            Some("complete"),
            "2026-01-01T00:01:00Z",
        ),
    ];

    let issues = run_checks(&events);
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("transition mismatch"));
}

#[test]
fn detect_timestamp_ordering_violation() {
    let events = vec![
        make_event(
            "task",
            "1.1",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:02:00Z",
        ),
        make_event(
            "task",
            "1.2",
            Some("ch"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:01:00Z",
        ),
    ];

    let issues = run_checks(&events);
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("Timestamp ordering"));
}

#[test]
fn empty_events_no_issues() {
    let issues = run_checks(&[]);
    assert!(issues.is_empty());
}

#[test]
fn different_scopes_are_independent() {
    let events = vec![
        make_event(
            "task",
            "1.1",
            Some("ch-1"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:00:00Z",
        ),
        make_event(
            "task",
            "1.1",
            Some("ch-2"),
            "create",
            None,
            Some("pending"),
            "2026-01-01T00:01:00Z",
        ),
    ];

    let issues = run_checks(&events);
    assert!(issues.is_empty()); // Different scopes, not duplicates
}
