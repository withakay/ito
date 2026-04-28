//! Best-effort synchronization of the local audit log to a dedicated remote branch.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Duration, Utc};
use ito_domain::audit::event::{AuditEvent, ops};

use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

use super::writer::audit_log_path;

const MAX_GIT_AUDIT_EVENTS: usize = 1000;
const MAX_GIT_AUDIT_AGE_DAYS: i64 = 30;
static TEMP_NAME_COUNTER: AtomicU64 = AtomicU64::new(0);

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
    let runner = SystemProcessRunner;
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

    let result = (|| -> Result<(), AuditMirrorError> {
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

pub(crate) fn append_jsonl_to_internal_branch(
    repo_root: &Path,
    branch: &str,
    jsonl: &str,
) -> Result<(), AuditMirrorError> {
    let runner = SystemProcessRunner;
    append_jsonl_to_internal_branch_with_runner(&runner, repo_root, branch, jsonl)
}

pub(crate) fn append_jsonl_to_internal_branch_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
    jsonl: &str,
) -> Result<(), AuditMirrorError> {
    if !is_git_worktree(runner, repo_root) {
        return Err(AuditMirrorError::new(
            "internal audit branch unavailable outside a git worktree",
        ));
    }

    let mut allow_retry = true;
    loop {
        match append_jsonl_to_internal_branch_attempt(runner, repo_root, branch, jsonl)? {
            AppendBranchResult::Appended => return Ok(()),
            AppendBranchResult::Conflict if allow_retry => {
                allow_retry = false;
            }
            AppendBranchResult::Conflict => {
                return Err(AuditMirrorError::new(format!(
                    "failed to update internal audit branch '{branch}' due to concurrent writes; retry the command"
                )));
            }
        }
    }
}

enum AppendBranchResult {
    Appended,
    Conflict,
}

fn append_jsonl_to_internal_branch_attempt(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
    jsonl: &str,
) -> Result<AppendBranchResult, AuditMirrorError> {
    let expected_old = current_branch_oid(runner, repo_root, branch)?;

    let worktree_path = unique_temp_worktree_path();
    add_detached_worktree(runner, repo_root, &worktree_path)?;
    let cleanup = WorktreeCleanup {
        repo_root: repo_root.to_path_buf(),
        worktree_path: worktree_path.clone(),
    };

    let result = (|| -> Result<AppendBranchResult, AuditMirrorError> {
        if expected_old.is_some() {
            checkout_detached_local_branch(runner, &worktree_path, branch)?;
        } else {
            checkout_orphan_branch(runner, &worktree_path)?;
        }

        write_merged_jsonl(&worktree_path.join(".ito/.state/audit/events.jsonl"), jsonl)?;
        stage_audit_log(runner, &worktree_path)?;

        if !has_staged_changes(runner, &worktree_path)? {
            return Ok(AppendBranchResult::Appended);
        }

        commit_internal_audit_log(runner, &worktree_path)?;
        match update_branch_ref(runner, &worktree_path, branch, expected_old.as_deref())? {
            UpdateRefResult::Updated => Ok(AppendBranchResult::Appended),
            UpdateRefResult::Conflict => Ok(AppendBranchResult::Conflict),
        }
    })();

    let cleanup_err = cleanup.cleanup_with_runner(runner);
    if let Err(err) = cleanup_err {
        eprintln!(
            "Warning: failed to remove temporary audit worktree '{}': {}",
            cleanup.worktree_path.display(),
            err
        );
    }
    result
}

fn current_branch_oid(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<Option<String>, AuditMirrorError> {
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["rev-parse", "--verify", &format!("refs/heads/{branch}")])
                .current_dir(repo_root),
        )
        .map_err(|e| AuditMirrorError::new(format!("git rev-parse failed: {e}")))?;
    if out.success {
        let oid = out.stdout.trim();
        return Ok((!oid.is_empty()).then(|| oid.to_string()));
    }
    let detail = render_output(&out).to_ascii_lowercase();
    if detail.contains("unknown revision") || detail.contains("needed a single revision") {
        return Ok(None);
    }
    Err(AuditMirrorError::new(format!(
        "failed to inspect internal audit branch '{branch}' ({})",
        render_output(&out)
    )))
}

pub(crate) fn read_internal_branch_log(
    repo_root: &Path,
    branch: &str,
) -> Result<InternalBranchLogRead, AuditMirrorError> {
    let runner = SystemProcessRunner;
    read_internal_branch_log_with_runner(&runner, repo_root, branch)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InternalBranchLogRead {
    BranchMissing,
    LogMissing,
    Contents(String),
}

pub(crate) fn read_internal_branch_log_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<InternalBranchLogRead, AuditMirrorError> {
    if !is_git_worktree(runner, repo_root) {
        return Err(AuditMirrorError::new(
            "internal audit branch unavailable outside a git worktree",
        ));
    }

    if !local_branch_exists(runner, repo_root, branch)? {
        return Ok(InternalBranchLogRead::BranchMissing);
    }

    let pathspec = format!("refs/heads/{branch}:.ito/.state/audit/events.jsonl");
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["show", &pathspec])
                .current_dir(repo_root),
        )
        .map_err(|e| AuditMirrorError::new(format!("git show failed: {e}")))?;
    if out.success {
        return Ok(InternalBranchLogRead::Contents(out.stdout));
    }

    let detail = render_output(&out).to_ascii_lowercase();
    if detail.contains("does not exist in")
        || detail.contains("path '.ito/.state/audit/events.jsonl' does not exist")
    {
        return Ok(InternalBranchLogRead::LogMissing);
    }

    Err(AuditMirrorError::new(format!(
        "failed to read internal audit branch '{branch}' ({})",
        render_output(&out)
    )))
}

fn write_merged_audit_log(local_log: &Path, target_log: &Path) -> Result<(), AuditMirrorError> {
    let local = fs::read_to_string(local_log)
        .map_err(|e| AuditMirrorError::new(format!("failed to read local audit log: {e}")))?;
    let remote = fs::read_to_string(target_log).unwrap_or_default();

    let merged = merge_jsonl_lines(&remote, &local);

    write_jsonl(target_log, &merged)
}

fn write_merged_jsonl(target_log: &Path, jsonl: &str) -> Result<(), AuditMirrorError> {
    let existing = fs::read_to_string(target_log).unwrap_or_default();
    let merged = merge_jsonl_lines(&existing, jsonl);

    write_jsonl(target_log, &merged)
}

fn write_jsonl(target_log: &Path, contents: &str) -> Result<(), AuditMirrorError> {
    if let Some(parent) = target_log.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            AuditMirrorError::new(format!("failed to create audit mirror dir: {e}"))
        })?;
    }
    fs::write(target_log, contents)
        .map_err(|e| AuditMirrorError::new(format!("failed to write audit mirror log: {e}")))?;
    Ok(())
}

fn merge_jsonl_lines(remote: &str, local: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut last_reconcile_key: Option<String> = None;

    for line in remote.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let reconcile_key = stable_reconcile_key(line);
        if reconcile_key.is_some() && reconcile_key == last_reconcile_key {
            increment_last_reconcile_count(&mut out);
            continue;
        }
        out.push(line.to_string());
        seen.insert(line.to_string());
        last_reconcile_key = reconcile_key;
    }

    for line in local.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let reconcile_key = stable_reconcile_key(line);
        if reconcile_key.is_some() && reconcile_key == last_reconcile_key {
            increment_last_reconcile_count(&mut out);
            continue;
        }
        if seen.contains(line) {
            continue;
        }
        out.push(line.to_string());
        seen.insert(line.to_string());
        last_reconcile_key = reconcile_key;
    }

    let out = truncate_git_audit_events(out);

    if out.is_empty() {
        return String::new();
    }
    // Ensure trailing newline so subsequent appends are line-oriented.
    format!("{}\n", out.join("\n"))
}

fn truncate_git_audit_events(lines: Vec<String>) -> Vec<String> {
    let mut events: Vec<(usize, DateTime<Utc>, String)> = Vec::new();
    for (index, line) in lines.into_iter().enumerate() {
        let Some(ts) = audit_event_timestamp(&line) else {
            continue;
        };
        events.push((index, ts, line));
    }
    let Some((_, newest, _)) = events.iter().max_by_key(|(_, ts, _)| *ts) else {
        return Vec::new();
    };
    let cutoff = *newest - Duration::days(MAX_GIT_AUDIT_AGE_DAYS);
    let mut events: Vec<(usize, DateTime<Utc>, String)> = events
        .into_iter()
        .filter(|(_, ts, _)| *ts >= cutoff)
        .collect();
    if events.len() > MAX_GIT_AUDIT_EVENTS {
        events.sort_by_key(|(index, ts, _)| (*ts, *index));
        events.drain(0..events.len() - MAX_GIT_AUDIT_EVENTS);
    }
    events.sort_by_key(|(index, _, _)| *index);
    events.into_iter().map(|(_, _, line)| line).collect()
}

fn audit_event_timestamp(line: &str) -> Option<DateTime<Utc>> {
    let Ok(event) = serde_json::from_str::<AuditEvent>(line) else {
        return None;
    };
    let Ok(ts) = DateTime::parse_from_rfc3339(&event.ts) else {
        return None;
    };
    Some(ts.with_timezone(&Utc))
}

fn increment_last_reconcile_count(out: &mut [String]) {
    let Some(last) = out.last_mut() else {
        return;
    };
    let Ok(mut event) = serde_json::from_str::<AuditEvent>(last) else {
        return;
    };
    if event.op != ops::RECONCILED {
        return;
    }
    event.count = event.count.saturating_add(1);
    let Ok(serialized) = serde_json::to_string(&event) else {
        return;
    };
    *last = serialized;
}

fn stable_reconcile_key(line: &str) -> Option<String> {
    let Ok(event) = serde_json::from_str::<AuditEvent>(line) else {
        return None;
    };
    if event.op != ops::RECONCILED {
        return None;
    }

    Some(format!(
        "{}\u{1f}{}\u{1f}{:?}\u{1f}{:?}\u{1f}{:?}\u{1f}{}\u{1f}{}",
        event.entity, event.entity_id, event.scope, event.from, event.to, event.actor, event.by
    ))
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

fn checkout_detached_local_branch(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    branch: &str,
) -> Result<(), AuditMirrorError> {
    let target = format!("refs/heads/{branch}");
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
        "failed to checkout internal audit branch '{branch}' ({})",
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

fn commit_internal_audit_log(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<(), AuditMirrorError> {
    let message = "chore(audit): update internal log";
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
        "failed to commit internal audit log ({})",
        render_output(&out)
    )))
}

enum UpdateRefResult {
    Updated,
    Conflict,
}

fn update_branch_ref(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    branch: &str,
    expected_old: Option<&str>,
) -> Result<UpdateRefResult, AuditMirrorError> {
    let target = format!("refs/heads/{branch}");
    let new_oid = branch_head_oid(runner, worktree_path)?;
    let expected = expected_old.unwrap_or("0000000000000000000000000000000000000000");
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["update-ref", &target, &new_oid, expected])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git update-ref failed: {e}")))?;
    if out.success {
        return Ok(UpdateRefResult::Updated);
    }
    let detail = render_output(&out);
    let lower = detail.to_ascii_lowercase();
    if lower.contains("cannot lock ref")
        || lower.contains("is at ")
        || lower.contains("reference already exists")
    {
        return Ok(UpdateRefResult::Conflict);
    }
    Err(AuditMirrorError::new(format!(
        "failed to update internal audit branch '{branch}' ({})",
        detail
    )))
}

fn branch_head_oid(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> Result<String, AuditMirrorError> {
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(worktree_path),
        )
        .map_err(|e| AuditMirrorError::new(format!("git rev-parse HEAD failed: {e}")))?;
    if out.success {
        let oid = out.stdout.trim();
        if !oid.is_empty() {
            return Ok(oid.to_string());
        }
    }
    Err(AuditMirrorError::new(format!(
        "failed to resolve internal audit commit ({})",
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

fn local_branch_exists(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<bool, AuditMirrorError> {
    let target = format!("refs/heads/{branch}");
    let out = runner
        .run(
            &ProcessRequest::new("git")
                .args(["show-ref", "--verify", "--quiet", &target])
                .current_dir(repo_root),
        )
        .map_err(|e| AuditMirrorError::new(format!("git show-ref failed: {e}")))?;
    if out.success {
        return Ok(true);
    }
    if out.exit_code == 1 {
        return Ok(false);
    }

    Err(AuditMirrorError::new(format!(
        "failed to inspect internal audit branch '{branch}' ({})",
        render_output(&out)
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
    let sequence = TEMP_NAME_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };
    std::env::temp_dir().join(format!("ito-audit-mirror-{pid}-{nanos}-{sequence}"))
}

fn unique_orphan_branch_name() -> String {
    let pid = std::process::id();
    let sequence = TEMP_NAME_COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };
    format!("ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}")
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
        let a = task_event_id("2026-04-26T11:28:00.000Z", "1", "create", "pending");
        let b = task_event_id("2026-04-26T11:28:01.000Z", "2", "create", "pending");
        let c = task_event_id("2026-04-26T11:28:02.000Z", "3", "create", "pending");
        let remote = format!("{a}\n{b}\n");
        let local = format!("{b}\n{c}\n");
        let merged = merge_jsonl_lines(&remote, &local);
        assert_eq!(merged, format!("{a}\n{b}\n{c}\n"));
    }

    #[test]
    fn merge_jsonl_ignores_blank_lines() {
        let event = task_event("2026-04-26T11:28:00.000Z", "create", "pending");
        let remote = format!("\n{event}\n\n");
        let local = "\n\n";
        let merged = merge_jsonl_lines(&remote, local);
        assert_eq!(merged, format!("{event}\n"));
    }

    #[test]
    fn merge_jsonl_aggregates_adjacent_equivalent_reconciled_events() {
        let remote = format!(
            "{}\n",
            reconcile_event("2026-04-26T11:28:01.873Z", "pending")
        );
        let local = format!(
            "{}\n",
            reconcile_event("2026-04-26T11:28:21.643Z", "pending")
        );

        let merged = merge_jsonl_lines(&remote, &local);
        let mut lines = merged.lines();
        let event: serde_json::Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        assert_eq!(event["count"], serde_json::json!(2));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn merge_jsonl_keeps_reconciled_events_after_different_event() {
        let remote = format!(
            "{}\n{}\n",
            reconcile_event("2026-04-26T11:28:01.873Z", "pending"),
            task_event("2026-04-26T11:28:10.000Z", "status_change", "complete")
        );
        let local = format!(
            "{}\n",
            reconcile_event("2026-04-26T11:28:21.643Z", "pending")
        );

        let merged = merge_jsonl_lines(&remote, &local);
        assert_eq!(merged, format!("{}{}", remote, local));
    }

    #[test]
    fn merge_jsonl_drops_events_older_than_one_month_from_newest_event() {
        let old = task_event_id("2026-03-01T00:00:00.000Z", "old", "create", "pending");
        let recent = task_event_id("2026-03-28T00:00:00.000Z", "recent", "create", "pending");
        let newest = task_event_id("2026-04-26T00:00:00.000Z", "newest", "create", "pending");
        let remote = format!("{old}\n{recent}\n");
        let local = format!("{newest}\n");

        let merged = merge_jsonl_lines(&remote, &local);
        assert_eq!(merged, format!("{recent}\n{newest}\n"));
    }

    #[test]
    fn merge_jsonl_caps_git_log_to_newest_1000_events() {
        let mut local = String::new();
        for i in 0..1005 {
            local.push_str(&task_event_id(
                "2026-04-26T00:00:00.000Z",
                &i.to_string(),
                "create",
                "pending",
            ));
            local.push('\n');
        }

        let merged = merge_jsonl_lines("", &local);
        let lines: Vec<&str> = merged.lines().collect();
        assert_eq!(lines.len(), 1000);
        assert!(lines[0].contains("\"entity_id\":\"5\""));
        assert!(lines[999].contains("\"entity_id\":\"1004\""));
    }

    #[test]
    fn merge_jsonl_count_cap_uses_timestamp_not_input_position() {
        let old = task_event_id("2026-04-01T00:00:00.000Z", "old", "create", "pending");
        let mut local = format!("{old}\n");
        for i in 0..1000 {
            local.push_str(&task_event_id(
                "2026-04-26T00:00:00.000Z",
                &i.to_string(),
                "create",
                "pending",
            ));
            local.push('\n');
        }

        let merged = merge_jsonl_lines("", &local);
        let lines: Vec<&str> = merged.lines().collect();
        assert_eq!(lines.len(), 1000);
        assert!(!merged.contains("\"entity_id\":\"old\""));
    }

    fn reconcile_event(ts: &str, from: &str) -> String {
        serde_json::json!({
            "v": 1,
            "ts": ts,
            "entity": "task",
            "entity_id": "3.2",
            "scope": "001-33_enhance-spec-driven-workflow-validation",
            "op": "reconciled",
            "from": from,
            "actor": "reconcile",
            "by": "@reconcile",
            "meta": {
                "reason": "task '3.2' has audit status 'pending' but no file entry"
            },
            "ctx": {
                "session_id": "test",
                "branch": "main",
                "worktree": "main",
                "commit": "abc123"
            }
        })
        .to_string()
    }

    fn task_event(ts: &str, op: &str, to: &str) -> String {
        task_event_id(ts, "3.2", op, to)
    }

    fn task_event_id(ts: &str, entity_id: &str, op: &str, to: &str) -> String {
        serde_json::json!({
            "v": 1,
            "ts": ts,
            "entity": "task",
            "entity_id": entity_id,
            "scope": "001-33_enhance-spec-driven-workflow-validation",
            "op": op,
            "to": to,
            "actor": "cli",
            "by": "@test",
            "ctx": {
                "session_id": "test"
            }
        })
        .to_string()
    }
}
