use super::*;

#[test]
fn backend_error_display_lease_conflict() {
    let err = BackendError::LeaseConflict(LeaseConflict {
        change_id: "024-02".to_string(),
        holder: "agent-1".to_string(),
        expires_at: None,
    });
    let msg = err.to_string();
    assert!(msg.contains("024-02"));
    assert!(msg.contains("agent-1"));
    assert!(msg.contains("already claimed"));
}

#[test]
fn backend_error_display_revision_conflict() {
    let err = BackendError::RevisionConflict(RevisionConflict {
        change_id: "024-02".to_string(),
        local_revision: "rev-1".to_string(),
        server_revision: "rev-2".to_string(),
    });
    let msg = err.to_string();
    assert!(msg.contains("024-02"));
    assert!(msg.contains("rev-1"));
    assert!(msg.contains("rev-2"));
}

#[test]
fn backend_error_display_unavailable() {
    let err = BackendError::Unavailable("connection refused".to_string());
    assert!(err.to_string().contains("connection refused"));
}

#[test]
fn backend_error_display_unauthorized() {
    let err = BackendError::Unauthorized("invalid token".to_string());
    assert!(err.to_string().contains("invalid token"));
}

#[test]
fn backend_error_display_not_found() {
    let err = BackendError::NotFound("change xyz".to_string());
    assert!(err.to_string().contains("change xyz"));
}

#[test]
fn backend_error_display_other() {
    let err = BackendError::Other("unexpected".to_string());
    assert!(err.to_string().contains("unexpected"));
}

#[test]
fn event_batch_roundtrip() {
    let event = crate::audit::event::AuditEvent {
        v: 1,
        ts: "2026-02-28T10:00:00.000Z".to_string(),
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
        ctx: crate::audit::event::EventContext {
            session_id: "sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    };
    let batch = EventBatch {
        events: vec![event],
        idempotency_key: "key-123".to_string(),
    };
    let json = serde_json::to_string(&batch).unwrap();
    let restored: EventBatch = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.events.len(), 1);
    assert_eq!(restored.idempotency_key, "key-123");
}

#[test]
fn event_ingest_result_roundtrip() {
    let result = EventIngestResult {
        accepted: 5,
        duplicates: 2,
    };
    let json = serde_json::to_string(&result).unwrap();
    let restored: EventIngestResult = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.accepted, 5);
    assert_eq!(restored.duplicates, 2);
}

#[test]
fn archive_result_roundtrip() {
    let result = ArchiveResult {
        change_id: "024-05".to_string(),
        archived_at: "2026-02-28T12:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&result).unwrap();
    let restored: ArchiveResult = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.change_id, "024-05");
    assert_eq!(restored.archived_at, "2026-02-28T12:00:00Z");
}

#[test]
fn artifact_bundle_roundtrip() {
    let bundle = ArtifactBundle {
        change_id: "test-change".to_string(),
        proposal: Some("# Proposal".to_string()),
        design: None,
        tasks: Some("- [ ] Task 1".to_string()),
        specs: vec![("auth".to_string(), "## ADDED".to_string())],
        revision: "rev-abc".to_string(),
    };
    let json = serde_json::to_string(&bundle).unwrap();
    let restored: ArtifactBundle = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.change_id, "test-change");
    assert_eq!(restored.revision, "rev-abc");
    assert_eq!(restored.specs.len(), 1);
}
