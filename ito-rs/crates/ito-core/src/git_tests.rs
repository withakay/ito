use super::*;
use crate::errors::CoreError;
use crate::process::ProcessExecutionError;
use std::cell::RefCell;
use std::collections::VecDeque;

struct StubRunner {
    outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    calls: RefCell<Vec<Vec<String>>>,
}

impl StubRunner {
    fn with_outputs(outputs: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
        Self {
            outputs: RefCell::new(outputs.into()),
            calls: RefCell::new(Vec::new()),
        }
    }
}

impl ProcessRunner for StubRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.calls.borrow_mut().push(request.args.clone());
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
    let err =
        fetch_coordination_branch_with_runner(&runner, &repo, "ito/internal/changes").unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::RemoteMissing);
    assert!(err.message.contains("does not exist"));
}

#[test]
fn push_coordination_branch_classifies_non_fast_forward_rejection() {
    let runner = StubRunner::with_outputs(vec![Ok(err_output(
        "! [rejected] HEAD -> ito/internal/changes (non-fast-forward)",
    ))]);
    let repo = std::env::temp_dir();
    let err = push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
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
    let err = push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
        .unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::ProtectedBranch);
}

#[test]
fn fetch_coordination_branch_classifies_missing_remote_configuration() {
    let runner = StubRunner::with_outputs(vec![Ok(err_output(
        "fatal: 'origin' does not appear to be a git repository\nfatal: No such remote: 'origin'",
    ))]);
    let repo = std::env::temp_dir();
    let err =
        fetch_coordination_branch_with_runner(&runner, &repo, "ito/internal/changes").unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::RemoteNotConfigured);
    assert!(err.message.contains("not configured"));
}

#[test]
fn setup_coordination_branch_returns_ready_when_remote_branch_exists() {
    let runner = StubRunner::with_outputs(vec![Ok(ok_output("true\n", "")), Ok(ok_output("", ""))]);
    let repo = std::env::temp_dir();
    let result =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
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
        Ok(ok_output("4b825dc642cb6eb9a060e54bf8d69288fbee4904\n", "")),
        Ok(ok_output("deadbeef1234567890abcdef1234567890abcdef\n", "")),
        Ok(ok_output("", "")),
    ]);
    let repo = std::env::temp_dir();
    let result =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
            .expect("setup should create branch");
    assert_eq!(result, CoordinationBranchSetupStatus::Created);

    let calls = runner.calls.borrow();
    assert_eq!(calls[2], ["mktree"]);
    assert_eq!(calls[3][0], "commit-tree");
    assert_eq!(calls[3][1], "4b825dc642cb6eb9a060e54bf8d69288fbee4904");
    assert!(
        !calls[3].contains(&"-p".to_string()),
        "coordination init commit must be a root commit: {:?}",
        calls[3]
    );
    assert!(
        calls[3].contains(&"Initialize coordination branch".to_string()),
        "commit-tree should use coordination init message: {:?}",
        calls[3]
    );
    assert_eq!(
        calls[4],
        [
            "push",
            "origin",
            "deadbeef1234567890abcdef1234567890abcdef:refs/heads/ito/internal/changes"
        ]
    );
    assert!(
        !calls[4][2].starts_with("HEAD:"),
        "setup must not create coordination branch from HEAD: {:?}",
        calls[4]
    );
}

#[test]
fn setup_coordination_branch_reports_empty_tree_failure() {
    let runner = StubRunner::with_outputs(vec![
        Ok(ok_output("true\n", "")),
        Ok(err_output(
            "fatal: couldn't find remote ref ito/internal/changes",
        )),
        Ok(err_output("fatal: invalid tree input")),
    ]);
    let repo = std::env::temp_dir();

    let err =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::CommandFailed);
    assert!(
        err.message.contains("failed to create empty tree"),
        "unexpected error message: {}",
        err.message
    );
}

#[test]
fn setup_coordination_branch_reports_commit_tree_failure() {
    let runner = StubRunner::with_outputs(vec![
        Ok(ok_output("true\n", "")),
        Ok(err_output(
            "fatal: couldn't find remote ref ito/internal/changes",
        )),
        Ok(ok_output("4b825dc642cb6eb9a060e54bf8d69288fbee4904\n", "")),
        Ok(err_output("fatal: unable to auto-detect email address")),
    ]);
    let repo = std::env::temp_dir();

    let err =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::CommandFailed);
    assert!(
        err.message
            .contains("failed to create empty coordination branch commit"),
        "unexpected error message: {}",
        err.message
    );
}

#[test]
fn setup_coordination_branch_rejects_empty_commit_tree_stdout() {
    let runner = StubRunner::with_outputs(vec![
        Ok(ok_output("true\n", "")),
        Ok(err_output(
            "fatal: couldn't find remote ref ito/internal/changes",
        )),
        Ok(ok_output("4b825dc642cb6eb9a060e54bf8d69288fbee4904\n", "")),
        Ok(ok_output("\n", "")),
    ]);
    let repo = std::env::temp_dir();

    let err =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::CommandFailed);
    assert!(
        err.message.contains("produced no commit hash"),
        "unexpected error message: {}",
        err.message
    );

    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 4, "empty commit hash must not be pushed");
}

#[test]
fn setup_coordination_branch_fails_when_not_git_worktree() {
    let runner = StubRunner::with_outputs(vec![Ok(err_output(
        "fatal: not a git repository (or any of the parent directories): .git",
    ))]);
    let repo = std::env::temp_dir();
    let err =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
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
    let err = push_coordination_branch_with_runner(&runner, &repo, "HEAD", "ito/internal/changes")
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
        Ok(ok_output("4b825dc642cb6eb9a060e54bf8d69288fbee4904\n", "")),
        Ok(ok_output("deadbeef1234567890abcdef1234567890abcdef\n", "")),
        Ok(err_output(
            "fatal: 'origin' does not appear to be a git repository",
        )),
    ]);
    let repo = std::env::temp_dir();
    let err =
        ensure_coordination_branch_on_origin_with_runner(&runner, &repo, "ito/internal/changes")
            .unwrap_err();
    assert_eq!(err.kind, CoordinationGitErrorKind::RemoteNotConfigured);
    assert!(err.message.contains("not configured"));
}

#[test]
fn setup_coordination_branch_core_wraps_process_error() {
    let repo = std::env::temp_dir().join("ito-not-a-repo");
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).expect("temp dir created");

    let err = ensure_coordination_branch_on_origin_core(&repo, "ito/internal/changes").unwrap_err();
    let CoreError::Process(msg) = err else {
        panic!("expected process error");
    };
    assert!(msg.contains("coordination setup failed"));
}
