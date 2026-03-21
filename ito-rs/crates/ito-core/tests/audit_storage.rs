use ito_core::audit::{
    AuditEventStore, AuditStorageLocation, EventFilter, read_audit_events_filtered_from_store,
    read_audit_events_from_store,
};
use ito_domain::audit::event::{AuditEvent, EventContext, SCHEMA_VERSION};
use ito_domain::audit::writer::AuditWriter;
use std::sync::Mutex;

#[derive(Default)]
struct MemoryAuditStore {
    events: Mutex<Vec<AuditEvent>>,
}

impl AuditWriter for MemoryAuditStore {
    fn append(&self, event: &AuditEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.events
            .lock()
            .expect("memory audit store lock")
            .push(event.clone());
        Ok(())
    }
}

impl AuditEventStore for MemoryAuditStore {
    fn read_all(&self) -> Vec<AuditEvent> {
        self.events.lock().expect("memory audit store lock").clone()
    }

    fn location(&self) -> AuditStorageLocation {
        AuditStorageLocation::Other("memory".to_string())
    }
}

fn memory_store(events: Vec<AuditEvent>) -> MemoryAuditStore {
    MemoryAuditStore {
        events: Mutex::new(events),
    }
}

fn event(entity: &str, entity_id: &str, scope: Option<&str>, op: &str) -> AuditEvent {
    AuditEvent {
        v: SCHEMA_VERSION,
        ts: "2026-03-16T12:00:00.000Z".to_string(),
        entity: entity.to_string(),
        entity_id: entity_id.to_string(),
        scope: scope.map(str::to_string),
        op: op.to_string(),
        from: None,
        to: Some("pending".to_string()),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        ctx: EventContext {
            session_id: "s1".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    }
}

#[test]
fn reads_events_from_injected_store_without_filesystem_path() {
    let store = memory_store(vec![event("task", "1.1", Some("009-03"), "create")]);

    let events = read_audit_events_from_store(&store);

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].entity_id, "1.1");
}

#[test]
fn filters_events_from_injected_store() {
    let store = memory_store(vec![
        event("task", "1.1", Some("009-03"), "create"),
        event("change", "009-03", None, "create"),
    ]);
    let filter = EventFilter {
        entity: Some("task".to_string()),
        ..Default::default()
    };

    let events = read_audit_events_filtered_from_store(&store, &filter);

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].entity, "task");
}

#[test]
fn memory_store_append_persists_events() {
    let store = MemoryAuditStore::default();

    store
        .append(&event("task", "1.1", Some("009-03"), "create"))
        .expect("append to memory store");

    let events = read_audit_events_from_store(&store);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].entity_id, "1.1");
}
