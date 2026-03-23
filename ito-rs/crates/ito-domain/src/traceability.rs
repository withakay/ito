//! Traceability computation for change-local requirement coverage.
//!
//! Given a set of delta requirements (each optionally carrying a stable ID) and a
//! parsed `tasks.md`, this module computes how well the tasks cover the declared
//! requirements and surfaces any gaps or inconsistencies.

use crate::tasks::{TaskStatus, TasksFormat, TasksParseResult};

/// Overall traceability readiness for a change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceStatus {
    /// All declared requirements have IDs and coverage can be computed.
    Ready,
    /// Some requirements have IDs but others do not — traceability is incomplete.
    Invalid {
        /// Requirement titles (not IDs) that are missing an ID.
        missing_ids: Vec<String>,
    },
    /// No requirement IDs are declared, or the tasks format does not support tracing.
    Unavailable {
        /// Human-readable explanation.
        reason: String,
    },
}

/// A requirement that is covered by at least one non-shelved task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoveredRequirement {
    /// The stable requirement ID.
    pub requirement_id: String,
    /// IDs of tasks that reference this requirement.
    pub covering_tasks: Vec<String>,
}

/// A task `Requirements` entry that does not match any declared requirement ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedReference {
    /// The task that contains the dangling reference.
    pub task_id: String,
    /// The requirement ID that could not be resolved.
    pub requirement_id: String,
}

/// Full result of computing traceability for a change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityResult {
    /// Overall status.
    pub status: TraceStatus,
    /// All requirement IDs declared in delta specs (deduplicated, sorted).
    pub declared_requirements: Vec<String>,
    /// Requirements that are covered by at least one non-shelved task.
    pub covered_requirements: Vec<CoveredRequirement>,
    /// Requirement IDs that have no covering non-shelved task.
    pub uncovered_requirements: Vec<String>,
    /// Task `Requirements` entries that do not match any declared requirement ID.
    pub unresolved_references: Vec<UnresolvedReference>,
    /// Informational or warning messages (e.g. duplicate IDs).
    pub diagnostics: Vec<String>,
}

/// Compute requirement coverage for a change from parsed delta specifications and tasks.
///
/// The function evaluates whether traceability can be computed and, if so, returns declared
/// requirement IDs (deduplicated and sorted), which declared requirements are covered by
/// non-shelved tasks (with the covering task IDs), which declared requirements are uncovered,
/// any dangling requirement references found in non-shelved tasks, and human-readable diagnostics.
///
/// Parameters:
/// - `delta_requirements`: pairs of `(requirement_title, requirement_id)` parsed from delta specs;
///   the `requirement_title` is used in diagnostics when an ID is missing.
/// - `tasks`: parsed tasks for the change; certain task formats (e.g., checkbox format) make
///   traceability unavailable.
///
/// # Returns
///
/// A `TraceabilityResult` describing the overall `TraceStatus` (Ready, Invalid with missing titles,
/// or Unavailable with a reason), the stabilized list of declared requirement IDs, covered and
/// uncovered requirements, unresolved task references, and any diagnostics such as duplicate IDs.
///
/// # Examples
///
/// ```
/// let delta = vec![("Add login".to_string(), Some("REQ-1".to_string()))];
/// let tasks = TasksParseResult { format: TasksFormat::Enhanced, tasks: vec![] };
/// let result = compute_traceability(&delta, &tasks);
/// assert!(matches!(result.status, TraceStatus::Ready));
/// ```
pub fn compute_traceability(
    delta_requirements: &[(String, Option<String>)],
    tasks: &TasksParseResult,
) -> TraceabilityResult {
    let mut diagnostics: Vec<String> = Vec::new();

    // --- Determine trace status based on ID presence ---
    let total = delta_requirements.len();
    let with_id: Vec<&(String, Option<String>)> = delta_requirements
        .iter()
        .filter(|(_, id)| id.is_some())
        .collect();
    let without_id: Vec<&(String, Option<String>)> = delta_requirements
        .iter()
        .filter(|(_, id)| id.is_none())
        .collect();

    if total == 0 || with_id.is_empty() {
        return TraceabilityResult {
            status: TraceStatus::Unavailable {
                reason: "No requirement IDs declared in delta specs".to_string(),
            },
            declared_requirements: Vec::new(),
            covered_requirements: Vec::new(),
            uncovered_requirements: Vec::new(),
            unresolved_references: Vec::new(),
            diagnostics,
        };
    }

    if tasks.format == TasksFormat::Checkbox {
        return TraceabilityResult {
            status: TraceStatus::Unavailable {
                reason: "Tasks use checkbox format; enhanced format required for traceability"
                    .to_string(),
            },
            declared_requirements: Vec::new(),
            covered_requirements: Vec::new(),
            uncovered_requirements: Vec::new(),
            unresolved_references: Vec::new(),
            diagnostics,
        };
    }

    if !without_id.is_empty() {
        let missing_ids: Vec<String> = without_id.iter().map(|(title, _)| title.clone()).collect();
        return TraceabilityResult {
            status: TraceStatus::Invalid {
                missing_ids: missing_ids.clone(),
            },
            declared_requirements: Vec::new(),
            covered_requirements: Vec::new(),
            uncovered_requirements: Vec::new(),
            unresolved_references: Vec::new(),
            diagnostics,
        };
    }

    // All requirements have IDs — collect them.
    let mut declared: Vec<String> = with_id
        .iter()
        .map(|(_, id)| id.as_ref().unwrap().clone())
        .collect();

    // Flag duplicate IDs.
    let mut seen_ids: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for id in &declared {
        if !seen_ids.insert(id.clone()) {
            diagnostics.push(format!("Duplicate requirement ID: {id}"));
        }
    }

    // Deduplicate and sort for stable output.
    declared.sort();
    declared.dedup();

    let declared_set: std::collections::BTreeSet<&str> =
        declared.iter().map(String::as_str).collect();

    // --- Compute coverage from non-shelved enhanced tasks ---
    // Map: requirement_id -> list of covering task IDs.
    let mut coverage: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut unresolved: Vec<UnresolvedReference> = Vec::new();

    for task in &tasks.tasks {
        if task.status == TaskStatus::Shelved {
            continue;
        }
        for req_id in &task.requirements {
            if declared_set.contains(req_id.as_str()) {
                coverage
                    .entry(req_id.clone())
                    .or_default()
                    .push(task.id.clone());
            } else {
                unresolved.push(UnresolvedReference {
                    task_id: task.id.clone(),
                    requirement_id: req_id.clone(),
                });
            }
        }
    }

    let mut covered: Vec<CoveredRequirement> = Vec::new();
    let mut uncovered: Vec<String> = Vec::new();

    for req_id in &declared {
        if let Some(covering_tasks) = coverage.get(req_id) {
            covered.push(CoveredRequirement {
                requirement_id: req_id.clone(),
                covering_tasks: covering_tasks.clone(),
            });
        } else {
            uncovered.push(req_id.clone());
        }
    }

    TraceabilityResult {
        status: TraceStatus::Ready,
        declared_requirements: declared,
        covered_requirements: covered,
        uncovered_requirements: uncovered,
        unresolved_references: unresolved,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::parse_tasks_tracking_file;

    fn req(title: &str, id: Option<&str>) -> (String, Option<String>) {
        (title.to_string(), id.map(str::to_string))
    }

    /// Constructs an enhanced-format markdown task string used in tests.
    ///
    /// The produced string contains a level-3 heading with the task id and lines for
    /// Dependencies, Requirements (omitted when `reqs` is empty), Updated At, and Status,
    /// formatted to match the enhanced task parser's expected input.
    ///
    /// # Parameters
    ///
    /// - `id`: task identifier inserted into the heading and used as the task id line.
    /// - `status`: human-readable status text placed after the status checkbox.
    /// - `reqs`: list of requirement IDs to include on the `**Requirements**` line; omitted when empty.
    ///
    /// # Returns
    ///
    /// A markdown string representing a single enhanced-format task.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = enhanced_task("T1", "In Progress", &["REQ-1", "REQ-2"]);
    /// assert!(s.contains("### Task T1: Task T1"));
    /// assert!(s.contains("- **Requirements**: REQ-1, REQ-2"));
    /// assert!(s.contains("- **Status**: [ ] In Progress"));
    /// ```
    fn enhanced_task(id: &str, status: &str, reqs: &[&str]) -> String {
        let req_line = if reqs.is_empty() {
            String::new()
        } else {
            format!("- **Requirements**: {}\n", reqs.join(", "))
        };
        format!(
            "### Task {id}: Task {id}\n- **Dependencies**: None\n{req_line}- **Updated At**: 2026-01-01\n- **Status**: [ ] {status}\n"
        )
    }

    /// Wraps the provided task markdown in a "Wave 1" section (with a default
    /// "Depends On: None" line) and parses the combined document.
    ///
    /// The function is a test helper that ensures the given fragment is treated as
    /// a single wave when passed to the parser.
    ///
    /// # Examples
    ///
    /// ```
    /// let md = "- **Requirements**: REQ-1\n\n- [ ] Implement feature A";
    /// let _ = make_tasks(md);
    /// ```
    fn make_tasks(tasks_md: &str) -> TasksParseResult {
        let full = format!("## Wave 1\n- **Depends On**: None\n\n{tasks_md}");
        parse_tasks_tracking_file(&full)
    }

    #[test]
    fn no_requirement_ids_gives_unavailable() {
        let reqs = vec![req("REQ A", None), req("REQ B", None)];
        let tasks = make_tasks(&enhanced_task("1.1", "pending", &[]));
        let result = compute_traceability(&reqs, &tasks);
        assert!(matches!(result.status, TraceStatus::Unavailable { .. }));
    }

    #[test]
    fn empty_requirements_gives_unavailable() {
        let tasks = make_tasks(&enhanced_task("1.1", "pending", &[]));
        let result = compute_traceability(&[], &tasks);
        assert!(matches!(result.status, TraceStatus::Unavailable { .. }));
    }

    #[test]
    fn checkbox_format_gives_unavailable() {
        let reqs = vec![req("REQ A", Some("REQ-001"))];
        let tasks = parse_tasks_tracking_file("- [ ] Task one\n");
        let result = compute_traceability(&reqs, &tasks);
        assert!(matches!(result.status, TraceStatus::Unavailable { .. }));
        if let TraceStatus::Unavailable { reason } = &result.status {
            assert!(reason.contains("checkbox"));
        }
    }

    #[test]
    fn partial_ids_gives_invalid() {
        let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", None)];
        let tasks = make_tasks(&enhanced_task("1.1", "pending", &[]));
        let result = compute_traceability(&reqs, &tasks);
        if let TraceStatus::Invalid { missing_ids } = &result.status {
            assert_eq!(missing_ids, &["REQ B".to_string()]);
        } else {
            panic!("expected Invalid, got {:?}", result.status);
        }
    }

    #[test]
    fn all_requirements_covered() {
        let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", Some("REQ-002"))];
        let tasks_md = format!(
            "{}{}",
            enhanced_task("1.1", "pending", &["REQ-001"]),
            enhanced_task("1.2", "pending", &["REQ-002"]),
        );
        let tasks = make_tasks(&tasks_md);
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert_eq!(result.covered_requirements.len(), 2);
        assert!(result.uncovered_requirements.is_empty());
        assert!(result.unresolved_references.is_empty());
    }

    #[test]
    fn uncovered_requirement() {
        let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", Some("REQ-002"))];
        let tasks_md = enhanced_task("1.1", "pending", &["REQ-001"]);
        let tasks = make_tasks(&tasks_md);
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert_eq!(result.covered_requirements.len(), 1);
        assert_eq!(result.uncovered_requirements, vec!["REQ-002".to_string()]);
    }

    #[test]
    fn unresolved_task_reference() {
        let reqs = vec![req("REQ A", Some("REQ-001"))];
        let tasks_md = enhanced_task("1.1", "pending", &["REQ-001", "REQ-GHOST"]);
        let tasks = make_tasks(&tasks_md);
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert_eq!(result.unresolved_references.len(), 1);
        assert_eq!(result.unresolved_references[0].task_id, "1.1");
        assert_eq!(result.unresolved_references[0].requirement_id, "REQ-GHOST");
    }

    #[test]
    fn shelved_task_does_not_count_as_coverage() {
        let reqs = vec![req("REQ A", Some("REQ-001"))];
        let tasks_md = enhanced_task("1.1", "shelved", &["REQ-001"]);
        let tasks = make_tasks(&tasks_md);
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert!(result.covered_requirements.is_empty());
        assert_eq!(result.uncovered_requirements, vec!["REQ-001".to_string()]);
    }

    #[test]
    fn duplicate_requirement_ids_flagged_in_diagnostics() {
        let reqs = vec![
            req("REQ A", Some("REQ-001")),
            req("REQ A dup", Some("REQ-001")),
        ];
        let tasks = make_tasks(&enhanced_task("1.1", "pending", &[]));
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert!(
            result.diagnostics.iter().any(|d| d.contains("REQ-001")),
            "expected duplicate diagnostic, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn multiple_tasks_can_cover_same_requirement() {
        let reqs = vec![req("REQ A", Some("REQ-001"))];
        let tasks_md = format!(
            "{}{}",
            enhanced_task("1.1", "pending", &["REQ-001"]),
            enhanced_task("1.2", "complete", &["REQ-001"]),
        );
        let tasks = make_tasks(&tasks_md);
        let result = compute_traceability(&reqs, &tasks);
        assert_eq!(result.status, TraceStatus::Ready);
        assert_eq!(result.covered_requirements.len(), 1);
        assert_eq!(result.covered_requirements[0].covering_tasks.len(), 2);
    }
}
