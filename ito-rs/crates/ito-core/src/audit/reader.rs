//! Audit log reader: parse events from JSONL file with optional filtering.

use std::path::Path;

use ito_domain::audit::event::AuditEvent;

use super::writer::audit_log_path;

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

/// Read all audit events from the project's JSONL file.
///
/// Malformed lines are skipped with a tracing warning. If the file does not
/// exist, returns an empty vector.
pub fn read_audit_events(ito_path: &Path) -> Vec<AuditEvent> {
    let path = audit_log_path(ito_path);

    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };

    let mut events = Vec::new();
    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<AuditEvent>(line) {
            Ok(event) => events.push(event),
            Err(e) => {
                tracing::warn!("audit log line {}: malformed event: {e}", line_num + 1);
            }
        }
    }

    events
}

/// Read audit events with a filter applied.
///
/// Only events matching all filter criteria are returned.
pub fn read_audit_events_filtered(ito_path: &Path, filter: &EventFilter) -> Vec<AuditEvent> {
    let all = read_audit_events(ito_path);
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
    use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};
    use ito_domain::audit::writer::AuditWriter;

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
}
