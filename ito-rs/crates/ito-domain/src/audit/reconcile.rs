//! Reconciliation diff logic: compare materialized audit state against
//! file-on-disk state and produce drift items and compensating events.
//!
//! This module contains only pure functions with no I/O. The orchestration
//! (reading files, writing events) lives in `ito-core`.

use std::collections::HashMap;

use super::event::{Actor, AuditEvent, AuditEventBuilder, EntityType, EventContext, ops};
use super::materialize::EntityKey;

/// File-on-disk state: a map from entity keys to their current status as
/// read from the filesystem (e.g., from tasks.md).
pub type FileState = HashMap<EntityKey, String>;

/// A single drift item: a discrepancy between audit log state and file state.
#[derive(Debug, Clone, PartialEq)]
pub enum Drift {
    /// Entity exists in files but has no events in the audit log.
    Missing {
        /// Entity key.
        key: EntityKey,
        /// Status found in the file.
        file_status: String,
    },
    /// Audit log and file disagree on the entity's status.
    Diverged {
        /// Entity key.
        key: EntityKey,
        /// Status according to the audit log.
        log_status: String,
        /// Status according to the file.
        file_status: String,
    },
    /// Entity has events in the audit log but does not exist in the files.
    Extra {
        /// Entity key.
        key: EntityKey,
        /// Status according to the audit log.
        log_status: String,
    },
}

impl std::fmt::Display for Drift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Drift::Missing { key, file_status } => write!(
                f,
                "Missing: {}/{} (scope: {:?}) has file status '{}' but no audit events",
                key.entity, key.entity_id, key.scope, file_status
            ),
            Drift::Diverged {
                key,
                log_status,
                file_status,
            } => write!(
                f,
                "Diverged: {}/{} (scope: {:?}) audit='{}' file='{}'",
                key.entity, key.entity_id, key.scope, log_status, file_status
            ),
            Drift::Extra { key, log_status } => write!(
                f,
                "Extra: {}/{} (scope: {:?}) has audit status '{}' but no file entry",
                key.entity, key.entity_id, key.scope, log_status
            ),
        }
    }
}

/// Compare materialized audit state against file-on-disk state.
///
/// Returns a list of drift items. An empty list means the log and files agree.
pub fn compute_drift(
    audit_entities: &HashMap<EntityKey, String>,
    file_state: &FileState,
) -> Vec<Drift> {
    let mut drifts = Vec::new();

    // Check all file entries against audit log
    for (key, file_status) in file_state {
        match audit_entities.get(key) {
            None => {
                drifts.push(Drift::Missing {
                    key: key.clone(),
                    file_status: file_status.clone(),
                });
            }
            Some(log_status) if log_status != file_status => {
                drifts.push(Drift::Diverged {
                    key: key.clone(),
                    log_status: log_status.clone(),
                    file_status: file_status.clone(),
                });
            }
            Some(_) => {
                // Match -- no drift
            }
        }
    }

    // Check for audit entries not in files (extras)
    for (key, log_status) in audit_entities {
        // Only report extras for task entities (other entities like config
        // may not have a corresponding file entry).
        if key.entity == "task" && !file_state.contains_key(key) {
            drifts.push(Drift::Extra {
                key: key.clone(),
                log_status: log_status.clone(),
            });
        }
    }

    // Sort for deterministic output
    drifts.sort_by(|a, b| {
        let key_a = match a {
            Drift::Missing { key, .. } => key,
            Drift::Diverged { key, .. } => key,
            Drift::Extra { key, .. } => key,
        };
        let key_b = match b {
            Drift::Missing { key, .. } => key,
            Drift::Diverged { key, .. } => key,
            Drift::Extra { key, .. } => key,
        };
        (&key_a.entity, &key_a.entity_id).cmp(&(&key_b.entity, &key_b.entity_id))
    });

    drifts
}

/// Generate compensating events that bring the audit log in sync with the file state.
///
/// Each drift item produces a single `reconciled` event with `actor: "reconcile"`.
pub fn generate_compensating_events(
    drifts: &[Drift],
    scope: Option<&str>,
    ctx: &EventContext,
) -> Vec<AuditEvent> {
    let mut events = Vec::new();

    for drift in drifts {
        let event = match drift {
            Drift::Missing { key, file_status } => AuditEventBuilder::new()
                .entity(parse_entity_type(&key.entity))
                .entity_id(&key.entity_id)
                .op(ops::RECONCILED)
                .to(file_status)
                .actor(Actor::Reconcile)
                .by("@reconcile")
                .meta(serde_json::json!({
                    "reason": format!(
                        "{} '{}' has file status '{}' but no audit events",
                        key.entity, key.entity_id, file_status
                    )
                }))
                .ctx(ctx.clone()),
            Drift::Diverged {
                key,
                log_status,
                file_status,
            } => AuditEventBuilder::new()
                .entity(parse_entity_type(&key.entity))
                .entity_id(&key.entity_id)
                .op(ops::RECONCILED)
                .from(log_status)
                .to(file_status)
                .actor(Actor::Reconcile)
                .by("@reconcile")
                .meta(serde_json::json!({
                    "reason": format!(
                        "{} '{}' audit status '{}' differs from file status '{}'",
                        key.entity, key.entity_id, log_status, file_status
                    )
                }))
                .ctx(ctx.clone()),
            Drift::Extra { key, log_status } => AuditEventBuilder::new()
                .entity(parse_entity_type(&key.entity))
                .entity_id(&key.entity_id)
                .op(ops::RECONCILED)
                .from(log_status)
                .actor(Actor::Reconcile)
                .by("@reconcile")
                .meta(serde_json::json!({
                    "reason": format!(
                        "{} '{}' has audit status '{}' but no file entry",
                        key.entity, key.entity_id, log_status
                    )
                }))
                .ctx(ctx.clone()),
        };

        // Add scope if provided
        let event = if let Some(s) = scope {
            event.scope(s)
        } else if let Some(s) = match drift {
            Drift::Missing { key, .. } => key.scope.as_deref(),
            Drift::Diverged { key, .. } => key.scope.as_deref(),
            Drift::Extra { key, .. } => key.scope.as_deref(),
        } {
            event.scope(s)
        } else {
            event
        };

        if let Some(built) = event.build() {
            events.push(built);
        }
    }

    events
}

/// Parse entity type string to `EntityType`, defaulting to `Task` for unknown types.
fn parse_entity_type(s: &str) -> EntityType {
    match s {
        "task" => EntityType::Task,
        "change" => EntityType::Change,
        "module" => EntityType::Module,
        "wave" => EntityType::Wave,
        "planning" => EntityType::Planning,
        "config" => EntityType::Config,
        // Default to Task for any unrecognized entity type
        _ => EntityType::Task,
    }
}

#[cfg(test)]
#[path = "reconcile_tests.rs"]
mod reconcile_tests;
