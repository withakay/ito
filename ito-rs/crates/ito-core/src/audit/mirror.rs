//! Best-effort synchronization of the local audit log to a dedicated remote branch.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

use super::writer::audit_log_path;

/// Failure details from an audit mirror sync attempt.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct AuditMirrorError {
    message: String,
}

impl AuditMirrorError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Attempt to sync the local audit log to `origin/<branch>`.
///
/// This function returns an error when a sync attempt fails; callers that
/// require best-effort semantics should log/print the error and continue.
pub fn sync_audit_mirror(
    repo_root: &Path,
    ito_path: &Path,
    branch: &str,
) -> Result<(), AuditMirrorError> {
    let runner = SystemProcessRunner::default();
    sync_audit_mirror_with_runner(&runner, repo_root, ito_path, branch)
}

pub(crate) fn sync_audit_mirror_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    ito_path: &Path,
    branch: &str,
) -> Result<(), AuditMirrorError> {
    if !is_git_worktree(runner, repo_root) {
        return Ok(());
    }

    let local_log = audit_log_path(ito_path);
    if !local_log.exists() {
        return Ok(());
    }

    let worktree_path = unique_temp_worktree_path();
    add_detached_worktree(runner, repo_root, &worktree_path)?;
    let cleanup = WorktreeCleanup {
        repo_root: repo_root.to_path_buf(),
        worktree_path: worktree_path.clone(),
    };

    let result = (|| {
        let fetched = fetch_branch(runner, repo_root, branch);
        match fetched {
            Ok(()) => checkout_detached_remote_branch(runner, &worktree_path, branch)?,
            Err(FetchError::RemoteMissing) => checkout_orphan_branch(runner, &worktree_path)?,
            Err(FetchError::Other(msg)) => return Err(AuditMirrorError::new(msg)),
        }

        write_merged_audit_log(
            &local_log,
            &worktree_path.join(".ito/.state/audit/events.jsonl"),
        )?;
        stage_audit_log(runner, &worktree_path)?;

        if !has_staged_changes(runner, &worktree_path)? {
            return Ok(());
        }

        commit_audit_log(runner, &worktree_path)?;
        if push_branch(runner, &worktree_path, branch)? {
            return Ok(());
        }

        // Retry once on non-fast-forward by refetching and re-merging.
        let fetched = fetch_branch(runner, repo_root, branch);
        match fetched {
            Ok(()) => checkout_detached_remote_branch(runner, &worktree_path, branch)?,
            Err(FetchError::RemoteMissing) => checkout_orphan_branch(runner, &worktree_path)?,
            Err(FetchError::Other(msg)) => return Err(AuditMirrorError::new(msg)),
        }
        write_merged_audit_log(
            &local_log,
            &worktree_path.join(".ito/.state/audit/events.jsonl"),
        )?;
        stage_audit_log(runner, &worktree_path)?;
        if has_staged_changes(runner, &worktree_path)? {
            commit_audit_log(runner, &worktree_path)?;
        }
        if push_branch(runner, &worktree_path, branch)? {
            return Ok(());
        }

        Err(AuditMirrorError::new(format!(
            "audit mirror push to '{branch}' failed due to a remote conflict; try 'git fetch origin {branch}' and re-run, or disable mirroring with 'ito config set audit.mirror.enabled false'"
        )))
    })();

    let cleanup_err = cleanup.cleanup_with_runner(runner);
    if let Err(err) = cleanup_err {
        eprintln!(
            "Warning: failed to remove temporary audit mirror worktree '{}': {}",
            cleanup.worktree_path.display(),
            err
        );
    }
    result
}

fn write_merged_audit_log(local_log: &Path, target_log: &Path) -> Result<(), AuditMirrorError> {
    let local = fs::read_to_string(local_log)
        .map_err(|e| AuditMirrorError::new(format!("failed to read local audit log: {e}")))?;
    let remote = fs::read_to_string(target_log).unwrap_or_default();

    let merged = merge_jsonl_lines(&remote, &local);

    if let Some(parent) = target_log.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            AuditMirrorError::new(format!("failed to create audit mirror dir: {e}"))
        })?;
    }
    fs::write(target_log, merged)
        .map_err(|e| AuditMirrorError::new(format!("failed to write audit mirror log: {e}")))?;
    Ok(())
}

fn merge_jsonl_lines(remote: &str, local: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for line in remote.lines() {
        if line.trim().is_empty() {
            continue;
        }
        out.push(line.to_string());
        seen.insert(line.to_string());
    }

    for line in local.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if seen.contains(line) {
            continue;
        }
        out.push(line.to_string());
        seen.insert(line.to_string());
    }

    if out.is_empty() {
        return String::new();
    }
    // Ensure trailing newline so subsequent appends are line-oriented.
    format!("{}\n", out.join("\n"))
}

fn add_detached_worktree(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    worktree_path: &Path,
) -> Result<(), AuditMirrorError> {
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args([
                    "worktree",
                    "add",
                    "--detach",
                    worktree_path.to_string_lossy().as_ref(),
                ])
                .current_dir(repo_root),
        )
        .map_err(|e| AuditMirrorError::new(format!("git worktree add failed: {e}")))?;
    if out.success {
        return Ok(());
    }
    Err(AuditMirrorError::new(format!(
        "git worktree add failed ({})",
        render_output(&out)
    )))
}

#[derive(Debug)]
enum FetchError {
    RemoteMissing,
    Other(String),
}

fn fetch_branch(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<(), FetchError> {
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["fetch", "origin", branch])
                .current_dir(repo_root),
        )
        .map_err(|e| FetchError::Other(format!("git fetch origin {branch} failed to run: {e}")))?;
    if out.success {
        return Ok(());
    }
    let detail = render_output(&out);
    if detail.contains("couldn't find remote ref") {
        return Err(FetchError::RemoteMissing);
    }
    Err(FetchError::Other(format!(
        "git fetch origin {branch} failed ({detail})"
    )))
}

fn checkout_detached_remote_branch(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    branch: &str,
) -> Result<(), AuditMirrorError> {
    let target = format!("origin/{branch}");
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["checkout", "--detach", &target])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git checkout failed: {e}")))?;
    if out.success {
        return Ok(());
    }
    Err(AuditMirrorError::new(format!(
        "failed to checkout audit mirror branch '{branch}' ({})",
        render_output(&out)
    )))
}

fn checkout_orphan_branch(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<(), AuditMirrorError> {
    let orphan = unique_orphan_branch_name();
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["checkout", "--orphan", orphan.as_str()])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git checkout --orphan failed: {e}")))?;
    if !out.success {
        return Err(AuditMirrorError::new(format!(
            "failed to create orphan audit mirror worktree ({})",
            render_output(&out)
        )));
    }

    // Remove tracked files from the index to keep the mirror branch focused.
    let _ = runner.run(
        &ProcessRequest::new("git")
            .args(["rm", "-rf", "."]) // best-effort; may fail on empty trees
            .current_dir(worktree_path),
    );
    Ok(())
}

fn stage_audit_log(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<(), AuditMirrorError> {
    let relative = ".ito/.state/audit/events.jsonl";
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["add", "-f", relative])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git add failed: {e}")))?;
    if out.success {
        return Ok(());
    }
    Err(AuditMirrorError::new(format!(
        "failed to stage audit mirror log ({})",
        render_output(&out)
    )))
}

fn has_staged_changes(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<bool, AuditMirrorError> {
    let relative = ".ito/.state/audit/events.jsonl";
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["diff", "--cached", "--quiet", "--", relative])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git diff --cached failed: {e}")))?;
    if out.success {
        return Ok(false);
    }
    if out.exit_code == 1 {
        return Ok(true);
    }
    Err(AuditMirrorError::new(format!(
        "failed to inspect staged audit mirror changes ({})",
        render_output(&out)
    )))
}

fn commit_audit_log(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<(), AuditMirrorError> {
    let message = "chore(audit): mirror events";
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["commit", "-m", message])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git commit failed: {e}")))?;
    if out.success {
        return Ok(());
    }
    Err(AuditMirrorError::new(format!(
        "failed to commit audit mirror update ({})",
        render_output(&out)
    )))
}

/// Push `HEAD` to `origin/<branch>`.
///
/// Returns `Ok(true)` when push succeeded, `Ok(false)` when the push was rejected due to non-fast-forward.
fn push_branch(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    branch: &str,
) -> Result<bool, AuditMirrorError> {
    let refspec = format!("HEAD:refs/heads/{branch}");
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["push", "origin", &refspec])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git push failed to run: {e}")))?;
    if out.success {
        return Ok(true);
    }

    let detail = render_output(&out);
    if detail.contains("non-fast-forward") {
        return Ok(false);
    }

    Err(AuditMirrorError::new(format!(
        "audit mirror push failed ({detail})"
    )))
}

fn is_git_worktree(runner: &dyn ProcessRunner, repo_root: &Path) -> bool {
    let out = runner.run(
        &ProcessRequest::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .current_dir(repo_root),
    );
    let Ok(out) = out else {
        return false;
    };
    out.success && out.stdout.trim() == "true"
}

fn render_output(out: &crate::process::ProcessOutput) -> String {
    let stdout = out.stdout.trim();
    let stderr = out.stderr.trim();

    if !stderr.is_empty() {
        return stderr.to_string();
    }
    if !stdout.is_empty() {
        return stdout.to_string();
    }
    "no command output".to_string()
}

fn unique_temp_worktree_path() -> PathBuf {
    let pid = std::process::id();
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };
    std::env::temp_dir().join(format!("ito-audit-mirror-{pid}-{nanos}"))
}

fn unique_orphan_branch_name() -> String {
    let pid = std::process::id();
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };
    format!("ito-audit-mirror-orphan-{pid}-{nanos}")
}

struct WorktreeCleanup {
    repo_root: PathBuf,
    worktree_path: PathBuf,
}

impl WorktreeCleanup {
    fn cleanup_with_runner(&self, runner: &dyn ProcessRunner) -> Result<(), String> {
        let out = runner.run(
            &ProcessRequest::new("git")
                .args([
                    "worktree",
                    "remove",
                    "--force",
                    self.worktree_path.to_string_lossy().as_ref(),
                ])
                .current_dir(&self.repo_root),
        );
        if let Err(err) = out {
            return Err(format!("git worktree remove failed: {err}"));
        }

        // Ensure the directory is gone even if git left it behind.
        let _ = fs::remove_dir_all(&self.worktree_path);
        Ok(())
    }
}

impl Drop for WorktreeCleanup {
    fn drop(&mut self) {
        // Best-effort panic-safety: if callers unwind before `git worktree remove`
        // runs, still remove the directory to avoid littering temp.
        let _ = fs::remove_dir_all(&self.worktree_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_jsonl_dedupes_and_appends_local_lines() {
        let remote = "{\"a\":1}\n{\"b\":2}\n";
        let local = "{\"b\":2}\n{\"c\":3}\n";
        let merged = merge_jsonl_lines(remote, local);
        assert_eq!(merged, "{\"a\":1}\n{\"b\":2}\n{\"c\":3}\n");
    }

    #[test]
    fn merge_jsonl_ignores_blank_lines() {
        let remote = "\n{\"a\":1}\n\n";
        let local = "\n\n";
        let merged = merge_jsonl_lines(remote, local);
        assert_eq!(merged, "{\"a\":1}\n");
    }
}
