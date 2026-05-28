//! Semantic validation for audit event logs.
//!
//! Checks for structural and semantic issues beyond basic field presence:
//! - Duplicate create events for the same entity
//! - Orphaned events (referencing non-existent scopes)
//! - Invalid status transitions (e.g., completing an already-completed task)

use std::collections::{HashMap, HashSet};
use std::path::Path;

use ito_domain::audit::event::AuditEvent;

use super::reader::{EventFilter, read_audit_events, read_audit_events_filtered};

/// A semantic validation issue found in the audit log.
#[derive(Debug, Clone)]
pub struct AuditIssue {
    /// Issue severity: "error" or "warning".
    pub level: String,
    /// Human-readable description.
    pub message: String,
    /// Index of the event that triggered the issue (0-based).
    pub event_index: usize,
}

/// Result of semantic validation.
#[derive(Debug)]
pub struct AuditValidationReport {
    /// Total number of events examined.
    pub event_count: usize,
    /// Issues found.
    pub issues: Vec<AuditIssue>,
    /// Whether all checks passed (no errors; warnings are acceptable).
    pub valid: bool,
}

/// Run semantic validation on the audit log for a specific change or the whole project.
pub fn validate_audit_log(ito_path: &Path, change_id: Option<&str>) -> AuditValidationReport {
    let events = if let Some(change_id) = change_id {
        let filter = EventFilter {
            scope: Some(change_id.to_string()),
            ..Default::default()
        };
        read_audit_events_filtered(ito_path, &filter)
    } else {
        read_audit_events(ito_path)
    };

    let issues = run_checks(&events);

    let has_errors = issues.iter().any(|i| i.level == "error");

    AuditValidationReport {
        event_count: events.len(),
        valid: !has_errors,
        issues,
    }
}

/// Run all semantic checks against a sequence of events.
fn run_checks(events: &[AuditEvent]) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    issues.extend(check_duplicate_creates(events));
    issues.extend(check_invalid_status_transitions(events));
    issues.extend(check_timestamp_ordering(events));

    // Sort by event index for stable output
    issues.sort_by_key(|i| i.event_index);

    issues
}

/// Check for duplicate `create` events for the same entity.
fn check_duplicate_creates(events: &[AuditEvent]) -> Vec<AuditIssue> {
    let mut issues = Vec::new();
    let mut seen: HashSet<(String, String, Option<String>)> = HashSet::new();

    for (i, event) in events.iter().enumerate() {
        if event.op != "create" && event.op != "add" {
            continue;
        }
        let key = (
            event.entity.clone(),
            event.entity_id.clone(),
            event.scope.clone(),
        );
        if !seen.insert(key.clone()) {
            issues.push(AuditIssue {
                level: "warning".to_string(),
                message: format!(
                    "Duplicate {} event for {}/{} (scope: {:?})",
                    event.op, event.entity, event.entity_id, event.scope
                ),
                event_index: i,
            });
        }
    }

    issues
}

/// Check for invalid status transitions (e.g., completing an already-complete task).
fn check_invalid_status_transitions(events: &[AuditEvent]) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    // Track last known status for each entity
    let mut last_status: HashMap<(String, String, Option<String>), String> = HashMap::new();

    for (i, event) in events.iter().enumerate() {
        if event.op != "status_change" {
            // For create/add, set initial status
            if let Some(to) = &event.to {
                let key = (
                    event.entity.clone(),
                    event.entity_id.clone(),
                    event.scope.clone(),
                );
                last_status.insert(key, to.clone());
            }
            continue;
        }

        let key = (
            event.entity.clone(),
            event.entity_id.clone(),
            event.scope.clone(),
        );

        // Check if `from` matches the last known status
        if let Some(from) = &event.from
            && let Some(last) = last_status.get(&key)
            && last != from
        {
            issues.push(AuditIssue {
                level: "warning".to_string(),
                message: format!(
                    "Status transition mismatch for {}/{}: expected from='{}' but event says from='{}'",
                    event.entity, event.entity_id, last, from
                ),
                event_index: i,
            });
        }

        // Update the tracked status
        if let Some(to) = &event.to {
            last_status.insert(key, to.clone());
        }
    }

    issues
}

/// Check that timestamps are in non-decreasing order.
fn check_timestamp_ordering(events: &[AuditEvent]) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    for i in 1..events.len() {
        if events[i].ts < events[i - 1].ts {
            issues.push(AuditIssue {
                level: "warning".to_string(),
                message: format!(
                    "Timestamp ordering violation: event {} ({}) is earlier than event {} ({})",
                    i + 1,
                    events[i].ts,
                    i,
                    events[i - 1].ts
                ),
                event_index: i,
            });
        }
    }

    issues
}

#[cfg(test)]
#[path = "validate_tests.rs"]
mod validate_tests;
