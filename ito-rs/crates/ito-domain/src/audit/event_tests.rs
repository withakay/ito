use super::*;

fn test_ctx() -> EventContext {
    EventContext {
        session_id: "test-session-id".to_string(),
        harness_session_id: None,
        branch: Some("main".to_string()),
        worktree: None,
        commit: Some("abc12345".to_string()),
    }
}

#[test]
fn audit_event_round_trip_serialization() {
    let event = AuditEvent {
        v: 1,
        ts: "2026-02-08T14:30:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: "2.1".to_string(),
        scope: Some("009-02_audit-log".to_string()),
        op: "status_change".to_string(),
        from: Some("pending".to_string()),
        to: Some("in-progress".to_string()),
        actor: "cli".to_string(),
        by: "@jack".to_string(),
        meta: None,
        count: 1,
        ctx: test_ctx(),
    };

    let json = serde_json::to_string(&event).expect("serialize");
    let parsed: AuditEvent = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(event, parsed);
}

#[test]
fn audit_event_serializes_to_single_line() {
    let event = AuditEvent {
        v: 1,
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
        count: 1,
        ctx: test_ctx(),
    };

    let json = serde_json::to_string(&event).expect("serialize");
    assert!(!json.contains('\n'));
}

#[test]
fn optional_fields_omitted_when_none() {
    let event = AuditEvent {
        v: 1,
        ts: "2026-02-08T14:30:00.000Z".to_string(),
        entity: "change".to_string(),
        entity_id: "test".to_string(),
        scope: None,
        op: "create".to_string(),
        from: None,
        to: None,
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        count: 1,
        ctx: EventContext {
            session_id: "sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    };

    let json = serde_json::to_string(&event).expect("serialize");
    assert!(!json.contains("scope"));
    assert!(!json.contains("from"));
    assert!(!json.contains("\"to\""));
    assert!(!json.contains("meta"));
    assert!(!json.contains("count"));
    assert!(!json.contains("harness_session_id"));
    assert!(!json.contains("branch"));
    assert!(!json.contains("worktree"));
    assert!(!json.contains("commit"));
}

#[test]
fn missing_count_deserializes_as_one() {
    let json = r#"{
            "v": 1,
            "ts": "2026-02-08T14:30:00.000Z",
            "entity": "task",
            "entity_id": "1.1",
            "op": "create",
            "actor": "cli",
            "by": "@test",
            "ctx": { "session_id": "sid" }
        }"#;

    let event: AuditEvent = serde_json::from_str(json).expect("deserialize");
    assert_eq!(event.count, 1);
}

#[test]
fn count_serializes_when_greater_than_one() {
    let event = AuditEvent {
        v: 1,
        ts: "2026-02-08T14:30:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: "1.1".to_string(),
        scope: None,
        op: "reconciled".to_string(),
        from: None,
        to: None,
        actor: "reconcile".to_string(),
        by: "@reconcile".to_string(),
        meta: None,
        count: 2,
        ctx: EventContext {
            session_id: "sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    };

    let json = serde_json::to_string(&event).expect("serialize");
    assert!(json.contains("\"count\":2"));
}

#[test]
fn entity_type_serializes_to_lowercase() {
    let json = serde_json::to_string(&EntityType::Task).expect("serialize");
    assert_eq!(json, "\"task\"");

    let json = serde_json::to_string(&EntityType::Config).expect("serialize");
    assert_eq!(json, "\"config\"");
}

#[test]
fn entity_type_round_trip() {
    let variants = [
        EntityType::Task,
        EntityType::Change,
        EntityType::Module,
        EntityType::Wave,
        EntityType::Planning,
        EntityType::Config,
    ];
    for variant in variants {
        let json = serde_json::to_string(&variant).expect("serialize");
        let parsed: EntityType = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(variant, parsed);
    }
}

#[test]
fn actor_serializes_to_lowercase() {
    assert_eq!(Actor::Cli.as_str(), "cli");
    assert_eq!(Actor::Reconcile.as_str(), "reconcile");
    assert_eq!(Actor::Ralph.as_str(), "ralph");
}

#[test]
fn actor_round_trip() {
    let variants = [Actor::Cli, Actor::Reconcile, Actor::Ralph];
    for variant in variants {
        let json = serde_json::to_string(&variant).expect("serialize");
        let parsed: Actor = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(variant, parsed);
    }
}

#[test]
fn builder_produces_valid_event() {
    let event = AuditEventBuilder::new()
        .entity(EntityType::Task)
        .entity_id("1.1")
        .scope("test-change")
        .op(ops::TASK_STATUS_CHANGE)
        .from("pending")
        .to("in-progress")
        .actor(Actor::Cli)
        .by("@jack")
        .ctx(test_ctx())
        .build()
        .expect("should build");

    assert_eq!(event.v, SCHEMA_VERSION);
    assert_eq!(event.entity, "task");
    assert_eq!(event.entity_id, "1.1");
    assert_eq!(event.scope, Some("test-change".to_string()));
    assert_eq!(event.op, "status_change");
    assert_eq!(event.from, Some("pending".to_string()));
    assert_eq!(event.to, Some("in-progress".to_string()));
    assert_eq!(event.actor, "cli");
    assert_eq!(event.by, "@jack");
    assert!(!event.ts.is_empty());
}

#[test]
fn builder_returns_none_without_required_fields() {
    assert!(AuditEventBuilder::new().build().is_none());

    // Missing entity_id
    assert!(
        AuditEventBuilder::new()
            .entity(EntityType::Task)
            .op("create")
            .actor(Actor::Cli)
            .by("@test")
            .ctx(test_ctx())
            .build()
            .is_none()
    );
}

#[test]
fn builder_with_meta() {
    let meta = serde_json::json!({"wave": 2, "name": "Test task"});
    let event = AuditEventBuilder::new()
        .entity(EntityType::Task)
        .entity_id("2.1")
        .scope("change-1")
        .op(ops::TASK_ADD)
        .to("pending")
        .actor(Actor::Cli)
        .by("@test")
        .meta(meta.clone())
        .ctx(test_ctx())
        .build()
        .expect("should build");

    assert_eq!(event.meta, Some(meta));
}

#[test]
fn schema_version_is_one() {
    assert_eq!(SCHEMA_VERSION, 1);
}

#[test]
fn event_context_round_trip() {
    let ctx = EventContext {
        session_id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890".to_string(),
        harness_session_id: Some("ses_abc123".to_string()),
        branch: Some("feat/audit-log".to_string()),
        worktree: Some("audit-log".to_string()),
        commit: Some("3a7f2b1c".to_string()),
    };

    let json = serde_json::to_string(&ctx).expect("serialize");
    let parsed: EventContext = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(ctx, parsed);
}

#[test]
fn entity_type_display() {
    assert_eq!(EntityType::Task.to_string(), "task");
    assert_eq!(EntityType::Planning.to_string(), "planning");
}

#[test]
fn entity_type_as_str_matches_serde() {
    let variants = [
        EntityType::Task,
        EntityType::Change,
        EntityType::Module,
        EntityType::Wave,
        EntityType::Planning,
        EntityType::Config,
    ];
    for variant in variants {
        let serde_str = serde_json::to_string(&variant)
            .expect("serialize")
            .trim_matches('"')
            .to_string();
        assert_eq!(variant.as_str(), serde_str);
    }
}
