//! State materialization from audit event sequences.
//!
//! Replays an ordered sequence of audit events to reconstruct the latest
//! known status of each entity. The resulting `AuditState` is used by the
//! reconciliation engine to compare against file-on-disk state.

use std::collections::HashMap;

use super::event::{AuditEvent, ops};

/// Key for uniquely identifying an entity in the materialized state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityKey {
    /// Entity type string (e.g., "task", "change").
    pub entity: String,
    /// Entity identifier (e.g., "1.1", "009-02").
    pub entity_id: String,
    /// Scoping context (e.g., change_id for tasks).
    pub scope: Option<String>,
}

/// The materialized state: a map from entity keys to their last-known status.
#[derive(Debug, Clone)]
pub struct AuditState {
    /// Map from entity key to last-known status string.
    pub entities: HashMap<EntityKey, String>,
    /// Total number of events replayed.
    pub event_count: usize,
}

/// Replay a sequence of events to build the materialized state.
///
/// Events are processed in order. For each event, the `to` field (if present)
/// becomes the current status of the entity identified by
/// `(entity, entity_id, scope)`.
pub fn materialize_state(events: &[AuditEvent]) -> AuditState {
    let mut entities: HashMap<EntityKey, String> = HashMap::new();

    for event in events {
        let key = EntityKey {
            entity: event.entity.clone(),
            entity_id: event.entity_id.clone(),
            scope: event.scope.clone(),
        };

        // If the event has a `to` value, that becomes the current state.
        // For events like "archive" that don't have a `to`, we use the op
        // as a sentinel value (e.g., "archived").
        if let Some(to) = &event.to {
            entities.insert(key, to.clone());
        } else if event.op == ops::RECONCILED {
            entities.remove(&key);
        } else if event.op == ops::CHANGE_ARCHIVE {
            entities.insert(key, "archived".to_string());
        }
    }

    AuditState {
        event_count: events.len(),
        entities,
    }
}

#[cfg(test)]
#[path = "materialize_tests.rs"]
mod materialize_tests;
