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
mod reader_tests {
    include!("reader_tests.rs");
}
