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
/// use ito_domain::tasks::{TasksParseResult, TasksFormat, ProgressInfo};
/// use ito_domain::traceability::compute_traceability;
/// let delta = vec![("Add login".to_string(), Some("REQ-1".to_string()))];
/// let tasks = TasksParseResult {
///     format: TasksFormat::Enhanced,
///     tasks: vec![],
///     waves: vec![],
///     diagnostics: vec![],
///     progress: ProgressInfo { total: 0, complete: 0, pending: 0, in_progress: 0, shelved: 0, remaining: 0 },
/// };
/// let result = compute_traceability(&delta, &tasks);
/// assert!(matches!(result.status, ito_domain::traceability::TraceStatus::Ready));
/// ```
pub fn compute_traceability(
    delta_requirements: &[(String, Option<String>)],
    tasks: &TasksParseResult,
) -> TraceabilityResult {
    let mut diagnostics: Vec<String> = Vec::new();

    // --- Determine trace status based on ID presence ---
    let total = delta_requirements.len();
    let mut with_id: Vec<&(String, Option<String>)> = Vec::new();
    let mut without_id: Vec<&(String, Option<String>)> = Vec::new();
    for req in delta_requirements {
        if req.1.is_some() {
            with_id.push(req);
        } else {
            without_id.push(req);
        }
    }

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
        let mut missing_ids: Vec<String> = Vec::with_capacity(without_id.len());
        for (title, _) in &without_id {
            missing_ids.push(title.clone());
        }
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
    let mut declared: Vec<String> = Vec::with_capacity(with_id.len());
    for (_, id) in &with_id {
        declared.push(id.as_ref().unwrap().clone());
    }

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

// Tests for this module live in `tests/traceability.rs` alongside the
// integration tests to keep this source file focused on production code.
