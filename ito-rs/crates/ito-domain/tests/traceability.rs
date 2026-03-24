//! Integration tests for the traceability computation module.

use ito_domain::tasks::parse_tasks_tracking_file;
use ito_domain::traceability::{CoveredRequirement, TraceStatus, compute_traceability};

/// Create a requirement tuple from a title and an optional identifier.
///
/// The returned tuple contains an owned `String` for the title and an `Option<String>` for the id;
/// if `id` is `Some`, it is converted to an owned `String`.
///
/// # Examples
///
/// ```
/// let a = req("Requirement A", Some("REQ-001"));
/// assert_eq!(a, ("Requirement A".to_string(), Some("REQ-001".to_string())));
///
/// let b = req("Requirement B", None);
/// assert_eq!(b, ("Requirement B".to_string(), None));
/// ```
fn req(title: &str, id: Option<&str>) -> (String, Option<String>) {
    (title.to_string(), id.map(str::to_string))
}

/// Parses the provided task block(s) by wrapping them into a minimal enhanced `tasks.md` containing a single wave.
///
/// The wrapper prepends the following lines before `tasks_md`:
/// ```text
/// ## Wave 1
/// - **Depends On**: None
/// ```
///
/// # Arguments
///
/// * `tasks_md` - Task block(s) in the enhanced tasks.md format (task headings and metadata) to be appended after the single wave header.
///
/// # Returns
///
/// The parsed `TasksParseResult` produced by `parse_tasks_tracking_file`.
///
/// # Examples
///
/// ```
/// let tasks_md = "\
/// ### Task 1: Task 1
/// - **Dependencies**: None
/// - **Updated At**: 2026-01-01
/// - **Status**: [ ] pending
/// ";
/// let _ = make_tasks(tasks_md);
/// ```
fn make_tasks(tasks_md: &str) -> ito_domain::tasks::TasksParseResult {
    let full = format!("## Wave 1\n- **Depends On**: None\n\n{tasks_md}");
    parse_tasks_tracking_file(&full)
}

/// Constructs an enhanced task markdown block for a task with the given id, status, and optional requirement IDs.
///
/// The returned string is a single task-level markdown block containing:
/// - a heading "Task {id}"
/// - a dependencies line
/// - an optional "Requirements" line when `reqs` is non-empty
/// - an "Updated At" timestamp
/// - a status line with the provided `status`
///
/// # Examples
///
/// ```
/// let md = task_block("1.1", "pending", &["REQ-001", "REQ-002"]);
/// assert!(md.contains("### Task 1.1: Task 1.1"));
/// assert!(md.contains("- **Requirements**: REQ-001, REQ-002"));
/// assert!(md.contains("- **Status**: [ ] pending"));
/// ```
fn task_block(id: &str, status: &str, reqs: &[&str]) -> String {
    let req_line = if reqs.is_empty() {
        String::new()
    } else {
        format!("- **Requirements**: {}\n", reqs.join(", "))
    };
    format!(
        "### Task {id}: Task {id}\n- **Dependencies**: None\n{req_line}- **Updated At**: 2026-01-01\n- **Status**: [ ] {status}\n"
    )
}

// ---------------------------------------------------------------------------
// Unavailable cases
// ---------------------------------------------------------------------------

/// Verifies that when all declared requirements lack IDs, traceability is reported as unavailable.
///
/// Constructs requirement entries that have titles but no IDs and a single pending task,
/// then asserts that `compute_traceability` yields `TraceStatus::Unavailable`.
///
/// # Examples
///
/// ```rust
/// let reqs = vec![req("REQ A", None), req("REQ B", None)];
/// let tasks = make_tasks(&task_block("1.1", "pending", &[]));
/// let result = compute_traceability(&reqs, &tasks);
/// assert!(matches!(result.status, TraceStatus::Unavailable { .. }));
/// ```
#[test]
fn no_requirement_ids_gives_unavailable() {
    let reqs = vec![req("REQ A", None), req("REQ B", None)];
    let tasks = make_tasks(&task_block("1.1", "pending", &[]));
    let result = compute_traceability(&reqs, &tasks);
    assert!(
        matches!(result.status, TraceStatus::Unavailable { .. }),
        "expected Unavailable, got {:?}",
        result.status
    );
}

#[test]
fn empty_requirements_list_gives_unavailable() {
    let tasks = make_tasks(&task_block("1.1", "pending", &[]));
    let result = compute_traceability(&[], &tasks);
    assert!(
        matches!(result.status, TraceStatus::Unavailable { .. }),
        "expected Unavailable for empty requirements"
    );
}

#[test]
fn checkbox_format_gives_unavailable() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks = parse_tasks_tracking_file("- [ ] Task one\n");
    let result = compute_traceability(&reqs, &tasks);
    assert!(
        matches!(result.status, TraceStatus::Unavailable { .. }),
        "expected Unavailable for checkbox format"
    );
    if let TraceStatus::Unavailable { reason } = &result.status {
        assert!(
            reason.contains("checkbox"),
            "reason should mention checkbox format, got: {reason}"
        );
    }
}

// ---------------------------------------------------------------------------
// Invalid cases
// ---------------------------------------------------------------------------

#[test]
fn partial_ids_gives_invalid_with_missing_titles() {
    let reqs = vec![
        req("REQ A", Some("REQ-001")),
        req("REQ B", None),
        req("REQ C", None),
    ];
    let tasks = make_tasks(&task_block("1.1", "pending", &[]));
    let result = compute_traceability(&reqs, &tasks);
    if let TraceStatus::Invalid { missing_ids } = &result.status {
        assert!(missing_ids.contains(&"REQ B".to_string()));
        assert!(missing_ids.contains(&"REQ C".to_string()));
        assert!(!missing_ids.contains(&"REQ A".to_string()));
    } else {
        panic!("expected Invalid, got {:?}", result.status);
    }
}

// ---------------------------------------------------------------------------
// Ready cases
// ---------------------------------------------------------------------------

#[test]
fn all_requirements_covered_by_tasks() {
    let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", Some("REQ-002"))];
    let tasks_md = format!(
        "{}{}",
        task_block("1.1", "pending", &["REQ-001"]),
        task_block("1.2", "pending", &["REQ-002"]),
    );
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(result.declared_requirements.len(), 2);
    assert_eq!(result.covered_requirements.len(), 2);
    assert!(result.uncovered_requirements.is_empty());
    assert!(result.unresolved_references.is_empty());
}

/// Verifies that a declared requirement not referenced by any non-shelved task appears in the uncovered requirements list.
///
/// # Examples
///
/// ```
/// let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", Some("REQ-002"))];
/// let tasks_md = task_block("1.1", "pending", &["REQ-001"]);
/// let tasks = make_tasks(&tasks_md);
/// let result = compute_traceability(&reqs, &tasks);
/// assert_eq!(result.status, TraceStatus::Ready);
/// assert_eq!(result.covered_requirements.len(), 1);
/// assert_eq!(result.uncovered_requirements, vec!["REQ-002".to_string()]);
/// ```
#[test]
fn uncovered_requirement_appears_in_uncovered_list() {
    let reqs = vec![req("REQ A", Some("REQ-001")), req("REQ B", Some("REQ-002"))];
    let tasks_md = task_block("1.1", "pending", &["REQ-001"]);
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(result.covered_requirements.len(), 1);
    assert_eq!(result.uncovered_requirements, vec!["REQ-002".to_string()]);
}

/// Verifies that a task referencing a non-declared requirement is reported as an unresolved reference.
///
/// The test constructs one declared requirement (`REQ-001`) and a task (`1.1`) that also
/// references an unknown requirement ID (`REQ-GHOST`). `compute_traceability` should mark the
/// overall traceability as ready and include a single unresolved reference pointing to task `1.1`
/// and requirement `REQ-GHOST`.
///
/// # Examples
///
/// ```
/// let reqs = vec![req("REQ A", Some("REQ-001"))];
/// let tasks_md = task_block("1.1", "pending", &["REQ-001", "REQ-GHOST"]);
/// let tasks = make_tasks(&tasks_md);
/// let result = compute_traceability(&reqs, &tasks);
/// assert_eq!(result.status, TraceStatus::Ready);
/// assert_eq!(result.unresolved_references.len(), 1);
/// assert_eq!(result.unresolved_references[0].task_id, "1.1");
/// assert_eq!(result.unresolved_references[0].requirement_id, "REQ-GHOST");
/// ```
#[test]
fn unresolved_task_reference_is_reported() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks_md = task_block("1.1", "pending", &["REQ-001", "REQ-GHOST"]);
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(result.unresolved_references.len(), 1);
    assert_eq!(result.unresolved_references[0].task_id, "1.1");
    assert_eq!(result.unresolved_references[0].requirement_id, "REQ-GHOST");
}

/// Verifies that a task with status "shelved" does not count as covering a requirement.
///
/// The test asserts that when a task referencing a declared requirement is marked
/// as "shelved", the requirement remains uncovered and the traceability result
/// reports `Ready` with an empty `covered_requirements` and the requirement listed
/// in `uncovered_requirements`.
///
/// # Examples
///
/// ```
/// // Constructs one requirement REQ-001 and a single shelved task that references it,
/// // then computes traceability and checks that the requirement is not covered.
/// let reqs = vec![req("REQ A", Some("REQ-001"))];
/// let tasks_md = task_block("1.1", "shelved", &["REQ-001"]);
/// let tasks = make_tasks(&tasks_md);
/// let result = compute_traceability(&reqs, &tasks);
/// assert_eq!(result.status, TraceStatus::Ready);
/// assert!(result.covered_requirements.is_empty());
/// assert_eq!(result.uncovered_requirements, vec!["REQ-001".to_string()]);
/// ```
#[test]
fn shelved_task_does_not_count_as_coverage() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks_md = task_block("1.1", "shelved", &["REQ-001"]);
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert!(
        result.covered_requirements.is_empty(),
        "shelved task must not count as coverage"
    );
    assert_eq!(result.uncovered_requirements, vec!["REQ-001".to_string()]);
}

#[test]
fn duplicate_requirement_ids_flagged_in_diagnostics() {
    let reqs = vec![
        req("REQ A", Some("REQ-001")),
        req("REQ A duplicate", Some("REQ-001")),
    ];
    let tasks = make_tasks(&task_block("1.1", "pending", &[]));
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert!(
        result.diagnostics.iter().any(|d| d.contains("REQ-001")),
        "expected duplicate diagnostic for REQ-001, got: {:?}",
        result.diagnostics
    );
    // After dedup, declared_requirements should have REQ-001 only once.
    assert_eq!(
        result
            .declared_requirements
            .iter()
            .filter(|id| *id == "REQ-001")
            .count(),
        1
    );
}

#[test]
fn multiple_tasks_can_cover_same_requirement() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks_md = format!(
        "{}{}",
        task_block("1.1", "pending", &["REQ-001"]),
        task_block("1.2", "complete", &["REQ-001"]),
    );
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(result.covered_requirements.len(), 1);
    assert_eq!(result.covered_requirements[0].covering_tasks.len(), 2);
}

#[test]
fn in_progress_task_counts_as_coverage() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks_md = task_block("1.1", "in-progress", &["REQ-001"]);
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(result.covered_requirements.len(), 1);
    assert!(result.uncovered_requirements.is_empty());
}

#[test]
fn complete_task_counts_as_coverage() {
    let reqs = vec![req("REQ A", Some("REQ-001"))];
    let tasks_md = task_block("1.1", "complete", &["REQ-001"]);
    let tasks = make_tasks(&tasks_md);
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(result.status, TraceStatus::Ready);
    assert_eq!(
        result.covered_requirements,
        vec![CoveredRequirement {
            requirement_id: "REQ-001".to_string(),
            covering_tasks: vec!["1.1".to_string()],
        }]
    );
}

#[test]
fn declared_requirements_are_sorted_and_deduplicated() {
    let reqs = vec![
        req("REQ C", Some("REQ-003")),
        req("REQ A", Some("REQ-001")),
        req("REQ B", Some("REQ-002")),
        req("REQ A dup", Some("REQ-001")),
    ];
    let tasks = make_tasks(&task_block("1.1", "pending", &[]));
    let result = compute_traceability(&reqs, &tasks);
    assert_eq!(
        result.declared_requirements,
        vec![
            "REQ-001".to_string(),
            "REQ-002".to_string(),
            "REQ-003".to_string()
        ]
    );
}
