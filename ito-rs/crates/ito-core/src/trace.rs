//! Core orchestration for the `ito trace` command.
//!
//! Loads a change (active or archived), extracts requirement IDs from its delta
//! specs, and delegates to [`ito_domain::traceability::compute_traceability`] to
//! produce a structured coverage report.

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use crate::show::{parse_change_show_json, read_change_delta_spec_files};
use ito_domain::changes::ChangeRepository;
use ito_domain::traceability::{TraceStatus, compute_traceability};
use serde::Serialize;

/// One covered requirement entry in a [`TraceOutput`].
#[derive(Debug, Clone, Serialize)]
pub struct CoveredEntry {
    /// The stable requirement ID.
    pub requirement_id: String,
    /// IDs of tasks that reference this requirement.
    pub covering_tasks: Vec<String>,
}

/// One unresolved task reference entry in a [`TraceOutput`].
#[derive(Debug, Clone, Serialize)]
pub struct UnresolvedEntry {
    /// The task that contains the dangling reference.
    pub task_id: String,
    /// The requirement ID that could not be resolved.
    pub requirement_id: String,
}

/// Structured output for the `ito trace` command.
#[derive(Debug, Clone, Serialize)]
pub struct TraceOutput {
    /// Change identifier.
    pub change_id: String,
    /// Lifecycle state: `"active"` or `"archived"`.
    pub lifecycle: String,
    /// Traceability status: `"ready"`, `"invalid"`, or `"unavailable"`.
    pub status: String,
    /// Human-readable reason (present for `invalid` and `unavailable`).
    pub reason: Option<String>,
    /// All requirement IDs declared in delta specs (deduplicated, sorted).
    pub declared_requirements: Vec<String>,
    /// Requirements covered by at least one active task.
    pub covered: Vec<CoveredEntry>,
    /// Requirement IDs not covered by any active task.
    pub uncovered: Vec<String>,
    /// Task references to unknown requirement IDs.
    pub unresolved: Vec<UnresolvedEntry>,
    /// Informational diagnostics (e.g. duplicate IDs).
    pub diagnostics: Vec<String>,
}

/// Compute requirement traceability for a change.
///
/// Loads the change, reads its delta specification files, extracts requirement
/// identifiers from the parsed deltas, determines the change lifecycle
/// ("archived" when the change path contains `/archive/` or `/archived/`,
/// otherwise "active"), and returns a structured `TraceOutput` describing
/// declared, covered, uncovered, and unresolved requirements along with
/// diagnostics and a status/reason.
///
/// # Examples
///
/// ```ignore
/// // Given a ChangeRepository `repo` and a change id:
/// let output = compute_trace_output(&repo, "CH-123").unwrap();
/// println!("{}", output.status);
/// ```
pub fn compute_trace_output(
    change_repo: &(impl ChangeRepository + ?Sized),
    change_id: &str,
) -> CoreResult<TraceOutput> {
    let change = change_repo.get(change_id).into_core()?;

    // Determine lifecycle from the change path.
    let lifecycle = {
        let path_str = change.path.to_string_lossy();
        if path_str.contains("/archive/") || path_str.contains("/archived/") {
            "archived".to_string()
        } else {
            "active".to_string()
        }
    };

    let delta_files = read_change_delta_spec_files(change_repo, change_id)?;
    if delta_files.is_empty() {
        return Err(CoreError::not_found(format!(
            "No delta spec files found for change '{change_id}'"
        )));
    }

    let show = parse_change_show_json(change_id, &delta_files);

    // Collect (title, id) pairs from all delta requirements.
    let mut delta_requirements: Vec<(String, Option<String>)> = Vec::new();
    for d in &show.deltas {
        for req in &d.requirements {
            delta_requirements.push((req.text.clone(), req.requirement_id.clone()));
        }
    }

    let trace_result = compute_traceability(&delta_requirements, &change.tasks);

    let (status, reason) = match &trace_result.status {
        TraceStatus::Ready => ("ready".to_string(), None),
        TraceStatus::Invalid { missing_ids } => {
            let reason = format!("Requirements missing IDs: {}", missing_ids.join(", "));
            ("invalid".to_string(), Some(reason))
        }
        TraceStatus::Unavailable { reason } => ("unavailable".to_string(), Some(reason.clone())),
    };

    let mut covered = Vec::new();
    for c in &trace_result.covered_requirements {
        covered.push(CoveredEntry {
            requirement_id: c.requirement_id.clone(),
            covering_tasks: c.covering_tasks.clone(),
        });
    }

    let mut unresolved = Vec::new();
    for u in &trace_result.unresolved_references {
        unresolved.push(UnresolvedEntry {
            task_id: u.task_id.clone(),
            requirement_id: u.requirement_id.clone(),
        });
    }

    Ok(TraceOutput {
        change_id: change_id.to_string(),
        lifecycle,
        status,
        reason,
        declared_requirements: trace_result.declared_requirements,
        covered,
        uncovered: trace_result.uncovered_requirements,
        unresolved,
        diagnostics: trace_result.diagnostics,
    })
}
