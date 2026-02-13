//! Git synchronization helpers for coordination workflows.

use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessOutput, ProcessRequest, ProcessRunner, SystemProcessRunner};
use ito_domain::tasks::tasks_path_checked;

/// Error category for coordination branch git operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinationGitErrorKind {
    /// Push was rejected because remote history moved ahead.
    NonFastForward,
    /// Push was rejected by branch protection.
    ProtectedBranch,
    /// Remote rejected the update for another reason.
    RemoteRejected,
    /// Requested branch does not exist on remote.
    RemoteMissing,
    /// Git remote is not configured/available.
    RemoteNotConfigured,
    /// Generic command failure.
    CommandFailed,
}

/// Structured failure details for coordination branch operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinationGitError {
    /// Classified error kind.
    pub kind: CoordinationGitErrorKind,
    /// Human-readable error message.
    pub message: String,
}

/// Outcome of a coordination branch setup attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinationBranchSetupStatus {
    /// Remote branch already existed and is reachable.
    Ready,
    /// Remote branch was created during setup.
    Created,
}

impl CoordinationGitError {
    /// Constructs a `CoordinationGitError` with the specified kind and human-readable message.
    ///
    /// The provided message is converted into a `String`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let _err = CoordinationGitError::new(
    ///     CoordinationGitErrorKind::RemoteMissing,
    ///     "origin/ref not found",
    /// );
    /// ```
    fn new(kind: CoordinationGitErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

/// Fetches a coordination branch from `origin` into remote-tracking refs.
///
/// Returns `Ok(())` on success. Returns a classified error when fetch fails.
pub fn fetch_coordination_branch(
    repo_root: &Path,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    let runner = SystemProcessRunner;
    fetch_coordination_branch_with_runner(&runner, repo_root, branch)
}

/// Pushes the given local ref to the coordination branch on `origin`.
///
/// Returns `Ok(())` on success, `Err(CoordinationGitError)` classified by failure reason otherwise.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use ito_core::git::push_coordination_branch;
///
/// let repo = Path::new(".");
/// let _ = push_coordination_branch(repo, "HEAD", "coordination");
/// ```
pub fn push_coordination_branch(
    repo_root: &Path,
    local_ref: &str,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    let runner = SystemProcessRunner;
    push_coordination_branch_with_runner(&runner, repo_root, local_ref, branch)
}

/// Reserves a newly created change on the coordination branch using an ephemeral worktree.
///
/// The reservation is written in a temporary worktree so the caller's current worktree and branch are not modified.
/// Returns `Ok(())` on success or a `CoordinationGitError` describing the failure.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// let repo = Path::new("/path/to/repo");
/// let ito = Path::new(".ito");
/// let change_id = "change-123";
/// let branch = "coordination";
/// ito_core::git::reserve_change_on_coordination_branch(repo, ito, change_id, branch).unwrap();
/// ```
pub fn reserve_change_on_coordination_branch(
    repo_root: &Path,
    ito_path: &Path,
    change_id: &str,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    let runner = SystemProcessRunner;
    reserve_change_on_coordination_branch_with_runner(
        &runner, repo_root, ito_path, change_id, branch,
    )
}

/// Ensures the coordination branch exists on `origin`.
///
/// If the branch is already present on the remote, this returns `CoordinationBranchSetupStatus::Ready`.
/// If the branch is missing and is created by pushing the local `HEAD`, this returns `CoordinationBranchSetupStatus::Created`.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// let status = ito_core::git::ensure_coordination_branch_on_origin(Path::new("."), "coordination-branch");
/// match status {
///     Ok(ito_core::git::CoordinationBranchSetupStatus::Ready) => println!("Branch already exists"),
///     Ok(ito_core::git::CoordinationBranchSetupStatus::Created) => println!("Branch created on origin"),
///     Err(e) => eprintln!("Failed to ensure branch: {:?}", e),
/// }
/// ```
pub fn ensure_coordination_branch_on_origin(
    repo_root: &Path,
    branch: &str,
) -> Result<CoordinationBranchSetupStatus, CoordinationGitError> {
    let runner = SystemProcessRunner;
    ensure_coordination_branch_on_origin_with_runner(&runner, repo_root, branch)
}

/// Fetches the coordination branch from the `origin` remote.
///
/// # Examples
///
/// ```no_run
/// use ito_core::git::fetch_coordination_branch_core;
///
/// let res = fetch_coordination_branch_core(std::path::Path::new("."), "coordination");
/// assert!(res.is_ok());
/// ```
pub fn fetch_coordination_branch_core(repo_root: &Path, branch: &str) -> CoreResult<()> {
    fetch_coordination_branch(repo_root, branch)
        .map_err(|err| CoreError::process(format!("coordination fetch failed: {}", err.message)))
}

/// Push a local ref to the coordination branch on origin, converting git-related failures into a CoreError prefixed with "coordination push failed:".
///
/// On success this returns `Ok(())`. On failure this returns a `CoreError` whose message begins with `coordination push failed:` followed by the underlying coordination git error message.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// // Attempt to push the local ref "HEAD" to the coordination branch "refs/heads/coord".
/// // In real usage, handle the CoreResult appropriately; here we simply call the function.
/// let _ = ito_core::git::push_coordination_branch_core(Path::new("."), "HEAD", "coord");
/// ```
pub fn push_coordination_branch_core(
    repo_root: &Path,
    local_ref: &str,
    branch: &str,
) -> CoreResult<()> {
    push_coordination_branch(repo_root, local_ref, branch)
        .map_err(|err| CoreError::process(format!("coordination push failed: {}", err.message)))
}

/// Reserve change metadata on the coordination branch, translating coordination git failures into `CoreError`.
///
/// On success the reservation is pushed to the remote coordination branch; on failure the returned error is a
/// `CoreError` describing the coordination failure.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let _ = ito_core::git::reserve_change_on_coordination_branch_core(
///     Path::new("/path/to/repo"),
///     Path::new(".ito"),
///     "CHANGE-123",
///     "coordination",
/// );
/// ```
pub fn reserve_change_on_coordination_branch_core(
    repo_root: &Path,
    ito_path: &Path,
    change_id: &str,
    branch: &str,
) -> CoreResult<()> {
    reserve_change_on_coordination_branch(repo_root, ito_path, change_id, branch).map_err(|err| {
        CoreError::process(format!("coordination reservation failed: {}", err.message))
    })
}

/// Ensure the coordination branch exists on the remote origin and report whether it was already present or was created.
///
/// On failure, converts coordination git errors into a `CoreError` whose message is prefixed with `coordination setup failed: `.
///
/// # Returns
///
/// `Ok(CoordinationBranchSetupStatus)` when the branch is present or was created; `Err(CoreError)` when the operation fails.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use ito_core::errors::CoreError;
///
/// let status = ito_core::git::ensure_coordination_branch_on_origin_core(Path::new("."), "coordination/main")?;
/// match status {
///     ito_core::git::CoordinationBranchSetupStatus::Ready => println!("Branch exists on origin"),
///     ito_core::git::CoordinationBranchSetupStatus::Created => println!("Branch was created on origin"),
/// }
/// # Ok::<(), CoreError>(())
/// ```
pub fn ensure_coordination_branch_on_origin_core(
    repo_root: &Path,
    branch: &str,
) -> CoreResult<CoordinationBranchSetupStatus> {
    ensure_coordination_branch_on_origin(repo_root, branch)
        .map_err(|err| CoreError::process(format!("coordination setup failed: {}", err.message)))
}

/// Ensures the coordination branch exists on the remote `origin`, creating it by pushing `HEAD` if necessary.
///
/// Attempts to fetch `origin/<branch>`; if the fetch succeeds the function returns
/// `CoordinationBranchSetupStatus::Ready`. If the fetch fails because the remote ref is
/// missing, the function pushes `HEAD` to `origin` to create the branch and returns
/// `CoordinationBranchSetupStatus::Created`. The function returns an `Err` if it is not
/// invoked inside a git worktree or if the underlying fetch/push fail for other reasons.
///
/// # Examples
///
/// ```ignore
/// // `runner` must implement `ProcessRunner`.
/// use std::path::Path;
/// let _ = ensure_coordination_branch_on_origin_with_runner(&runner, Path::new("/path/to/repo"), "coordination").unwrap();
/// ```
pub(crate) fn ensure_coordination_branch_on_origin_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<CoordinationBranchSetupStatus, CoordinationGitError> {
    if !is_git_worktree(runner, repo_root) {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            "cannot set up coordination branch outside a git worktree",
        ));
    }

    match fetch_coordination_branch_with_runner(runner, repo_root, branch) {
        Ok(()) => Ok(CoordinationBranchSetupStatus::Ready),
        Err(err) => {
            if err.kind != CoordinationGitErrorKind::RemoteMissing {
                return Err(err);
            }

            push_coordination_branch_with_runner(runner, repo_root, "HEAD", branch)
                .map(|()| CoordinationBranchSetupStatus::Created)
        }
    }
}

/// Fetches the coordination branch from `origin` into the repository's remote-tracking refs.
///
/// Returns a `CoordinationGitError` when the operation fails:
/// - `RemoteMissing` if the remote branch does not exist on `origin`.
/// - `RemoteNotConfigured` if the `origin` remote is not configured or unreachable.
/// - `CommandFailed` for other failures; the error message includes git command output.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// // `runner` should implement `ProcessRunner` (e.g., `SystemProcessRunner` in production).
/// let runner = crate::tests::StubRunner::default(); // replace with a real runner in real usage
/// let repo = Path::new("/path/to/repo");
/// let _ = fetch_coordination_branch_with_runner(&runner, repo, "coordination");
/// ```
pub(crate) fn fetch_coordination_branch_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    validate_coordination_branch_name(branch)?;

    let request = ProcessRequest::new("git")
        .args(["fetch", "origin", branch])
        .current_dir(repo_root);
    let output = run_git(runner, request, "fetch")?;
    if output.success {
        return Ok(());
    }

    let detail = render_output(&output);
    let detail_lower = detail.to_ascii_lowercase();
    if detail_lower.contains("couldn't find remote ref")
        || detail_lower.contains("remote ref does not exist")
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::RemoteMissing,
            format!("remote branch '{branch}' does not exist ({detail})"),
        ));
    }
    if detail_lower.contains("no such remote")
        || detail_lower.contains("does not appear to be a git repository")
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::RemoteNotConfigured,
            format!("git remote 'origin' is not configured ({detail})"),
        ));
    }

    Err(CoordinationGitError::new(
        CoordinationGitErrorKind::CommandFailed,
        format!("git fetch origin {branch} failed ({detail})"),
    ))
}

/// Pushes a local ref to the coordination branch on the `origin` remote.
///
/// The `local_ref` is pushed to `refs/heads/<branch>` on the `origin` remote. The function
/// validates the branch name before attempting the push and classifies failures into
/// meaningful `CoordinationGitErrorKind` variants.
///
/// # Parameters
///
/// - `repo_root`: repository working directory where the git command is executed.
/// - `local_ref`: source ref to push (for example `"HEAD"` or `"refs/heads/my-branch"`).
/// - `branch`: target coordination branch name on `origin`.
///
/// # Returns
///
/// `Ok(())` if the push succeeded; `Err(CoordinationGitError)` on failure with a kind that
/// indicates the failure reason (for example: non-fast-forward, protected branch, remote rejected,
/// remote not configured, or a general command failure).
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// // `runner` should implement ProcessRunner; `repo_root` should point to a git repository.
/// // push_coordination_branch_with_runner(&runner, Path::new("/path/to/repo"), "HEAD", "coordination")
/// //     .expect("push should succeed");
/// ```
pub(crate) fn push_coordination_branch_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    local_ref: &str,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    validate_coordination_branch_name(branch)?;

    let refspec = format!("{local_ref}:refs/heads/{branch}");
    let request = ProcessRequest::new("git")
        .args(["push", "origin", &refspec])
        .current_dir(repo_root);
    let output = run_git(runner, request, "push")?;
    if output.success {
        return Ok(());
    }

    let detail = render_output(&output);
    let detail_lower = detail.to_ascii_lowercase();
    if detail_lower.contains("non-fast-forward") {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::NonFastForward,
            format!(
                "push to '{branch}' was rejected because remote is ahead; sync and retry ({detail})"
            ),
        ));
    }
    if detail_lower.contains("protected branch")
        || detail_lower.contains("protected branch hook declined")
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::ProtectedBranch,
            format!("push to '{branch}' blocked by branch protection ({detail})"),
        ));
    }
    if detail_lower.contains("[rejected]") || detail_lower.contains("remote rejected") {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::RemoteRejected,
            format!("push to '{branch}' was rejected by remote ({detail})"),
        ));
    }
    if detail_lower.contains("no such remote")
        || detail_lower.contains("does not appear to be a git repository")
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::RemoteNotConfigured,
            format!("git remote 'origin' is not configured ({detail})"),
        ));
    }

    Err(CoordinationGitError::new(
        CoordinationGitErrorKind::CommandFailed,
        format!("git push for '{branch}' failed ({detail})"),
    ))
}

pub(crate) fn reserve_change_on_coordination_branch_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    ito_path: &Path,
    change_id: &str,
    branch: &str,
) -> Result<(), CoordinationGitError> {
    if !is_git_worktree(runner, repo_root) {
        return Ok(());
    }

    validate_coordination_branch_name(branch)?;

    let Some(tasks_path) = tasks_path_checked(ito_path, change_id) else {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("invalid change id path segment: '{change_id}'"),
        ));
    };
    let Some(source_change_dir) = tasks_path.parent() else {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!(
                "failed to derive change directory from '{}'",
                tasks_path.display()
            ),
        ));
    };

    if !source_change_dir.exists() {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!(
                "change directory '{}' does not exist",
                source_change_dir.display()
            ),
        ));
    }

    let worktree_path = unique_temp_worktree_path();

    run_git(
        runner,
        ProcessRequest::new("git")
            .args([
                "worktree",
                "add",
                "--detach",
                worktree_path.to_string_lossy().as_ref(),
            ])
            .current_dir(repo_root),
        "worktree add",
    )?;

    let cleanup = WorktreeCleanup {
        repo_root: repo_root.to_path_buf(),
        worktree_path: worktree_path.clone(),
    };

    let fetch_result = fetch_coordination_branch_with_runner(runner, repo_root, branch);
    match fetch_result {
        Ok(()) => {
            let checkout_target = format!("origin/{branch}");
            let checkout = run_git(
                runner,
                ProcessRequest::new("git")
                    .args(["checkout", "--detach", &checkout_target])
                    .current_dir(&worktree_path),
                "checkout coordination branch",
            )?;
            if !checkout.success {
                return Err(CoordinationGitError::new(
                    CoordinationGitErrorKind::CommandFailed,
                    format!(
                        "failed to checkout coordination branch '{branch}' ({})",
                        render_output(&checkout),
                    ),
                ));
            }
        }
        Err(err) => {
            if err.kind != CoordinationGitErrorKind::RemoteMissing {
                return Err(err);
            }
        }
    }

    let target_change_dir = worktree_path.join(".ito").join("changes").join(change_id);
    if target_change_dir.exists() {
        fs::remove_dir_all(&target_change_dir).map_err(|err| {
            CoordinationGitError::new(
                CoordinationGitErrorKind::CommandFailed,
                format!(
                    "failed to replace existing reserved change '{}' ({err})",
                    target_change_dir.display()
                ),
            )
        })?;
    }
    copy_dir_recursive(source_change_dir, &target_change_dir).map_err(|err| {
        CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("failed to copy change into reservation worktree: {err}"),
        )
    })?;

    let relative_change_path = format!(".ito/changes/{change_id}");
    let add = run_git(
        runner,
        ProcessRequest::new("git")
            .args(["add", &relative_change_path])
            .current_dir(&worktree_path),
        "add reserved change",
    )?;
    if !add.success {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("failed to stage reserved change ({})", render_output(&add)),
        ));
    }

    let staged = run_git(
        runner,
        ProcessRequest::new("git")
            .args(["diff", "--cached", "--quiet", "--", &relative_change_path])
            .current_dir(&worktree_path),
        "check staged changes",
    )?;
    if staged.success {
        if let Err(err) = cleanup.cleanup_with_runner(runner) {
            eprintln!(
                "Warning: failed to remove temporary coordination worktree '{}': {}",
                cleanup.worktree_path.display(),
                err.message
            );
        }
        drop(cleanup);
        return Ok(());
    }
    if staged.exit_code != 1 {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!(
                "failed to inspect staged reservation changes ({})",
                render_output(&staged)
            ),
        ));
    }

    let commit_message = format!("chore(coordination): reserve {change_id}");
    let commit = run_git(
        runner,
        ProcessRequest::new("git")
            .args(["commit", "-m", &commit_message])
            .current_dir(&worktree_path),
        "commit reserved change",
    )?;
    if !commit.success {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!(
                "failed to commit reserved change ({})",
                render_output(&commit)
            ),
        ));
    }

    let push = push_coordination_branch_with_runner(runner, &worktree_path, "HEAD", branch);
    if let Err(err) = cleanup.cleanup_with_runner(runner) {
        eprintln!(
            "Warning: failed to remove temporary coordination worktree '{}': {}",
            cleanup.worktree_path.display(),
            err.message
        );
    }
    drop(cleanup);
    push
}

fn run_git(
    runner: &dyn ProcessRunner,
    request: ProcessRequest,
    operation: &str,
) -> Result<ProcessOutput, CoordinationGitError> {
    runner.run(&request).map_err(|err| {
        CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("git {operation} command failed to run: {err}"),
        )
    })
}

fn render_output(output: &ProcessOutput) -> String {
    let stdout = output.stdout.trim();
    let stderr = output.stderr.trim();

    if !stderr.is_empty() {
        return stderr.to_string();
    }
    if !stdout.is_empty() {
        return stdout.to_string();
    }
    "no command output".to_string()
}

fn copy_dir_recursive(source: &Path, target: &Path) -> std::io::Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path)?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            eprintln!(
                "Warning: skipped symlink while reserving coordination change: {}",
                source_path.display()
            );
            continue;
        }
        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &target_path)?;
            continue;
        }
        if file_type.is_file() {
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}

fn is_git_worktree(runner: &dyn ProcessRunner, repo_root: &Path) -> bool {
    let request = ProcessRequest::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(repo_root);
    let Ok(output) = runner.run(&request) else {
        return false;
    };
    output.success && output.stdout.trim() == "true"
}

fn unique_temp_worktree_path() -> std::path::PathBuf {
    let pid = std::process::id();
    let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0,
    };
    std::env::temp_dir().join(format!("ito-coordination-{pid}-{nanos}"))
}

fn validate_coordination_branch_name(branch: &str) -> Result<(), CoordinationGitError> {
    if branch.is_empty()
        || branch.starts_with('-')
        || branch.starts_with('/')
        || branch.ends_with('/')
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("invalid coordination branch name '{branch}'"),
        ));
    }
    if branch.contains("..")
        || branch.contains("@{")
        || branch.contains("//")
        || branch.ends_with('.')
        || branch.ends_with(".lock")
    {
        return Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!("invalid coordination branch name '{branch}'"),
        ));
    }

    for ch in branch.chars() {
        if ch.is_ascii_control() || ch == ' ' {
            return Err(CoordinationGitError::new(
                CoordinationGitErrorKind::CommandFailed,
                format!("invalid coordination branch name '{branch}'"),
            ));
        }
        if ch == '~' || ch == '^' || ch == ':' || ch == '?' || ch == '*' || ch == '[' || ch == '\\'
        {
            return Err(CoordinationGitError::new(
                CoordinationGitErrorKind::CommandFailed,
                format!("invalid coordination branch name '{branch}'"),
            ));
        }
    }

    for segment in branch.split('/') {
        if segment.is_empty()
            || segment.starts_with('.')
            || segment.ends_with('.')
            || segment.ends_with(".lock")
        {
            return Err(CoordinationGitError::new(
                CoordinationGitErrorKind::CommandFailed,
                format!("invalid coordination branch name '{branch}'"),
            ));
        }
    }

    Ok(())
}

struct WorktreeCleanup {
    repo_root: std::path::PathBuf,
    worktree_path: std::path::PathBuf,
}

impl WorktreeCleanup {
    fn cleanup_with_runner(&self, runner: &dyn ProcessRunner) -> Result<(), CoordinationGitError> {
        let output = run_git(
            runner,
            ProcessRequest::new("git")
                .args([
                    "worktree",
                    "remove",
                    "--force",
                    self.worktree_path.to_string_lossy().as_ref(),
                ])
                .current_dir(&self.repo_root),
            "worktree remove",
        )?;
        if output.success {
            return Ok(());
        }

        Err(CoordinationGitError::new(
            CoordinationGitErrorKind::CommandFailed,
            format!(
                "failed to remove temporary worktree '{}' ({})",
                self.worktree_path.display(),
                render_output(&output)
            ),
        ))
    }
}

impl Drop for WorktreeCleanup {
    fn drop(&mut self) {
        let _ = std::process::Command::new("git")
            .args([
                "worktree",
                "remove",
                "--force",
                self.worktree_path.to_string_lossy().as_ref(),
            ])
            .current_dir(&self.repo_root)
            .output();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::CoreError;
    use crate::process::ProcessExecutionError;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    struct StubRunner {
        outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    }

    impl StubRunner {
        fn with_outputs(outputs: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
            Self {
                outputs: RefCell::new(outputs.into()),
            }
        }
    }

    impl ProcessRunner for StubRunner {
        fn run(&self, _request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.outputs
                .borrow_mut()
                .pop_front()
                .expect("expected process output")
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!("not used")
        }
    }

    fn ok_output(stdout: &str, stderr: &str) -> ProcessOutput {
        ProcessOutput {
            exit_code: 0,
            success: true,
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            timed_out: false,
        }
    }

    fn err_output(stderr: &str) -> ProcessOutput {
        ProcessOutput {
            exit_code: 1,
            success: false,
            stdout: String::new(),
            stderr: stderr.to_string(),
            timed_out: false,
        }
    }

    #[test]
    fn fetch_coordination_branch_succeeds_on_clean_fetch() {
        let runner = StubRunner::with_outputs(vec![Ok(ok_output("", ""))]);
        let repo = std::env::temp_dir();
        let result = fetch_coordination_branch_with_runner(&runner, &repo, "ito/internal/changes");
        assert!(result.is_ok());
    }

    #[test]
    fn fetch_coordination_branch_classifies_missing_remote_branch() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "fatal: couldn't find remote ref ito/internal/changes",
        ))]);
        let repo = std::env::temp_dir();
        let err = fetch_coordination_branch_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::RemoteMissing);
        assert!(err.message.contains("does not exist"));
    }

    #[test]
    fn push_coordination_branch_classifies_non_fast_forward_rejection() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "! [rejected] HEAD -> ito/internal/changes (non-fast-forward)",
        ))]);
        let repo = std::env::temp_dir();
        let err =
            push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
                .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::NonFastForward);
        assert!(err.message.contains("sync and retry"));
    }

    #[test]
    fn push_coordination_branch_classifies_protection_rejection() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "remote: error: GH006: Protected branch update failed",
        ))]);
        let repo = std::env::temp_dir();
        let err =
            push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
                .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::ProtectedBranch);
    }

    #[test]
    fn fetch_coordination_branch_classifies_missing_remote_configuration() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "fatal: 'origin' does not appear to be a git repository\nfatal: No such remote: 'origin'",
        ))]);
        let repo = std::env::temp_dir();
        let err = fetch_coordination_branch_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::RemoteNotConfigured);
        assert!(err.message.contains("not configured"));
    }

    #[test]
    fn setup_coordination_branch_returns_ready_when_remote_branch_exists() {
        let runner =
            StubRunner::with_outputs(vec![Ok(ok_output("true\n", "")), Ok(ok_output("", ""))]);
        let repo = std::env::temp_dir();
        let result = ensure_coordination_branch_on_origin_with_runner(
            &runner,
            &repo,
            "ito/internal/changes",
        )
        .expect("setup should succeed");
        assert_eq!(result, CoordinationBranchSetupStatus::Ready);
    }

    #[test]
    fn setup_coordination_branch_creates_branch_when_remote_missing() {
        let runner = StubRunner::with_outputs(vec![
            Ok(ok_output("true\n", "")),
            Ok(err_output(
                "fatal: couldn't find remote ref ito/internal/changes",
            )),
            Ok(ok_output("", "")),
        ]);
        let repo = std::env::temp_dir();
        let result = ensure_coordination_branch_on_origin_with_runner(
            &runner,
            &repo,
            "ito/internal/changes",
        )
        .expect("setup should create branch");
        assert_eq!(result, CoordinationBranchSetupStatus::Created);
    }

    #[test]
    fn setup_coordination_branch_fails_when_not_git_worktree() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "fatal: not a git repository (or any of the parent directories): .git",
        ))]);
        let repo = std::env::temp_dir();
        let err = ensure_coordination_branch_on_origin_with_runner(
            &runner,
            &repo,
            "ito/internal/changes",
        )
        .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::CommandFailed);
        assert!(err.message.contains("outside a git worktree"));
    }

    #[test]
    fn push_coordination_branch_classifies_missing_remote_configuration() {
        let runner = StubRunner::with_outputs(vec![Ok(err_output(
            "fatal: 'origin' does not appear to be a git repository\nfatal: No such remote: 'origin'",
        ))]);
        let repo = std::env::temp_dir();
        let err =
            push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
                .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::RemoteNotConfigured);
        assert!(err.message.contains("not configured"));
    }

    #[test]
    fn setup_coordination_branch_reports_missing_origin_when_create_push_fails() {
        let runner = StubRunner::with_outputs(vec![
            Ok(ok_output("true\n", "")),
            Ok(err_output(
                "fatal: couldn't find remote ref ito/internal/changes",
            )),
            Ok(err_output(
                "fatal: 'origin' does not appear to be a git repository",
            )),
        ]);
        let repo = std::env::temp_dir();
        let err = ensure_coordination_branch_on_origin_with_runner(
            &runner,
            &repo,
            "ito/internal/changes",
        )
        .unwrap_err();
        assert_eq!(err.kind, CoordinationGitErrorKind::RemoteNotConfigured);
        assert!(err.message.contains("not configured"));
    }

    #[test]
    fn setup_coordination_branch_core_wraps_process_error() {
        let repo = std::env::temp_dir().join("ito-not-a-repo");
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("temp dir created");

        let err =
            ensure_coordination_branch_on_origin_core(&repo, "ito/internal/changes").unwrap_err();
        let CoreError::Process(msg) = err else {
            panic!("expected process error");
        };
        assert!(msg.contains("coordination setup failed"));
    }
}
