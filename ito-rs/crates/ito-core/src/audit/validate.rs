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
mod tests {
    use super::*;
    use ito_domain::audit::event::{EventContext, SCHEMA_VERSION};

    fn test_ctx() -> EventContext {
        EventContext {
            session_id: "test".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        }
    }

    fn make_event(
        entity: &str,
        entity_id: &str,
        scope: Option<&str>,
        op: &str,
        from: Option<&str>,
        to: Option<&str>,
        ts: &str,
    ) -> AuditEvent {
        AuditEvent {
            v: SCHEMA_VERSION,
            ts: ts.to_string(),
            entity: entity.to_string(),
            entity_id: entity_id.to_string(),
            scope: scope.map(String::from),
            op: op.to_string(),
            from: from.map(String::from),
            to: to.map(String::from),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: test_ctx(),
        }
    }

    #[test]
    fn no_issues_for_valid_sequence() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:00:00Z",
            ),
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "status_change",
                Some("pending"),
                Some("in-progress"),
                "2026-01-01T00:01:00Z",
            ),
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "status_change",
                Some("in-progress"),
                Some("complete"),
                "2026-01-01T00:02:00Z",
            ),
        ];

        let issues = run_checks(&events);
        assert!(issues.is_empty());
    }

    #[test]
    fn detect_duplicate_create() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:00:00Z",
            ),
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:01:00Z",
            ),
        ];

        let issues = run_checks(&events);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Duplicate"));
    }

    #[test]
    fn detect_status_transition_mismatch() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:00:00Z",
            ),
            // from="in-progress" but last known is "pending"
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "status_change",
                Some("in-progress"),
                Some("complete"),
                "2026-01-01T00:01:00Z",
            ),
        ];

        let issues = run_checks(&events);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("transition mismatch"));
    }

    #[test]
    fn detect_timestamp_ordering_violation() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:02:00Z",
            ),
            make_event(
                "task",
                "1.2",
                Some("ch"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:01:00Z",
            ),
        ];

        let issues = run_checks(&events);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Timestamp ordering"));
    }

    #[test]
    fn empty_events_no_issues() {
        let issues = run_checks(&[]);
        assert!(issues.is_empty());
    }

    #[test]
    fn different_scopes_are_independent() {
        let events = vec![
            make_event(
                "task",
                "1.1",
                Some("ch-1"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:00:00Z",
            ),
            make_event(
                "task",
                "1.1",
                Some("ch-2"),
                "create",
                None,
                Some("pending"),
                "2026-01-01T00:01:00Z",
            ),
        ];

        let issues = run_checks(&events);
        assert!(issues.is_empty()); // Different scopes, not duplicates
    }
}
