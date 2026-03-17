//! Routed audit log reader with optional filtering.

use std::path::Path;

use ito_domain::audit::event::AuditEvent;

use super::store::AuditEventStore;

/// Filter criteria for reading audit events.
#[derive(Debug, Default, Clone)]
pub struct EventFilter {
    /// Only include events for this entity type.
    pub entity: Option<String>,
    /// Only include events scoped to this change.
    pub scope: Option<String>,
    /// Only include events with this operation.
    pub op: Option<String>,
}

impl EventFilter {
    /// Check if an event matches this filter.
    fn matches(&self, event: &AuditEvent) -> bool {
        if let Some(entity) = &self.entity
            && event.entity != *entity
        {
            return false;
        }
        if let Some(scope) = &self.scope {
            match &event.scope {
                Some(event_scope) if event_scope == scope => {}
                _ => return false,
            }
        }
        if let Some(op) = &self.op
            && event.op != *op
        {
            return false;
        }
        true
    }
}

/// Read all audit events from the project's routed audit store.
pub fn read_audit_events(ito_path: &Path) -> Vec<AuditEvent> {
    let store = super::store::default_audit_store(ito_path);
    read_audit_events_from_store(store.as_ref())
}

/// Read all audit events from an injected audit store.
pub fn read_audit_events_from_store(store: &dyn AuditEventStore) -> Vec<AuditEvent> {
    store.read_all()
}

/// Read audit events with a filter applied from the routed audit store.
pub fn read_audit_events_filtered(ito_path: &Path, filter: &EventFilter) -> Vec<AuditEvent> {
    let store = super::store::default_audit_store(ito_path);
    read_audit_events_filtered_from_store(store.as_ref(), filter)
}

/// Read audit events with a filter from an injected audit store.
pub fn read_audit_events_filtered_from_store(
    store: &dyn AuditEventStore,
    filter: &EventFilter,
) -> Vec<AuditEvent> {
    let all = read_audit_events_from_store(store);
    let mut filtered = Vec::new();
    for event in all {
        if filter.matches(&event) {
            filtered.push(event);
        }
    }
    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::writer::FsAuditWriter;
    use crate::audit::{AuditEventStore, AuditStorageLocation};
    use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
    use ito_domain::audit::writer::AuditWriter;

    #[derive(Default)]
    struct MemoryAuditStore {
        events: Vec<AuditEvent>,
    }

    impl AuditWriter for MemoryAuditStore {
        fn append(
            &self,
            _event: &AuditEvent,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    impl AuditEventStore for MemoryAuditStore {
        fn read_all(&self) -> Vec<AuditEvent> {
            self.events.clone()
        }

        fn location(&self) -> AuditStorageLocation {
            AuditStorageLocation::Other("memory")
        }
    }

    fn make_event(entity: &str, entity_id: &str, scope: Option<&str>, op: &str) -> AuditEvent {
        AuditEvent {
            v: SCHEMA_VERSION,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: entity.to_string(),
            entity_id: entity_id.to_string(),
            scope: scope.map(String::from),
            op: op.to_string(),
            from: None,
            to: Some("pending".to_string()),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: EventContext {
                session_id: "test-sid".to_string(),
                harness_session_id: None,
                branch: None,
                worktree: None,
                commit: None,
            },
        }
    }

    #[test]
    fn read_from_missing_file_returns_empty() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let events = read_audit_events(&tmp.path().join(".ito"));
        assert!(events.is_empty());
    }

    #[test]
    fn read_parses_valid_events() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch"), "create"))
            .unwrap();
        writer
            .append(&make_event("task", "1.2", Some("ch"), "create"))
            .unwrap();

        let events = read_audit_events(&ito_path);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].entity_id, "1.1");
        assert_eq!(events[1].entity_id, "1.2");
    }

    #[test]
    fn skips_malformed_lines() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch"), "create"))
            .unwrap();

        // Manually append a malformed line
        let log_path = super::super::writer::audit_log_path(&ito_path);
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&log_path)
            .unwrap();
        use std::io::Write;
        writeln!(file, "{{invalid json").unwrap();

        writer
            .append(&make_event("task", "1.2", Some("ch"), "create"))
            .unwrap();

        let events = read_audit_events(&ito_path);
        assert_eq!(events.len(), 2); // Malformed line skipped
    }

    #[test]
    fn skips_empty_lines() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch"), "create"))
            .unwrap();

        // Append empty lines
        let log_path = super::super::writer::audit_log_path(&ito_path);
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&log_path)
            .unwrap();
        use std::io::Write;
        writeln!(file).unwrap();
        writeln!(file).unwrap();

        let events = read_audit_events(&ito_path);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn filter_by_entity_type() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch"), "create"))
            .unwrap();
        writer
            .append(&make_event("change", "ch", None, "create"))
            .unwrap();

        let filter = EventFilter {
            entity: Some("task".to_string()),
            ..Default::default()
        };
        let events = read_audit_events_filtered(&ito_path, &filter);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity, "task");
    }

    #[test]
    fn filter_by_scope() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch-1"), "create"))
            .unwrap();
        writer
            .append(&make_event("task", "2.1", Some("ch-2"), "create"))
            .unwrap();

        let filter = EventFilter {
            scope: Some("ch-1".to_string()),
            ..Default::default()
        };
        let events = read_audit_events_filtered(&ito_path, &filter);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].scope, Some("ch-1".to_string()));
    }

    #[test]
    fn filter_by_operation() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch"), "create"))
            .unwrap();
        writer
            .append(&make_event("task", "1.1", Some("ch"), "status_change"))
            .unwrap();

        let filter = EventFilter {
            op: Some("status_change".to_string()),
            ..Default::default()
        };
        let events = read_audit_events_filtered(&ito_path, &filter);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].op, "status_change");
    }

    #[test]
    fn combined_filters() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");

        let writer = FsAuditWriter::new(&ito_path);
        writer
            .append(&make_event("task", "1.1", Some("ch-1"), "create"))
            .unwrap();
        writer
            .append(&make_event("task", "1.1", Some("ch-1"), "status_change"))
            .unwrap();
        writer
            .append(&make_event("task", "2.1", Some("ch-2"), "create"))
            .unwrap();
        writer
            .append(&make_event("change", "ch-1", None, "create"))
            .unwrap();

        let filter = EventFilter {
            entity: Some("task".to_string()),
            scope: Some("ch-1".to_string()),
            op: Some("create".to_string()),
        };
        let events = read_audit_events_filtered(&ito_path, &filter);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_id, "1.1");
    }

    #[test]
    fn reads_events_from_injected_store() {
        let store = MemoryAuditStore {
            events: vec![make_event("task", "1.1", Some("ch"), "create")],
        };

        let events = read_audit_events_from_store(&store);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_id, "1.1");
    }
}
