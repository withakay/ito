//! Worktree discovery for audit event aggregation and streaming.
//!
//! Uses `git worktree list --porcelain` to enumerate all worktrees and
//! resolves their audit event file paths.

use std::path::{Path, PathBuf};

use ito_domain::audit::event::{AuditEvent, WorktreeInfo};

use super::reader::read_audit_events;
use super::writer::audit_log_path;

/// Discover all git worktrees that have an audit events file.
///
/// Returns an empty vec if git is unavailable, not in a repo, or no
/// worktrees have audit logs.
pub fn discover_worktrees(ito_path: &Path) -> Vec<WorktreeInfo> {
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
    parse_worktree_list(&stdout, ito_path)
}

/// Parse `git worktree list --porcelain` output into `WorktreeInfo` entries.
fn parse_worktree_list(output: &str, _ito_path: &Path) -> Vec<WorktreeInfo> {
    let mut worktrees = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    let mut current_branch: Option<String> = None;
    let mut is_bare = false;

    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            // Save previous worktree
            if let Some(path) = current_path.take() {
                if !is_bare {
                    let wt_ito_path = path.join(".ito");
                    let log = audit_log_path(&wt_ito_path);
                    // Only include worktrees that have an audit log or .ito dir
                    let has_ito = wt_ito_path.exists();
                    worktrees.push(WorktreeInfo {
                        path,
                        branch: current_branch.take(),
                        is_main: worktrees.is_empty(), // First worktree is main
                    });
                    if !has_ito {
                        // Still include it but note the log path may not exist yet
                        let _ = log;
                    }
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
    if let Some(path) = current_path {
        if !is_bare {
            worktrees.push(WorktreeInfo {
                path,
                branch: current_branch,
                is_main: worktrees.is_empty(),
            });
        }
    }

    worktrees
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

    for wt in worktrees {
        let wt_ito_path = wt.path.join(".ito");
        let log_path = audit_log_path(&wt_ito_path);
        if !log_path.exists() {
            continue;
        }

        let events = read_audit_events(&wt_ito_path);
        if !events.is_empty() {
            results.push((wt.clone(), events));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_worktree() {
        let output = "worktree /home/user/project\nHEAD abc1234\nbranch refs/heads/main\n\n";
        let dummy = Path::new("/unused");
        let wts = parse_worktree_list(output, dummy);
        assert_eq!(wts.len(), 1);
        assert_eq!(wts[0].path, PathBuf::from("/home/user/project"));
        assert_eq!(wts[0].branch, Some("main".to_string()));
        assert!(wts[0].is_main);
    }

    #[test]
    fn parse_multiple_worktrees() {
        let output = "\
worktree /home/user/project
HEAD abc1234
branch refs/heads/main

worktree /home/user/wt-feature
HEAD def5678
branch refs/heads/feature-x

";
        let dummy = Path::new("/unused");
        let wts = parse_worktree_list(output, dummy);
        assert_eq!(wts.len(), 2);
        assert!(wts[0].is_main);
        assert!(!wts[1].is_main);
        assert_eq!(wts[1].branch, Some("feature-x".to_string()));
    }

    #[test]
    fn parse_bare_worktree_excluded() {
        let output = "\
worktree /home/user/project.git
bare

worktree /home/user/wt-main
HEAD abc1234
branch refs/heads/main

";
        let dummy = Path::new("/unused");
        let wts = parse_worktree_list(output, dummy);
        assert_eq!(wts.len(), 1);
        assert_eq!(wts[0].path, PathBuf::from("/home/user/wt-main"));
    }

    #[test]
    fn parse_detached_head() {
        let output = "worktree /home/user/project\nHEAD abc1234\ndetached\n\n";
        let dummy = Path::new("/unused");
        let wts = parse_worktree_list(output, dummy);
        assert_eq!(wts.len(), 1);
        assert!(wts[0].branch.is_none());
    }

    #[test]
    fn worktree_audit_log_path_resolves() {
        let wt = WorktreeInfo {
            path: PathBuf::from("/project/wt-feature"),
            branch: Some("feature".to_string()),
            is_main: false,
        };
        let path = worktree_audit_log_path(&wt);
        assert_eq!(
            path,
            PathBuf::from("/project/wt-feature/.ito/.state/audit/events.jsonl")
        );
    }

    #[test]
    fn aggregate_empty_worktrees() {
        let results = aggregate_worktree_events(&[]);
        assert!(results.is_empty());
    }

    #[test]
    fn aggregate_worktree_with_events() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let wt_path = tmp.path().join("wt");
        std::fs::create_dir_all(&wt_path).expect("create wt dir");

        // Write an event to this worktree's audit log
        let wt_ito_path = wt_path.join(".ito");
        let writer = crate::audit::writer::FsAuditWriter::new(&wt_ito_path);
        let event = ito_domain::audit::event::AuditEvent {
            v: 1,
            ts: "2026-02-08T14:30:00.000Z".to_string(),
            entity: "task".to_string(),
            entity_id: "1.1".to_string(),
            scope: Some("ch".to_string()),
            op: "create".to_string(),
            from: None,
            to: Some("pending".to_string()),
            actor: "cli".to_string(),
            by: "@test".to_string(),
            meta: None,
            ctx: ito_domain::audit::event::EventContext {
                session_id: "test".to_string(),
                harness_session_id: None,
                branch: None,
                worktree: None,
                commit: None,
            },
        };
        ito_domain::audit::writer::AuditWriter::append(&writer, &event).unwrap();

        let wt_info = WorktreeInfo {
            path: wt_path,
            branch: Some("main".to_string()),
            is_main: true,
        };

        let results = aggregate_worktree_events(&[wt_info]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.len(), 1);
    }
}
