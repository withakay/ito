//! Change worktree ensure: verify or create the correct worktree for a change.
//!
//! The `ensure_worktree` function is the primary entrypoint used by
//! `ito worktree ensure --change <id>`. It resolves the expected path, creates
//! the worktree if absent, runs initialization (file copy + setup), and returns
//! the resolved absolute path.

use std::path::{Path, PathBuf};

use ito_config::types::ItoConfig;

#[cfg(feature = "coordination-branch")]
use crate::coordination_worktree::repair_current_worktree_coordination_links;
use crate::errors::{CoreError, CoreResult};
use crate::implementation_readiness::{
    ReadinessPhase, ReadinessReport, ReadinessRequest, evaluate_execute_from_prepare,
    evaluate_readiness,
};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
use crate::repo_paths::{ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature, WorktreeSelector};
use crate::worktree_init;

#[cfg(not(feature = "coordination-branch"))]
fn repair_current_worktree_coordination_links(
    _project_root: &Path,
    _ito_path: &Path,
    _config: &ItoConfig,
) -> CoreResult<()> {
    Ok(())
}

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
/// 3. Existing worktrees must pass execute readiness before reuse or setup.
/// 4. New worktrees must pass prepare readiness, are created from the captured
///    authority OID, and must pass execute readiness before initialization.
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
    ensure_worktree_with_runner(
        &runner,
        &SystemWorktreeReadiness,
        change_id,
        config,
        env,
        worktree_paths,
        cwd,
    )
}

pub(crate) trait WorktreeReadinessEvaluator {
    fn evaluate(&self, request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport;

    fn execute_from_prepare(
        &self,
        prepare: &ReadinessReport,
        request: &ReadinessRequest,
        config: &ItoConfig,
    ) -> ReadinessReport;
}

struct SystemWorktreeReadiness;

impl WorktreeReadinessEvaluator for SystemWorktreeReadiness {
    fn evaluate(&self, request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport {
        evaluate_readiness(request, config)
    }

    fn execute_from_prepare(
        &self,
        prepare: &ReadinessReport,
        request: &ReadinessRequest,
        config: &ItoConfig,
    ) -> ReadinessReport {
        evaluate_execute_from_prepare(prepare, request, config)
    }
}

/// Testable inner implementation of [`ensure_worktree`].
pub(crate) fn ensure_worktree_with_runner(
    runner: &dyn ProcessRunner,
    readiness: &dyn WorktreeReadinessEvaluator,
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

    let existing_checkout = worktree_path.join(".git").exists();
    let prepare_report = if existing_checkout {
        let request = ReadinessRequest::new(change_id, ReadinessPhase::Execute, &env.project_root)
            .with_current_checkout(&worktree_path);
        let report = readiness.evaluate(&request, config);
        require_ready(&report)?;
        None
    } else {
        let request = ReadinessRequest::new(change_id, ReadinessPhase::Prepare, &env.project_root);
        let report = readiness.evaluate(&request, config);
        require_ready(&report)?;
        Some(report)
    };

    // If the worktree exists and was fully initialized, return it without
    // re-init. We check for a `.git` file/dir (present in all git worktrees)
    // AND our `ito-initialized` marker inside the gitdir (proves init
    // completed without polluting `git status`). If the directory exists but
    // lacks the marker, it was partially initialized and we re-run init.
    if worktree_path.is_dir() {
        let git_entry = worktree_path.join(".git");
        let has_git = git_entry.exists();
        let ito_path = worktree_path.join(".ito");
        let has_marker = has_git && {
            resolve_gitdir(&git_entry)
                .map(|gitdir| gitdir.join(INIT_MARKER).exists())
                .unwrap_or(false)
        };
        if has_git && has_marker {
            repair_current_worktree_coordination_links(&env.project_root, &ito_path, config)?;
            return Ok(worktree_path);
        }
        // If the directory exists with .git but no marker, re-run init.
        // If no .git at all, fall through to creation (the dir is stale).
        if has_git {
            repair_current_worktree_coordination_links(&env.project_root, &ito_path, config)?;
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

    // Create the Worktrunk-managed worktree from the captured authority OID.
    let prepare_report = prepare_report.expect("absent worktree has prepare readiness");
    let authority_oid = prepare_report
        .authority
        .oid
        .as_deref()
        .expect("successful prepare readiness contains authority OID");
    let creation_state = WorktreeCreationState {
        target_preexisted: worktree_path.exists(),
        branch_preexisted: local_branch_exists(runner, &env.project_root, change_id)?,
    };
    let worktrunk_config =
        FileSnapshot::capture(env.ito_root.join("worktrunk").join("worktree-path.toml"))?;
    if let Err(error) = create_change_worktree(
        runner,
        &env.project_root,
        &env.ito_root,
        change_id,
        authority_oid,
        &worktree_path,
    ) {
        return rollback_creation_failure(
            runner,
            &env.project_root,
            change_id,
            &worktree_path,
            creation_state,
            worktrunk_config,
            error,
        );
    }

    let initialize = || -> CoreResult<()> {
        let execute_request =
            ReadinessRequest::new(change_id, ReadinessPhase::Execute, &env.project_root)
                .with_current_checkout(&worktree_path);
        let execute_report =
            readiness.execute_from_prepare(&prepare_report, &execute_request, config);
        require_ready(&execute_report)?;

        let source_root = worktree_paths.main_worktree_root.as_deref().unwrap_or(cwd);
        let ito_path = worktree_path.join(".ito");
        repair_current_worktree_coordination_links(&env.project_root, &ito_path, config)?;
        worktree_init::init_worktree_with_runner(
            runner,
            source_root,
            &worktree_path,
            &config.worktrees,
        )?;
        write_init_marker(&worktree_path)
    };
    if let Err(error) = initialize() {
        return rollback_creation_failure(
            runner,
            &env.project_root,
            change_id,
            &worktree_path,
            creation_state,
            worktrunk_config,
            error,
        );
    }

    Ok(worktree_path)
}

#[derive(Clone, Copy)]
struct WorktreeCreationState {
    target_preexisted: bool,
    branch_preexisted: bool,
}

fn rollback_creation_failure(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    change_id: &str,
    worktree_path: &Path,
    creation_state: WorktreeCreationState,
    worktrunk_config: FileSnapshot,
    error: CoreError,
) -> CoreResult<PathBuf> {
    let cleanup = rollback_created_worktree(
        runner,
        project_root,
        change_id,
        worktree_path,
        creation_state,
    );
    combine_creation_failure(error, cleanup, worktrunk_config.restore())
}

fn combine_creation_failure(
    error: CoreError,
    cleanup: CoreResult<()>,
    config_restore: CoreResult<()>,
) -> CoreResult<PathBuf> {
    let mut rollback_errors = Vec::new();
    if let Err(cleanup_error) = cleanup {
        rollback_errors.push(cleanup_error.to_string());
    }
    if let Err(restore_error) = config_restore {
        rollback_errors.push(restore_error.to_string());
    }
    if rollback_errors.is_empty() {
        return Err(error);
    }
    Err(CoreError::process(format!(
        "{error}\nRollback also failed: {}",
        rollback_errors.join("; ")
    )))
}

fn require_ready(report: &ReadinessReport) -> CoreResult<()> {
    if report.ready {
        return Ok(());
    }
    let failures = report
        .conditions
        .iter()
        .filter(|condition| !condition.passed)
        .map(|condition| {
            let remediation = condition
                .remediation
                .as_deref()
                .map(|value| format!(" Fix: {value}"))
                .unwrap_or_default();
            format!("{}: {}{remediation}", condition.code, condition.message)
        })
        .collect::<Vec<_>>()
        .join("\n");
    Err(CoreError::validation(format!(
        "Change '{}' is not ready for {}.\n{}",
        report.change_id,
        match report.phase {
            ReadinessPhase::Prepare => "worktree preparation",
            ReadinessPhase::Execute => "implementation execution",
        },
        failures
    )))
}

struct FileSnapshot {
    path: PathBuf,
    contents: Option<Vec<u8>>,
    parent_existed: bool,
}

impl FileSnapshot {
    fn capture(path: PathBuf) -> CoreResult<Self> {
        let parent_existed = path.parent().map(Path::exists).unwrap_or(false);
        let contents = match std::fs::read(&path) {
            Ok(contents) => Some(contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(CoreError::io(
                    format!("Cannot snapshot '{}'.", path.display()),
                    error,
                ));
            }
        };
        Ok(Self {
            path,
            contents,
            parent_existed,
        })
    }

    fn restore(self) -> CoreResult<()> {
        match self.contents {
            Some(contents) => std::fs::write(&self.path, contents).map_err(|error| {
                CoreError::io(format!("Cannot restore '{}'.", self.path.display()), error)
            })?,
            None => match std::fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(CoreError::io(
                        format!("Cannot remove generated '{}'.", self.path.display()),
                        error,
                    ));
                }
            },
        }
        if !self.parent_existed
            && let Some(parent) = self.path.parent()
        {
            match std::fs::remove_dir(parent) {
                Ok(()) => {}
                Err(error)
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
                    ) => {}
                Err(error) => {
                    return Err(CoreError::io(
                        format!("Cannot remove generated directory '{}'.", parent.display()),
                        error,
                    ));
                }
            }
        }
        Ok(())
    }
}

fn rollback_created_worktree(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    change_id: &str,
    target_path: &Path,
    creation_state: WorktreeCreationState,
) -> CoreResult<()> {
    let project = project_root.to_string_lossy().to_string();
    if !creation_state.target_preexisted && target_path.exists() {
        let target = target_path.to_string_lossy().to_string();
        let remove = runner
            .run(
                &ProcessRequest::new("git")
                    .args(["-C", &project, "worktree", "remove", "--force", &target]),
            )
            .map_err(|error| CoreError::process(format!("Cannot roll back worktree: {error}")))?;
        if !remove.success {
            return Err(CoreError::process(format!(
                "Cannot roll back worktree '{}': {}",
                target_path.display(),
                remove.stderr.trim()
            )));
        }
    }

    if creation_state.branch_preexisted || !local_branch_exists(runner, project_root, change_id)? {
        return Ok(());
    }
    let branch = runner
        .run(&ProcessRequest::new("git").args(["-C", &project, "branch", "-D", change_id]))
        .map_err(|error| CoreError::process(format!("Cannot roll back branch: {error}")))?;
    if !branch.success {
        return Err(CoreError::process(format!(
            "Cannot roll back branch '{change_id}': {}",
            branch.stderr.trim()
        )));
    }
    Ok(())
}

fn local_branch_exists(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    change_id: &str,
) -> CoreResult<bool> {
    let project = project_root.to_string_lossy().to_string();
    let branch_ref = format!("refs/heads/{change_id}");
    let output = runner
        .run(&ProcessRequest::new("git").args([
            "-C",
            &project,
            "show-ref",
            "--verify",
            "--quiet",
            &branch_ref,
        ]))
        .map_err(|error| CoreError::process(format!("Cannot inspect worktree branch: {error}")))?;
    match output.exit_code {
        0 => Ok(true),
        1 => Ok(false),
        _ => Err(CoreError::process(format!(
            "Cannot inspect branch '{change_id}' before worktree creation: {}",
            output.stderr.trim()
        ))),
    }
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

/// Create a Worktrunk-managed worktree for a change.
fn create_change_worktree(
    runner: &dyn ProcessRunner,
    project_root: &Path,
    ito_root: &Path,
    change_id: &str,
    base_oid: &str,
    target_path: &Path,
) -> CoreResult<()> {
    let config_path = write_worktrunk_path_config(ito_root, target_path)?;
    let config_arg = config_path.to_string_lossy().to_string();
    let project_root_arg = project_root.to_string_lossy().to_string();

    let request = ProcessRequest::new("wt")
        .args([
            "--config",
            &config_arg,
            "-C",
            &project_root_arg,
            "--yes",
            "switch",
            "--create",
            change_id,
            "--base",
            base_oid,
        ])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot create worktree for change '{change_id}' at '{target}'.\n\
             Worktrunk command failed to run: {err}\n\
             Requested change: {change_id}\n\
             Verified authority OID: {base_oid}\n\
             Fix: install Worktrunk and ensure `wt` is available on PATH, then retry the guarded Ito worktree command.",
            target = target_path.display(),
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
         Worktrunk reported: {detail}\n\
         Requested change: {change_id}\n\
         Verified authority OID: {base_oid}\n\
         Fix: ensure Worktrunk can access verified commit '{base_oid}', the target path is free, and the local Worktrunk path config points at the Ito worktree root.",
        target = target_path.display(),
    )))
}

fn write_worktrunk_path_config(ito_root: &Path, target_path: &Path) -> CoreResult<PathBuf> {
    let parent = target_path.parent().ok_or_else(|| {
        CoreError::validation(format!(
            "Cannot derive Worktrunk path config for '{}'.\n\
             Fix: configure worktrees so the target path has a parent directory.",
            target_path.display(),
        ))
    })?;

    let config_dir = ito_root.join("worktrunk");
    std::fs::create_dir_all(&config_dir).map_err(|err| {
        CoreError::io(
            format!(
                "Cannot create Worktrunk config directory '{}'.\n\
                 Fix: ensure the Ito directory is writable.",
                config_dir.display(),
            ),
            err,
        )
    })?;

    let config_path = config_dir.join("worktree-path.toml");
    let template = parent.join("{{ branch | sanitize }}");
    let contents = format!(
        "worktree-path = \"{}\"\n",
        template
            .to_string_lossy()
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
    );

    std::fs::write(&config_path, contents).map_err(|err| {
        CoreError::io(
            format!(
                "Cannot write Worktrunk path config '{}'.\n\
                 Fix: ensure the Ito directory is writable.",
                config_path.display(),
            ),
            err,
        )
    })?;

    Ok(config_path)
}

#[cfg(test)]
#[path = "worktree_ensure_tests.rs"]
mod worktree_ensure_tests;
