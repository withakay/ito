//! Change worktree ensure: verify or create the correct worktree for a change.
//!
//! The [`ensure_worktree`] function is the primary entrypoint used by
//! `ito worktree ensure --change <id>`. It resolves the expected path, creates
//! the worktree if absent, runs initialization (file copy + setup), and returns
//! the resolved absolute path.

use std::path::{Path, PathBuf};

use ito_config::types::ItoConfig;

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
use crate::repo_paths::{ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature, WorktreeSelector};
use crate::worktree_init;

/// Marker file written into the worktree's gitdir after successful initialization.
///
/// For linked worktrees, `.git` is a file containing `gitdir: <path>`.  The
/// marker is placed inside that resolved gitdir directory so it never appears
/// as an untracked file in `git status`.
const INIT_MARKER: &str = "ito-initialized";

/// Ensure the correct change worktree exists and is initialized.
///
/// Returns the resolved absolute path to the worktree.
///
/// # Behaviour
///
/// 1. When `worktrees.enabled` is `false`, returns `cwd` (the current working
///    directory passed in).
/// 2. Derives the expected worktree path from the configured strategy and layout.
/// 3. If the path exists and is a directory, assumes it is already initialized
///    and returns it immediately (no re-initialization, no setup re-run).
/// 4. If the path does not exist, creates the worktree branching from
///    `worktrees.default_branch`, runs [`worktree_init::init_worktree`], and
///    returns the path.
///
/// # Errors
///
/// Returns [`CoreError`] if path resolution fails, git worktree creation fails,
/// or initialization fails.
pub fn ensure_worktree(
    change_id: &str,
    config: &ItoConfig,
    env: &ResolvedEnv,
    worktree_paths: &ResolvedWorktreePaths,
    cwd: &Path,
) -> CoreResult<PathBuf> {
    let runner = SystemProcessRunner;
    ensure_worktree_with_runner(&runner, change_id, config, env, worktree_paths, cwd)
}

/// Testable inner implementation of [`ensure_worktree`].
pub(crate) fn ensure_worktree_with_runner(
    runner: &dyn ProcessRunner,
    change_id: &str,
    config: &ItoConfig,
    env: &ResolvedEnv,
    worktree_paths: &ResolvedWorktreePaths,
    cwd: &Path,
) -> CoreResult<PathBuf> {
    // Validate change_id to prevent path traversal and git flag injection.
    validate_change_id(change_id)?;

    // When worktrees are disabled, work in the current directory.
    let WorktreeFeature::Enabled = worktree_paths.feature else {
        return Ok(cwd.to_path_buf());
    };

    // Resolve the expected path for this change.
    let selector = WorktreeSelector::Change(change_id.to_string());
    let worktree_path = worktree_paths.path_for_selector(&selector).ok_or_else(|| {
        CoreError::validation(format!(
            "Cannot resolve worktree path for change '{change_id}'.\n\
             Worktrees are enabled but the worktrees root could not be determined.\n\
             Fix: check 'worktrees.strategy' and 'worktrees.layout' in .ito/config.json.",
        ))
    })?;

    // If the worktree exists and was fully initialized, return it without
    // re-init. We check for a `.git` file/dir (present in all git worktrees)
    // AND our `ito-initialized` marker inside the gitdir (proves init
    // completed without polluting `git status`). If the directory exists but
    // lacks the marker, it was partially initialized and we re-run init.
    if worktree_path.is_dir() {
        let git_entry = worktree_path.join(".git");
        let has_git = git_entry.exists();
        let has_marker = has_git && {
            resolve_gitdir(&git_entry)
                .map(|gitdir| gitdir.join(INIT_MARKER).exists())
                .unwrap_or(false)
        };
        if has_git && has_marker {
            return Ok(worktree_path);
        }
        // If the directory exists with .git but no marker, re-run init.
        // If no .git at all, fall through to creation (the dir is stale).
        if has_git {
            let source_root = worktree_paths.main_worktree_root.as_deref().unwrap_or(cwd);
            worktree_init::init_worktree_with_runner(
                runner,
                source_root,
                &worktree_path,
                &config.worktrees,
            )?;
            write_init_marker(&worktree_path)?;
            return Ok(worktree_path);
        }
    }

    // Create the parent directory if needed.
    if let Some(parent) = worktree_path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            CoreError::io(
                format!(
                    "Cannot create worktrees directory '{}'.\n\
                     Fix: ensure the path is writable.",
                    parent.display(),
                ),
                err,
            )
        })?;
    }

    // Create the git worktree.
    let default_branch = &config.worktrees.default_branch;
    create_change_worktree(
        runner,
        &env.project_root,
        change_id,
        default_branch,
        &worktree_path,
    )?;

    // Resolve the source root (main worktree) for file copy.
    let source_root = worktree_paths.main_worktree_root.as_deref().unwrap_or(cwd);

    // Initialize: copy files + run setup.
    worktree_init::init_worktree_with_runner(
        runner,
        source_root,
        &worktree_path,
        &config.worktrees,
    )?;

    // Write marker to indicate initialization completed successfully.
    write_init_marker(&worktree_path)?;

    Ok(worktree_path)
}

/// Resolve the actual gitdir path for a worktree.
///
/// For a regular (main) worktree, `.git` is a directory and is returned as-is.
/// For a linked worktree, `.git` is a file whose first line has the form
/// `gitdir: <path>` — the `<path>` is resolved relative to the worktree root
/// and returned.
///
/// Returns `None` if the `.git` entry does not exist, cannot be read, does
/// not contain a valid `gitdir:` pointer, or the resolved path does not
/// exist on disk.
///
/// For linked worktrees the pointer is resolved relative to the directory
/// containing the `.git` file and then canonicalized to eliminate `..`
/// segments and symlinks.  This prevents a crafted `gitdir:` value from
/// escaping the repository tree.
fn resolve_gitdir(git_entry: &Path) -> Option<PathBuf> {
    if git_entry.is_dir() {
        return Some(git_entry.to_path_buf());
    }

    // Linked worktree: `.git` is a file containing `gitdir: <path>`.
    let content = std::fs::read_to_string(git_entry).ok()?;
    let line = content.lines().next()?;
    let pointer = line.strip_prefix("gitdir:")?;
    let pointer = pointer.trim();

    if pointer.is_empty() {
        return None;
    }

    // Resolve relative to the directory that contains the `.git` file,
    // then canonicalize so `..` segments and symlinks cannot escape.
    // `canonicalize` returns `Err` when the target does not exist — that
    // is fine because a legitimate linked-worktree gitdir always exists
    // by the time `ensure_worktree` runs.
    let parent = git_entry.parent()?;
    let gitdir = parent.join(pointer);
    gitdir.canonicalize().ok()
}

/// Write the initialization marker into the worktree's gitdir.
///
/// The marker is placed inside the resolved gitdir (not the working tree root)
/// so it never appears as an untracked file in `git status`.
fn write_init_marker(worktree_path: &Path) -> CoreResult<()> {
    let git_entry = worktree_path.join(".git");
    let gitdir = resolve_gitdir(&git_entry).ok_or_else(|| {
        CoreError::validation(format!(
            "Cannot resolve gitdir for worktree at '{}'.\n\
             Fix: ensure the worktree has a valid .git file or directory.",
            worktree_path.display(),
        ))
    })?;

    let marker_path = gitdir.join(INIT_MARKER);
    std::fs::write(&marker_path, "initialized\n").map_err(|err| {
        CoreError::io(
            format!(
                "Cannot write initialization marker at '{}'.\n\
                 Fix: ensure the gitdir path is writable.",
                marker_path.display(),
            ),
            err,
        )
    })
}

/// Validate that a change ID is safe to use as a branch name and path segment.
///
/// Rejects IDs that:
/// - Are empty
/// - Start with `-` (could be interpreted as git flags)
/// - Contain `..` (path traversal)
/// - Contain path separators (`/` or `\`)
/// - Contain NUL bytes
fn validate_change_id(change_id: &str) -> CoreResult<()> {
    if change_id.is_empty() {
        return Err(CoreError::validation(
            "Change ID must not be empty.\n\
             Fix: provide a valid change ID (e.g. '012-05_my-change').",
        ));
    }
    if change_id.starts_with('-') {
        return Err(CoreError::validation(format!(
            "Change ID '{change_id}' must not start with '-'.\n\
             A leading dash could be misinterpreted as a git flag.\n\
             Fix: use a change ID that starts with an alphanumeric character.",
        )));
    }
    if change_id.contains("..") {
        return Err(CoreError::validation(format!(
            "Change ID '{change_id}' must not contain '..'.\n\
             This could enable path traversal.\n\
             Fix: use a change ID without '..' components.",
        )));
    }
    if change_id.contains('/') || change_id.contains('\\') || change_id.contains('\0') {
        return Err(CoreError::validation(format!(
            "Change ID '{change_id}' contains invalid characters (/, \\, or NUL).\n\
             Fix: use a change ID with only alphanumeric characters, dashes, and underscores.",
        )));
    }
    Ok(())
}

/// Check whether a git branch already exists in the repository.
///
/// Runs `git rev-parse --verify <branch>` and returns `true` when the exit
/// code is 0 (branch exists), `false` when it is non-zero (branch absent).
/// Any process-execution error is propagated as a [`CoreError`].
fn branch_exists(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch: &str,
) -> CoreResult<bool> {
    let request = ProcessRequest::new("git")
        .args(["rev-parse", "--verify", branch])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot check whether branch '{branch}' exists.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{project_root}' is a git repository.",
            project_root = project_root.display(),
        ))
    })?;

    Ok(output.success)
}

/// Create a git worktree for a change.
///
/// Pre-checks whether the branch already exists using `git rev-parse --verify`
/// to avoid relying on English-only git error message text.  When the branch
/// exists, `git worktree add <path> <branch>` is used; when it does not,
/// `git worktree add <path> -b <branch> <base_branch>` creates it.
fn create_change_worktree(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    change_id: &str,
    base_branch: &str,
    target_path: &Path,
) -> CoreResult<()> {
    let target_str = target_path.to_string_lossy();

    // Pre-check branch existence to choose the right `git worktree add` form.
    let branch_already_exists = branch_exists(runner, project_root, change_id)?;

    let request = if branch_already_exists {
        // Branch exists — attach the new worktree to it without -b.
        ProcessRequest::new("git")
            .args(["worktree", "add", target_str.as_ref(), change_id])
            .current_dir(project_root)
    } else {
        // Branch absent — create it from base_branch.
        ProcessRequest::new("git")
            .args([
                "worktree",
                "add",
                target_str.as_ref(),
                "-b",
                change_id,
                base_branch,
            ])
            .current_dir(project_root)
    };

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot create worktree for change '{change_id}' at '{target}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{project_root}' is a git repository.",
            target = target_path.display(),
            project_root = project_root.display(),
        ))
    })?;

    if output.success {
        return Ok(());
    }

    let detail = if !output.stderr.trim().is_empty() {
        output.stderr.trim().to_string()
    } else if !output.stdout.trim().is_empty() {
        output.stdout.trim().to_string()
    } else {
        "no command output".to_string()
    };

    Err(CoreError::process(format!(
        "Cannot create worktree for change '{change_id}' at '{target}'.\n\
         Git reported: {detail}\n\
         Fix: ensure the base branch '{base_branch}' exists and the target path \
         does not already exist. If the branch is already checked out in another \
         worktree, run `git worktree list` to inspect.",
        target = target_path.display(),
    )))
}

#[cfg(test)]
#[path = "worktree_ensure_tests.rs"]
mod worktree_ensure_tests;
