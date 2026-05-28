//! Worktree discovery for audit event aggregation and streaming.
//!
//! Uses worktree listing commands to enumerate all worktrees and resolves
//! their routed audit stores.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use ito_domain::audit::event::{AuditEvent, WorktreeInfo};

use super::store::{audit_storage_location_key, default_audit_store};
use super::writer::audit_log_path;

/// Discover all git worktrees that have an audit events file.
///
/// Returns an empty vec if git is unavailable, not in a repo, or no
/// worktrees have audit logs.
pub fn discover_worktrees(_ito_path: &Path) -> Vec<WorktreeInfo> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };

    if !output.status.success() {
        return Vec::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_worktree_list(&stdout)
}

/// Parse `git worktree list --porcelain` output into `WorktreeInfo` entries.
fn parse_worktree_list(output: &str) -> Vec<WorktreeInfo> {
    let mut worktrees = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    let mut current_branch: Option<String> = None;
    let mut is_bare = false;

    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            // Save previous worktree
            if let Some(path) = current_path.take() {
                if !is_bare {
                    worktrees.push(WorktreeInfo {
                        path,
                        branch: current_branch.take(),
                        is_main: worktrees.is_empty(), // First worktree is main
                    });
                }
                current_branch = None;
                is_bare = false;
            }
            current_path = Some(PathBuf::from(path));
        } else if let Some(branch_ref) = line.strip_prefix("branch ") {
            // refs/heads/main -> main
            current_branch = branch_ref.strip_prefix("refs/heads/").map(String::from);
        } else if line == "bare" {
            is_bare = true;
        } else if line.is_empty() {
            // Block separator — flush current
            if let Some(path) = current_path.take() {
                if !is_bare {
                    worktrees.push(WorktreeInfo {
                        path,
                        branch: current_branch.take(),
                        is_main: worktrees.is_empty(),
                    });
                }
                current_branch = None;
                is_bare = false;
            }
        }
    }

    // Flush last entry
    if let Some(path) = current_path
        && !is_bare
    {
        worktrees.push(WorktreeInfo {
            path,
            branch: current_branch,
            is_main: worktrees.is_empty(),
        });
    }

    worktrees
}

/// Find the worktree path for a branch by parsing `git worktree list --porcelain`.
///
/// Returns the worktree root directory if a non-bare worktree exists whose
/// branch name matches `branch`. Bare worktrees are excluded.
///
/// Used by Ralph to resolve the effective working directory for a change
/// when worktree-based workflows are active.
pub fn find_worktree_for_branch(branch: &str) -> Option<PathBuf> {
    if let Some(path) = find_worktree_for_branch_with_worktrunk(branch) {
        return Some(path);
    }

    let output = std::process::Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    find_worktree_for_branch_in_output(&stdout, branch)
}

fn find_worktree_for_branch_with_worktrunk(branch: &str) -> Option<PathBuf> {
    let output = std::process::Command::new("wt")
        .args(["list", "--format=json"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    find_worktree_for_branch_in_worktrunk_json(&stdout, branch)
}

fn find_worktree_for_branch_in_worktrunk_json(output: &str, branch: &str) -> Option<PathBuf> {
    let value: serde_json::Value = serde_json::from_str(output).ok()?;
    let entries = value
        .as_array()
        .or_else(|| value.get("worktrees").and_then(serde_json::Value::as_array))?;

    entries.iter().find_map(|entry| {
        let is_bare = entry
            .get("bare")
            .or_else(|| entry.get("is_bare"))
            .or_else(|| entry.get("isBare"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if is_bare {
            return None;
        }

        let entry_branch = entry
            .get("branch")
            .or_else(|| entry.get("name"))
            .and_then(serde_json::Value::as_str)
            .and_then(normalize_branch_name)?;
        if entry_branch != branch {
            return None;
        }

        let path = entry
            .get("path")
            .or_else(|| entry.get("worktree"))
            .or_else(|| entry.get("root"))
            .and_then(serde_json::Value::as_str)?;
        Some(PathBuf::from(path))
    })
}

fn normalize_branch_name(branch: &str) -> Option<&str> {
    branch.strip_prefix("refs/heads/").or(Some(branch))
}

/// Parse porcelain worktree output and find the path for a given branch name.
///
/// This is the testable core of [`find_worktree_for_branch`].
fn find_worktree_for_branch_in_output(output: &str, branch: &str) -> Option<PathBuf> {
    let worktrees = parse_worktree_list(output);
    worktrees
        .into_iter()
        .find(|wt| wt.branch.as_deref() == Some(branch))
        .map(|wt| wt.path)
}

/// Get the audit log path for a worktree.
pub fn worktree_audit_log_path(worktree: &WorktreeInfo) -> PathBuf {
    audit_log_path(&worktree.path.join(".ito"))
}

/// Read and aggregate events from all worktrees.
///
/// Returns events grouped by worktree. Only worktrees with existing
/// event files are included.
pub fn aggregate_worktree_events(
    worktrees: &[WorktreeInfo],
) -> Vec<(WorktreeInfo, Vec<AuditEvent>)> {
    let mut results = Vec::new();
    let mut seen_locations = HashSet::new();

    for wt in worktrees {
        let wt_ito_path = wt.path.join(".ito");
        if !wt_ito_path.exists() {
            continue;
        }

        let store = default_audit_store(&wt_ito_path);
        let key = audit_storage_location_key(&store.location());
        if !seen_locations.insert(key) {
            continue;
        }

        let events = store.read_all();
        if !events.is_empty() {
            results.push((wt.clone(), events));
        }
    }

    results
}

#[cfg(test)]
#[path = "worktree_tests.rs"]
mod worktree_tests;
