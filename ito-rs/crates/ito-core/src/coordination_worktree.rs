//! Coordination worktree lifecycle management.
//!
//! Provides `create_coordination_worktree` and `remove_coordination_worktree`
//! for setting up and tearing down a persistent git worktree that tracks a
//! coordination branch. The coordination branch is used to share Ito state
//! (changes, specs, audit events) across team members without touching the
//! project's main branch.
//!
//! # Branch resolution order
//!
//! When creating a worktree, the branch is resolved in this order:
//!
//! 1. Local branch already exists → use it directly.
//! 2. Remote `origin/<branch>` exists → fetch and use it.
//! 3. Neither exists → create an orphan branch with an empty initial commit.

use std::fs;
use std::path::Path;

use ito_config::types::{CoordinationStorage, ItoConfig};
use ito_config::{ConfigContext, load_cascading_project_config};
use sha2::{Digest, Sha256};

use crate::coordination::{update_gitignore_for_symlinks, wire_coordination_symlinks};
use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
use crate::repo_paths::coordination_worktree_path;

// ── Subdirectories created inside the coordination worktree ──────────────────

const ITO_SUBDIRS: &[&str] = &["changes", "specs", "modules", "workflows", "audit"];

// ── Public API ───────────────────────────────────────────────────────────────

/// Stages all changes in the coordination worktree and commits them.
///
/// The sequence is:
///
/// 1. `git -C <worktree_path> add -A` — stage everything.
/// 2. `git -C <worktree_path> diff --cached --quiet` — check for staged changes.
/// 3. If changes exist (exit code 1), commit with `message`.
/// 4. If nothing is staged (exit code 0), return `Ok(())` — no-op.
///
/// # Errors
///
/// Returns [`CoreError::Process`] when `git add` or `git commit` fails, with a
/// message that includes the worktree path and suggests a remediation step.
/// "Nothing to commit" is **not** an error.
pub fn auto_commit_coordination(worktree_path: &Path, message: &str) -> CoreResult<()> {
    let runner = SystemProcessRunner;
    auto_commit_coordination_with_runner(&runner, worktree_path, message)
}

/// Creates a git worktree at `target_path` tracking `branch_name`.
///
/// The branch is resolved in this order:
///
/// 1. **Local branch exists** — used directly.
/// 2. **Remote branch exists** — fetched from `origin` and used.
/// 3. **Neither** — an orphan branch is created with an empty initial commit.
///
/// After the worktree is created, the `.ito/` directory structure is
/// initialised inside it (subdirectories: `changes`, `specs`, `modules`,
/// `workflows`, `audit`).
///
/// # Errors
///
/// Returns [`CoreError::Process`] when any git command fails, with a message
/// that names the branch or path involved, explains what went wrong, and
/// suggests a remediation step.
pub fn create_coordination_worktree(
    project_root: &Path,
    branch_name: &str,
    target_path: &Path,
) -> CoreResult<()> {
    let runner = SystemProcessRunner;
    create_coordination_worktree_with_runner(&runner, project_root, branch_name, target_path)
}

/// Conditionally auto-commits the coordination worktree when storage mode is `worktree`.
///
/// This is the CLI-level hook to call after any mutating operation. It:
///
/// 1. Loads the cascading config from `project_root` / `ito_path` to check
///    `changes.coordination_branch.storage`.
/// 2. Returns `Ok(())` immediately (no-op) when storage is not
///    [`CoordinationStorage::Worktree`].
/// 3. Resolves the coordination worktree path from the config.
/// 4. Calls [`auto_commit_coordination`] with the resolved path and `message`.
///
/// # Errors
///
/// Returns [`CoreError`] only when config deserialization fails in an
/// unrecoverable way. Auto-commit git failures are surfaced as errors so the
/// caller can decide whether to treat them as warnings.
///
/// In practice, callers in the CLI layer should print a warning and continue
/// rather than failing the primary operation.
pub fn maybe_auto_commit_coordination(
    project_root: &Path,
    ito_path: &Path,
    message: &str,
) -> CoreResult<()> {
    let ctx = ConfigContext::from_process_env();
    let cfg_value = load_cascading_project_config(project_root, ito_path, &ctx).merged;

    let typed: ito_config::types::ItoConfig = serde_json::from_value(cfg_value).map_err(|e| {
        CoreError::serde(
            "parse Ito configuration for auto-commit check",
            e.to_string(),
        )
    })?;

    let coord = &typed.changes.coordination_branch;

    // Only act when storage mode is worktree.
    let CoordinationStorage::Worktree = coord.storage else {
        return Ok(());
    };

    // Resolve org/repo for the worktree path.  When resolution fails (e.g. no
    // git remote), fall back to a stable FNV-1a hash of the absolute project
    // root so that different local-only projects never share the same path.
    let (org, repo) = match crate::git_remote::resolve_org_repo_from_config_or_remote(
        project_root,
        &typed.backend,
    ) {
        Some(pair) => pair,
        None => {
            let hash = fnv1a_hash(project_root.to_string_lossy().as_bytes());
            ("_local".to_string(), format!("{hash:016x}"))
        }
    };

    let worktree_path = coordination_worktree_path(coord, ito_path, &org, &repo);

    // Only attempt the commit when the worktree directory actually exists.
    // If it hasn't been created yet, silently skip — the user hasn't set up
    // coordination storage yet.
    if !worktree_path.is_dir() {
        return Ok(());
    }

    auto_commit_coordination(&worktree_path, message)
}

/// Removes the coordination worktree at `target_path` and prunes stale refs.
///
/// Attempts a clean removal first; falls back to `--force` if the worktree
/// has untracked or modified files. After removal, `git worktree prune` is
/// run to clean up any dangling metadata.
///
/// # Errors
///
/// Returns [`CoreError::Process`] when the worktree cannot be removed, with a
/// message that includes the path and suggests running
/// `git worktree remove --force <path>` manually.
pub fn remove_coordination_worktree(project_root: &Path, target_path: &Path) -> CoreResult<()> {
    let runner = SystemProcessRunner;
    remove_coordination_worktree_with_runner(&runner, project_root, target_path)
}

/// Provision the coordination worktree: resolve org/repo, compute path, create
/// worktree, wire symlinks, update `.gitignore`.
///
/// Returns `Ok(Some(storage))` with the resulting [`CoordinationStorage`] mode,
/// or `Ok(None)` if setup was skipped because backend mode is active.
///
/// # Behaviour
///
/// 1. If `skip` is `true`, returns `Ok(Some(CoordinationStorage::Embedded))`
///    immediately — the caller opted out of worktree setup.
/// 2. Loads the cascading config to check whether backend mode is active.
///    When `backend.enabled` is `true`, returns `Ok(None)` — the backend owns
///    coordination and no local worktree should be created.
/// 3. Deserialises the `CoordinationBranchConfig` from the merged config.
/// 4. If `coord_config.worktree_path` is explicitly set, uses it directly
///    (skips org/repo resolution).
/// 5. Otherwise resolves `(org, repo)` from config or the `origin` remote URL.
///    Returns an error when resolution fails.
/// 6. Computes the worktree path via [`coordination_worktree_path`].
/// 7. If the worktree path does not yet exist, calls
///    [`create_coordination_worktree`] to create it.
/// 8. Wires `.ito/<dir>` → `<worktree>/.ito/<dir>` symlinks via
///    [`wire_coordination_symlinks`].
/// 9. Updates `.gitignore` via [`update_gitignore_for_symlinks`].
/// 10. Returns `Ok(Some(CoordinationStorage::Worktree))`.
///
/// On any failure the error is returned to the caller, which decides whether to
/// treat it as fatal or fall back to embedded mode.
///
/// # Errors
///
/// Returns [`CoreError`] when:
/// - The cascading config cannot be deserialised.
/// - Org/repo resolution fails (no config values and no parseable `origin` remote).
/// - Worktree creation fails (git errors).
/// - Symlink wiring fails (filesystem errors).
/// - `.gitignore` update fails (filesystem errors).
pub fn provision_coordination_worktree(
    project_root: &Path,
    ito_path: &Path,
    skip: bool,
) -> CoreResult<Option<CoordinationStorage>> {
    // 1. Caller opted out — use embedded mode.
    if skip {
        return Ok(Some(CoordinationStorage::Embedded));
    }

    // 2. Load merged config and check backend mode.
    let ctx = ConfigContext::from_process_env();
    let cfg_value = load_cascading_project_config(project_root, ito_path, &ctx).merged;

    let typed: ItoConfig = serde_json::from_value(cfg_value).map_err(|e| {
        CoreError::serde(
            "parse Ito configuration for coordination worktree setup",
            e.to_string(),
        )
    })?;

    if typed.backend.enabled {
        // Backend mode is active — backend owns coordination.
        return Ok(None);
    }

    let coord_config = typed.changes.coordination_branch;

    // 4 & 5. Resolve the worktree path.
    //
    // When `worktree_path` is explicitly set in config, use it directly and
    // skip org/repo resolution entirely.  Otherwise resolve org/repo from
    // config or the origin remote.
    let worktree_path = if coord_config
        .worktree_path
        .as_deref()
        .filter(|s| !s.is_empty())
        .is_some()
    {
        // Explicit override — pass dummy org/repo; coordination_worktree_path
        // will return the explicit path without using them.
        coordination_worktree_path(&coord_config, ito_path, "", "")
    } else {
        let (org, repo) =
            crate::git_remote::resolve_org_repo_from_config_or_remote(project_root, &typed.backend)
                .ok_or_else(|| {
                    CoreError::process(
                        "Cannot resolve org/repo for coordination worktree.\n\
                         Neither `backend.project.org`/`backend.project.repo` are set in \
                         .ito/config.json, nor does the repository have a parseable `origin` \
                         remote URL.\n\
                         Fix: add an 'origin' remote (`git remote add origin <url>`) or set \
                         `backend.project.org` and `backend.project.repo` in .ito/config.json.",
                    )
                })?;
        coordination_worktree_path(&coord_config, ito_path, &org, &repo)
    };

    // 7. Create the worktree only when it does not yet exist.
    if !worktree_path.exists() {
        let branch_name = &coord_config.name;
        create_coordination_worktree(project_root, branch_name, &worktree_path)?;
    }

    // 8. Wire symlinks (idempotent — safe to call even when worktree already existed).
    let worktree_ito_path = worktree_path.join(".ito");
    wire_coordination_symlinks(ito_path, &worktree_ito_path)?;

    // 9. Update .gitignore — best-effort; don't fail provisioning for it.
    let _ = update_gitignore_for_symlinks(project_root);

    // 10. Return the resulting storage mode.
    Ok(Some(CoordinationStorage::Worktree))
}

// ── Testable inner implementations ───────────────────────────────────────────

pub(crate) fn auto_commit_coordination_with_runner(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    message: &str,
) -> CoreResult<()> {
    stage_all(runner, worktree_path)?;

    let has_changes = has_staged_changes(runner, worktree_path)?;
    if !has_changes {
        return Ok(());
    }

    commit_staged(runner, worktree_path, message)?;
    Ok(())
}

pub(crate) fn create_coordination_worktree_with_runner(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch_name: &str,
    target_path: &Path,
) -> CoreResult<()> {
    let branch_exists_locally = local_branch_exists(runner, project_root, branch_name)?;

    if !branch_exists_locally {
        let fetched = fetch_branch_from_origin(runner, project_root, branch_name)?;
        if !fetched {
            create_orphan_branch(runner, project_root, branch_name)?;
        }
    }

    add_worktree(runner, project_root, branch_name, target_path)?;
    ensure_ito_dirs(target_path)?;

    Ok(())
}

pub(crate) fn remove_coordination_worktree_with_runner(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    target_path: &Path,
) -> CoreResult<()> {
    remove_worktree(runner, project_root, target_path)?;
    prune_worktrees(runner, project_root)?;
    Ok(())
}

// ── Git helpers ───────────────────────────────────────────────────────────────

/// Returns `true` when `branch_name` exists as a local ref.
fn local_branch_exists(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch_name: &str,
) -> CoreResult<bool> {
    let request = ProcessRequest::new("git")
        .args(["rev-parse", "--verify", branch_name])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot check whether branch '{branch_name}' exists locally.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{project_root}' is a git repository.",
            project_root = project_root.display(),
        ))
    })?;

    Ok(output.success)
}

/// Attempts to fetch `branch_name` from `origin`.
///
/// Returns `true` when the fetch succeeded (branch now exists on origin),
/// `false` when the remote branch does not exist, `origin` is not configured,
/// or the remote is unreachable, and an error for any other failure
/// (e.g. authentication errors).
fn fetch_branch_from_origin(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch_name: &str,
) -> CoreResult<bool> {
    let request = ProcessRequest::new("git")
        .args(["fetch", "origin", branch_name])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot fetch branch '{branch_name}' from origin.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and the remote 'origin' is reachable.",
        ))
    })?;

    if output.success {
        return Ok(true);
    }

    let detail = render_output(&output);
    let detail_lower = detail.to_ascii_lowercase();

    // A missing remote ref is expected when the branch has never been pushed.
    if detail_lower.contains("couldn't find remote ref")
        || detail_lower.contains("remote ref does not exist")
    {
        return Ok(false);
    }

    // No remote configured at all — treat as "branch not on remote" so the
    // caller can fall through to orphan-branch creation.
    if detail_lower.contains("no such remote")
        || detail_lower.contains("does not appear to be a git repository")
    {
        return Ok(false);
    }

    Err(CoreError::process(format!(
        "Cannot fetch branch '{branch_name}' from origin.\n\
         Git reported: {detail}\n\
         Fix: check that the remote 'origin' is configured and reachable \
         (`git remote -v`).",
    )))
}

/// Creates an orphan branch with an empty initial commit.
///
/// Strategy (in order):
///
/// 1. **`git worktree add --orphan <branch> <tmp-path>`** — available in git
///    ≥ 2.36. Creates the orphan branch directly in a temporary worktree
///    without touching the main checkout. The temporary worktree is removed
///    immediately after the branch is created.
///
/// 2. **`commit-tree` fallback** — for older git that lacks `--orphan` on
///    `worktree add`. Creates an empty tree object, commits it, and creates
///    the branch pointing at that commit. No checkout is ever performed, so
///    the working tree is never disturbed.
fn create_orphan_branch(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch_name: &str,
) -> CoreResult<()> {
    // Build a stable temporary path for the short-lived worktree.  We use a
    // sanitised branch name so the path is human-readable in error messages.
    let safe_name = branch_name.replace('/', "-").replace(' ', "_");
    let tmp_wt = project_root.join(format!(".git-orphan-tmp-{safe_name}"));
    let tmp_str = tmp_wt.to_string_lossy();

    // ── Attempt 1: git worktree add --orphan ─────────────────────────────────
    let worktree_orphan = runner
        .run(
            &ProcessRequest::new("git")
                .args(["worktree", "add", "--orphan", branch_name, tmp_str.as_ref()])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot create orphan branch '{branch_name}' in '{project_root}'.\n\
                 Git command failed to run: {err}\n\
                 Fix: ensure git is installed and '{project_root}' is a git repository.",
                project_root = project_root.display(),
            ))
        })?;

    if worktree_orphan.success {
        // The branch now exists; remove the temporary worktree immediately.
        // Failure here is non-fatal — the branch was created successfully.
        let _ = runner.run(
            &ProcessRequest::new("git")
                .args(["worktree", "remove", "--force", tmp_str.as_ref()])
                .current_dir(project_root),
        );
        // Also prune any leftover metadata.
        let _ = runner.run(
            &ProcessRequest::new("git")
                .args(["worktree", "prune"])
                .current_dir(project_root),
        );
        return Ok(());
    }

    let worktree_orphan_detail = render_output(&worktree_orphan);

    // ── Attempt 2: commit-tree fallback (git < 2.36) ─────────────────────────
    //
    // 1. Resolve the repository object format.
    // 2. Compute the corresponding empty-tree object hash.
    // 3. Commit it with `git commit-tree`.
    // 4. Create the branch pointing at that commit.
    //
    // No checkout is performed, so the working tree is never touched.

    let empty_tree = empty_tree_hash(runner, project_root)?;

    let commit_tree = runner
        .run(
            &ProcessRequest::new("git")
                .args([
                    "commit-tree",
                    empty_tree.as_str(),
                    "-m",
                    "Initialize coordination branch",
                ])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot create initial commit for orphan branch '{branch_name}'.\n\
                 Git command failed to run: {err}",
            ))
        })?;

    if !commit_tree.success {
        return Err(CoreError::process(format!(
            "Cannot create orphan branch '{branch_name}' in '{project_root}'.\n\
             `git worktree add --orphan` reported: {worktree_orphan_detail}\n\
             `git commit-tree` reported: {}\n\
             Fix: upgrade git to ≥ 2.36, or ensure git user.name and user.email are \
             configured (`git config --global user.email \"you@example.com\"`).",
            render_output(&commit_tree),
            project_root = project_root.display(),
        )));
    }

    let commit_hash = commit_tree.stdout.trim().to_string();

    let branch_create = runner
        .run(
            &ProcessRequest::new("git")
                .args(["branch", branch_name, &commit_hash])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot create branch '{branch_name}' pointing at initial commit.\n\
                 Git command failed to run: {err}",
            ))
        })?;

    if !branch_create.success {
        return Err(CoreError::process(format!(
            "Cannot create branch '{branch_name}' pointing at initial commit '{commit_hash}'.\n\
             Git reported: {}\n\
             Fix: ensure the branch name is valid and does not already exist \
             (`git branch -l '{branch_name}'`).",
            render_output(&branch_create),
        )));
    }

    Ok(())
}

/// Runs `git worktree add <target_path> <branch_name>`.
fn add_worktree(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    branch_name: &str,
    target_path: &Path,
) -> CoreResult<()> {
    let target_str = target_path.to_string_lossy();
    let request = ProcessRequest::new("git")
        .args(["worktree", "add", target_str.as_ref(), branch_name])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot add worktree at '{target}' for branch '{branch_name}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and the target path is writable.",
            target = target_path.display(),
        ))
    })?;

    if output.success {
        return Ok(());
    }

    Err(CoreError::process(format!(
        "Cannot add worktree at '{target}' for branch '{branch_name}'.\n\
         Git reported: {detail}\n\
         Fix: check that '{target}' does not already exist and that the branch \
         is not already checked out in another worktree.",
        target = target_path.display(),
        detail = render_output(&output),
    )))
}

/// Creates the `.ito/` subdirectory structure inside the worktree.
fn ensure_ito_dirs(worktree_root: &Path) -> CoreResult<()> {
    let ito_root = worktree_root.join(".ito");
    for subdir in ITO_SUBDIRS {
        let dir = ito_root.join(subdir);
        fs::create_dir_all(&dir).map_err(|err| {
            CoreError::io(
                format!(
                    "Cannot create .ito/{subdir} inside coordination worktree '{worktree}'.\n\
                     Fix: ensure the worktree path is writable.",
                    worktree = worktree_root.display(),
                ),
                err,
            )
        })?;
    }
    Ok(())
}

/// Removes the worktree at `target_path`, falling back to `--force` if needed.
fn remove_worktree(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    target_path: &Path,
) -> CoreResult<()> {
    let target_str = target_path.to_string_lossy();

    // Try clean removal first.
    let clean = runner
        .run(
            &ProcessRequest::new("git")
                .args(["worktree", "remove", target_str.as_ref()])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot remove coordination worktree at '{target}'.\n\
                 Git command failed to run: {err}\n\
                 Fix: run `git worktree remove {target}` manually.",
                target = target_path.display(),
            ))
        })?;

    if clean.success {
        return Ok(());
    }

    // Fall back to --force for worktrees with untracked/modified files.
    let forced = runner
        .run(
            &ProcessRequest::new("git")
                .args(["worktree", "remove", "--force", target_str.as_ref()])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot force-remove coordination worktree at '{target}'.\n\
                 Git command failed to run: {err}\n\
                 Fix: run `git worktree remove --force {target}` manually.",
                target = target_path.display(),
            ))
        })?;

    if forced.success {
        return Ok(());
    }

    Err(CoreError::process(format!(
        "Cannot remove coordination worktree at '{target}'.\n\
         Git reported: {detail}\n\
         Fix: run `git worktree remove --force {target}` manually, \
         or delete the directory and run `git worktree prune`.",
        target = target_path.display(),
        detail = render_output(&forced),
    )))
}

/// Runs `git worktree prune` to clean up stale worktree metadata.
fn prune_worktrees(runner: &dyn ProcessRunner, project_root: &Path) -> CoreResult<()> {
    let output = runner
        .run(
            &ProcessRequest::new("git")
                .args(["worktree", "prune"])
                .current_dir(project_root),
        )
        .map_err(|err| {
            CoreError::process(format!(
                "Cannot prune stale worktree metadata in '{project_root}'.\n\
                 Git command failed to run: {err}\n\
                 Fix: run `git worktree prune` manually.",
                project_root = project_root.display(),
            ))
        })?;

    if output.success {
        return Ok(());
    }

    Err(CoreError::process(format!(
        "Cannot prune stale worktree metadata in '{project_root}'.\n\
         Git reported: {detail}\n\
         Fix: run `git worktree prune` manually.",
        project_root = project_root.display(),
        detail = render_output(&output),
    )))
}

/// Runs `git -C <worktree_path> add -A` to stage all changes.
fn stage_all(runner: &dyn ProcessRunner, worktree_path: &Path) -> CoreResult<()> {
    let worktree_str = worktree_path.to_string_lossy();
    let request = ProcessRequest::new("git").args(["-C", worktree_str.as_ref(), "add", "-A"]);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot stage changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;

    if output.success {
        return Ok(());
    }

    Err(CoreError::process(format!(
        "Cannot stage changes in coordination worktree '{worktree}'.\n\
         Git reported: {detail}\n\
         Fix: ensure '{worktree}' is a valid git worktree and the files are readable.",
        worktree = worktree_path.display(),
        detail = render_output(&output),
    )))
}

/// Returns `true` when there are staged changes ready to commit.
///
/// Uses `git diff --cached --quiet`: exit code 0 means no changes, exit code 1
/// means changes exist. Any other failure (e.g. not a git repo) is an error.
fn has_staged_changes(runner: &dyn ProcessRunner, worktree_path: &Path) -> CoreResult<bool> {
    let worktree_str = worktree_path.to_string_lossy();
    let request = ProcessRequest::new("git").args([
        "-C",
        worktree_str.as_ref(),
        "diff",
        "--cached",
        "--quiet",
    ]);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot check for staged changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;

    // exit code 0 → no staged changes; exit code 1 → staged changes exist.
    // Any other non-zero exit code is unexpected — treat it as an error.
    match output.exit_code {
        0 => Ok(false),
        1 => Ok(true),
        code => Err(CoreError::process(format!(
            "Cannot check for staged changes in coordination worktree '{worktree}'.\n\
             Git exited with unexpected code {code}: {detail}\n\
             Fix: ensure '{worktree}' is a valid git worktree.",
            worktree = worktree_path.display(),
            detail = render_output(&output),
        ))),
    }
}

/// Runs `git -C <worktree_path> commit -m <message>` to commit staged changes.
fn commit_staged(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    message: &str,
) -> CoreResult<()> {
    let worktree_str = worktree_path.to_string_lossy();
    let request =
        ProcessRequest::new("git").args(["-C", worktree_str.as_ref(), "commit", "-m", message]);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot commit staged changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;

    if output.success {
        return Ok(());
    }

    Err(CoreError::process(format!(
        "Cannot commit staged changes in coordination worktree '{worktree}'.\n\
         Git reported: {detail}\n\
         Fix: ensure git user.name and user.email are configured \
         (`git config --global user.email \"you@example.com\"`).",
        worktree = worktree_path.display(),
        detail = render_output(&output),
    )))
}

// ── Hashing ───────────────────────────────────────────────────────────────────

/// FNV-1a 64-bit hash — stable across Rust versions, no external dependencies.
///
/// Used to derive a deterministic per-project identifier when no git remote is
/// available. Unlike `DefaultHasher`, FNV-1a produces the same output for the
/// same input regardless of the Rust version or platform.
fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

// ── Shared utilities ──────────────────────────────────────────────────────────

fn render_output(output: &crate::process::ProcessOutput) -> String {
    let stderr = output.stderr.trim();
    let stdout = output.stdout.trim();
    if !stderr.is_empty() {
        return stderr.to_string();
    }
    if !stdout.is_empty() {
        return stdout.to_string();
    }
    "no command output".to_string()
}

fn empty_tree_hash(runner: &dyn ProcessRunner, project_root: &Path) -> CoreResult<String> {
    let object_format = repository_object_format(runner, project_root)?;
    let hash = match object_format {
        GitObjectFormat::Sha1 => "4b825dc642cb6eb9a060e54bf8d69288fbee4904".to_string(),
        GitObjectFormat::Sha256 => hex::encode(Sha256::digest(b"tree 0\0")),
    };
    Ok(hash)
}

fn repository_object_format(
    runner: &dyn ProcessRunner,
    project_root: &Path,
) -> CoreResult<GitObjectFormat> {
    let output = runner.run(
        &ProcessRequest::new("git")
            .args(["rev-parse", "--show-object-format"])
            .current_dir(project_root),
    );

    let Ok(output) = output else {
        return Ok(GitObjectFormat::Sha1);
    };
    if !output.success {
        return Ok(GitObjectFormat::Sha1);
    }

    let format = output.stdout.trim();
    let object_format = match format {
        "sha256" => GitObjectFormat::Sha256,
        _ => GitObjectFormat::Sha1,
    };
    Ok(object_format)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GitObjectFormat {
    Sha1,
    Sha256,
}

#[cfg(test)]
#[path = "coordination_worktree_tests.rs"]
mod coordination_worktree_tests;
