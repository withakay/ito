use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;

use ito_config::types::{ItoConfig, WorktreeInitConfig, WorktreeStrategy};

use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRunner, ProcessRequest};
use crate::repo_paths::{GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature};

// ── Stub runner ──────────────────────────────────────────────────────────────

struct StubRunner {
    outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    calls: RefCell<Vec<(String, Vec<String>)>>,
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
        self.calls
            .borrow_mut()
            .push((request.program.clone(), request.args.clone()));
        self.outputs
            .borrow_mut()
            .pop_front()
            .expect("StubRunner ran out of queued outputs")
    }

    fn run_with_timeout(
        &self,
        _request: &ProcessRequest,
        _timeout: std::time::Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        unreachable!("not used in worktree_ensure tests")
    }
}

fn ok_output() -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 0,
        success: true,
        stdout: String::new(),
        stderr: String::new(),
        timed_out: false,
    })
}

fn fail_output(stderr: &str) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 1,
        success: false,
        stdout: String::new(),
        stderr: stderr.to_string(),
        timed_out: false,
    })
}

fn make_env(project_root: &std::path::Path) -> ResolvedEnv {
    ResolvedEnv {
        worktree_root: project_root.to_path_buf(),
        project_root: project_root.to_path_buf(),
        ito_root: project_root.join(".ito"),
        git_repo_kind: GitRepoKind::NonBare,
    }
}

fn make_enabled_paths(worktrees_root: PathBuf, main_root: PathBuf) -> ResolvedWorktreePaths {
    ResolvedWorktreePaths {
        feature: WorktreeFeature::Enabled,
        strategy: WorktreeStrategy::BareControlSiblings,
        worktrees_root: Some(worktrees_root),
        main_worktree_root: Some(main_root),
    }
}

fn make_disabled_paths() -> ResolvedWorktreePaths {
    ResolvedWorktreePaths {
        feature: WorktreeFeature::Disabled,
        strategy: WorktreeStrategy::CheckoutSubdir,
        worktrees_root: None,
        main_worktree_root: None,
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn ensure_worktrees_disabled_returns_cwd() {
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let config = ItoConfig::default();
    let env = make_env(cwd);
    let paths = make_disabled_paths();
    let runner = StubRunner::with_outputs(vec![]);

    let result = ensure_worktree_with_runner(&runner, "my-change", &config, &env, &paths, cwd);
    assert_eq!(result.unwrap(), cwd.to_path_buf());
    assert!(runner.calls.borrow().is_empty());
}

#[test]
fn ensure_existing_worktree_returns_path_without_creation() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let change_dir = worktrees_root.join("my-change");
    std::fs::create_dir_all(&change_dir).unwrap();

    let config = ItoConfig::default();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, project_root.to_path_buf());
    let runner = StubRunner::with_outputs(vec![]);

    let result = ensure_worktree_with_runner(
        &runner,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );
    assert_eq!(result.unwrap(), change_dir);
    // No git commands should have been issued.
    assert!(runner.calls.borrow().is_empty());
}

#[test]
fn ensure_creates_worktree_when_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();

    let config = ItoConfig::default();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root.clone(), main_root);
    // git worktree add → success
    let runner = StubRunner::with_outputs(vec![ok_output()]);

    let result = ensure_worktree_with_runner(
        &runner,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );

    // The function should have attempted to create the worktree via git.
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "git");
    assert!(calls[0].1.contains(&"worktree".to_string()));
    assert!(calls[0].1.contains(&"add".to_string()));
    assert!(calls[0].1.contains(&"my-change".to_string()));

    // The result is the expected path (the directory gets created by git,
    // but since we stub git, we created it ourselves to make the test valid).
    // In practice git creates it; here we just verify the returned path.
    let expected = worktrees_root.join("my-change");
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn ensure_with_include_files_copies_them() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();
    std::fs::write(main_root.join(".env"), "SECRET=abc").unwrap();

    // The runner will create the directory when git worktree add is called,
    // simulating git's behaviour.

    let mut config = ItoConfig::default();
    config.worktrees.init = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let env = make_env(project_root);
    let change_wt_path = worktrees_root.join("my-change");
    let paths = make_enabled_paths(worktrees_root, main_root.clone());

    // We'll use a custom runner that creates the directory when git worktree add is called.
    struct CreatingRunner {
        target_path: PathBuf,
    }

    impl ProcessRunner for CreatingRunner {
        fn run(
            &self,
            _request: &ProcessRequest,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            // Simulate git creating the worktree directory.
            std::fs::create_dir_all(&self.target_path).unwrap();
            Ok(ProcessOutput {
                exit_code: 0,
                success: true,
                stdout: String::new(),
                stderr: String::new(),
                timed_out: false,
            })
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!()
        }
    }

    let runner = CreatingRunner {
        target_path: change_wt_path,
    };

    let result = ensure_worktree_with_runner(
        &runner,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );

    let wt_path = result.unwrap();
    assert!(wt_path.join(".env").exists());
    assert_eq!(
        std::fs::read_to_string(wt_path.join(".env")).unwrap(),
        "SECRET=abc"
    );
}

#[test]
fn ensure_git_failure_returns_error() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();

    let config = ItoConfig::default();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, main_root);
    let runner = StubRunner::with_outputs(vec![fail_output("fatal: not a git repository")]);

    let result = ensure_worktree_with_runner(
        &runner,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("my-change"));
}
