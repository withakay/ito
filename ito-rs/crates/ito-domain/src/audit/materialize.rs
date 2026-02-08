//! State materialization from audit event sequences.
//!
//! Replays an ordered sequence of audit events to reconstruct the latest
//! known status of each entity. The resulting `AuditState` is used by the
//! reconciliation engine to compare against file-on-disk state.

use std::collections::HashMap;

use super::event::AuditEvent;

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
        } else if event.op == "archive" {
            entities.insert(key, "archived".to_string());
        }
    }

    AuditState {
        event_count: events.len(),
        entities,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::event::{EventContext, SCHEMA_VERSION};

    fn make_event(
        entity: &str,
        entity_id: &str,
        scope: Option<&str>,
        op: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> AuditEvent {
        AuditEvent {
            v: SCHEMA_VERSION,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: entity.to_string(),
            entity_id: entity_id.to_string(),
            scope: scope.map(String::from),
            op: op.to_string(),
            from: from.map(String::from),
            to: to.map(String::from),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: EventContext {
                session_id: "test".to_string(),
                harness_session_id: None,
                branch: None,
                worktree: None,
                commit: None,
            },
        }
    }

    #[test]
    fn empty_events_produce_empty_state() {
        let state = materialize_state(&[]);
        assert!(state.entities.is_empty());
        assert_eq!(state.event_count, 0);
    }

    #[test]
    fn single_create_event() {
        let events = vec![make_event(
            "task",
            "1.1",
            Some("change-1"),
            "create",
            None,
            Some("pending"),
        )];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("change-1".to_string()),
        };

        assert_eq!(state.entities.get(&key), Some(&"pending".to_string()));
        assert_eq!(state.event_count, 1);
    }

    #[test]
    fn status_change_updates_state() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "create",
                None,
                Some("pending"),
            ),
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "status_change",
                Some("pending"),
                Some("in-progress"),
            ),
        ];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("change-1".to_string()),
        };

        assert_eq!(state.entities.get(&key), Some(&"in-progress".to_string()));
        assert_eq!(state.event_count, 2);
    }

    #[test]
    fn multiple_entities_tracked_independently() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "create",
                None,
                Some("pending"),
            ),
            make_event(
                "task",
                "1.2",
                Some("change-1"),
                "create",
                None,
                Some("pending"),
            ),
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "status_change",
                Some("pending"),
                Some("complete"),
            ),
        ];

        let state = materialize_state(&events);

        let key1 = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("change-1".to_string()),
        };
        let key2 = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.2".to_string(),
            scope: Some("change-1".to_string()),
        };

        assert_eq!(state.entities.get(&key1), Some(&"complete".to_string()));
        assert_eq!(state.entities.get(&key2), Some(&"pending".to_string()));
    }

    #[test]
    fn archive_event_without_to_uses_sentinel() {
        let events = vec![make_event("change", "009-02", None, "archive", None, None)];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "change".to_string(),
            entity_id: "009-02".to_string(),
            scope: None,
        };

        assert_eq!(state.entities.get(&key), Some(&"archived".to_string()));
    }

    #[test]
    fn reconciled_events_update_state() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "create",
                None,
                Some("pending"),
            ),
            make_event(
                "task",
                "1.1",
                Some("change-1"),
                "reconciled",
                Some("pending"),
                Some("complete"),
            ),
        ];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("change-1".to_string()),
        };

        assert_eq!(state.entities.get(&key), Some(&"complete".to_string()));
    }

    #[test]
    fn global_entities_have_no_scope() {
        let events = vec![make_event(
            "config",
            "worktrees.enabled",
            None,
            "set",
            None,
            Some("true"),
        )];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "config".to_string(),
            entity_id: "worktrees.enabled".to_string(),
            scope: None,
        };

        assert_eq!(state.entities.get(&key), Some(&"true".to_string()));
    }

    #[test]
    fn last_event_wins() {
        let events = vec![
            make_event("task", "1.1", Some("ch"), "status_change", None, Some("a")),
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "status_change",
                Some("a"),
                Some("b"),
            ),
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "status_change",
                Some("b"),
                Some("c"),
            ),
        ];

        let state = materialize_state(&events);
        let key = EntityKey {
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("ch".to_string()),
        };

        assert_eq!(state.entities.get(&key), Some(&"c".to_string()));
    }
}
