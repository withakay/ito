//! Read-only validation of whether the current checkout matches an expected change worktree.

use crate::repo_paths::{ResolvedWorktreePaths, WorktreeFeature, WorktreeSelector};
use std::path::{Component, Path, PathBuf};

/// Machine-readable worktree validation result for humans and hook callers.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorktreeValidation {
    /// Validation status.
    pub status: WorktreeValidationStatus,
    /// Change ID that was validated.
    pub change_id: String,
    /// Current checkout/worktree path that was inspected.
    pub current_path: PathBuf,
    /// Expected dedicated change worktree path when worktrees are enabled.
    pub expected_path: Option<PathBuf>,
    /// Human-readable explanation of the outcome.
    pub message: String,
}

/// Distinct outcomes for worktree validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeValidationStatus {
    /// Current checkout is acceptable for the requested change.
    Ok,
    /// Worktree validation is disabled by configuration.
    Disabled,
    /// Current checkout is the main/control worktree and must not be used.
    MainCheckout,
    /// Current checkout is outside main/control, but path/branch does not match the change.
    Mismatch,
}

/// Validate that the current checkout is an acceptable worktree for `change_id`.
///
/// This is the read-only validator intended for CLI commands and harness/plugin
/// hooks that need to distinguish:
/// - hard failures on the main/control checkout,
/// - advisory mismatches outside the main/control checkout, and
/// - disabled validation when worktrees are off in config.
pub fn validate_change_worktree(
    change_id: &str,
    current_path: &Path,
    worktree_paths: &ResolvedWorktreePaths,
    current_branch: Option<&str>,
) -> WorktreeValidation {
    let expected_path =
        worktree_paths.path_for_selector(&WorktreeSelector::Change(change_id.to_string()));

    let current_path = current_path.to_path_buf();

    let WorktreeFeature::Enabled = worktree_paths.feature else {
        return WorktreeValidation {
            status: WorktreeValidationStatus::Disabled,
            change_id: change_id.to_string(),
            current_path,
            expected_path,
            message:
                "Worktree validation is disabled by configuration (`worktrees.enabled=false`)."
                    .to_string(),
        };
    };

    if is_main_checkout(
        &current_path,
        worktree_paths.main_worktree_root.as_deref(),
        worktree_paths.worktrees_root.as_deref(),
    ) {
        let message = main_checkout_message(change_id, &current_path, expected_path.as_deref());
        return WorktreeValidation {
            status: WorktreeValidationStatus::MainCheckout,
            change_id: change_id.to_string(),
            current_path,
            expected_path,
            message,
        };
    }

    if path_or_branch_matches_change_id(&current_path, current_branch, change_id) {
        return WorktreeValidation {
            status: WorktreeValidationStatus::Ok,
            change_id: change_id.to_string(),
            current_path,
            expected_path,
            message: format!("Current worktree is valid for change '{change_id}'."),
        };
    }

    let message = mismatch_message(
        change_id,
        &current_path,
        current_branch,
        expected_path.as_deref(),
    );

    WorktreeValidation {
        status: WorktreeValidationStatus::Mismatch,
        change_id: change_id.to_string(),
        current_path,
        expected_path,
        message,
    }
}

fn is_main_checkout(
    current_path: &Path,
    main_worktree_root: Option<&Path>,
    worktrees_root: Option<&Path>,
) -> bool {
    let Some(main_root) = main_worktree_root else {
        return false;
    };

    if current_path == main_root {
        return true;
    }

    current_path.starts_with(main_root)
        && worktrees_root
            .map(|root| !current_path.starts_with(root))
            .unwrap_or(true)
}

fn path_or_branch_matches_change_id(
    current_path: &Path,
    current_branch: Option<&str>,
    change_id: &str,
) -> bool {
    current_branch
        .map(|branch| branch_starts_with_change_id(branch, change_id))
        .unwrap_or(false)
        || current_path
            .components()
            .filter_map(|component| match component {
                Component::Normal(segment) => segment.to_str(),
                Component::Prefix(_)
                | Component::RootDir
                | Component::CurDir
                | Component::ParentDir => None,
            })
            .any(|segment| segment_starts_with_change_id(segment, change_id))
}

fn branch_starts_with_change_id(branch: &str, change_id: &str) -> bool {
    segment_starts_with_change_id(branch, change_id)
}

fn segment_starts_with_change_id(segment: &str, change_id: &str) -> bool {
    segment == change_id
        || segment.starts_with(&format!("{change_id}-"))
        || segment.starts_with(&format!("{change_id}_"))
}

fn main_checkout_message(
    change_id: &str,
    current_path: &Path,
    expected_path: Option<&Path>,
) -> String {
    match expected_path {
        Some(expected_path) => format!(
            "Current checkout '{}' is the main/control worktree. Change work for '{change_id}' must run from a dedicated change worktree, such as '{}'.",
            current_path.display(),
            expected_path.display()
        ),
        None => format!(
            "Current checkout '{}' is the main/control worktree. Change work for '{change_id}' must run from a dedicated change worktree.",
            current_path.display()
        ),
    }
}

fn mismatch_message(
    change_id: &str,
    current_path: &Path,
    current_branch: Option<&str>,
    expected_path: Option<&Path>,
) -> String {
    let branch_note = current_branch
        .filter(|branch| !branch.trim().is_empty())
        .map(|branch| format!(" Current branch: '{branch}'."))
        .unwrap_or_default();

    let expected_note = expected_path
        .map(|expected| format!(" Expected worktree: '{}'.", expected.display()))
        .unwrap_or_default();

    format!(
        "Current checkout '{}' does not match change '{change_id}'. The branch or worktree path should include the full change ID.{branch_note}{expected_note}",
        current_path.display()
    )
}

#[cfg(test)]
#[path = "worktree_validate_tests.rs"]
mod tests;
