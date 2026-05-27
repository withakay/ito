use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;

use ito_config::types::{CoordinationStorage, ItoConfig, WorktreeInitConfig, WorktreeStrategy};

use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
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

#[allow(dead_code)]
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

fn make_embedded_config() -> ItoConfig {
    let mut config = ItoConfig::default();
    config.changes.coordination_branch.storage = CoordinationStorage::Embedded;
    config
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn validate_change_id_accepts_normal_ids() {
    assert!(validate_change_id("012-05_my-change").is_ok());
    assert!(validate_change_id("simple").is_ok());
    assert!(validate_change_id("with-dashes-and_underscores").is_ok());
}

#[test]
fn validate_change_id_rejects_empty() {
    assert!(validate_change_id("").is_err());
}

#[test]
fn validate_change_id_rejects_leading_dash() {
    assert!(validate_change_id("--orphan").is_err());
    assert!(validate_change_id("-b").is_err());
}

#[test]
fn validate_change_id_rejects_path_traversal() {
    assert!(validate_change_id("../escape").is_err());
    assert!(validate_change_id("foo/../bar").is_err());
}

#[test]
fn validate_change_id_rejects_path_separators() {
    assert!(validate_change_id("foo/bar").is_err());
    assert!(validate_change_id("foo\\bar").is_err());
}

#[test]
fn validate_change_id_rejects_nul() {
    assert!(validate_change_id("foo\0bar").is_err());
}

#[test]
fn ensure_worktrees_disabled_returns_cwd() {
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let config = make_embedded_config();
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
    // Simulate a linked worktree: .git file pointing to a gitdir sibling.
    let fake_gitdir = worktrees_root.join("my-change.git");
    std::fs::create_dir_all(&change_dir).unwrap();
    std::fs::create_dir_all(&fake_gitdir).unwrap();
    // .git file uses a relative pointer to the fake gitdir.
    std::fs::write(change_dir.join(".git"), "gitdir: ../my-change.git").unwrap();
    // Marker lives inside the resolved gitdir, not the working tree.
    std::fs::write(fake_gitdir.join(INIT_MARKER), "initialized\n").unwrap();

    let config = make_embedded_config();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, project_root.to_path_buf());
    let runner = StubRunner::with_outputs(vec![]);

    let result =
        ensure_worktree_with_runner(&runner, "my-change", &config, &env, &paths, project_root);
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

    let config = make_embedded_config();
    let env = make_env(project_root);
    let expected = worktrees_root.join("my-change");
    // The fake gitdir that the .git file will point to.
    let fake_gitdir = worktrees_root.join("my-change.git");
    let paths = make_enabled_paths(worktrees_root.clone(), main_root);

    struct CreatingRunner {
        target_path: PathBuf,
        fake_gitdir: PathBuf,
        calls: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl ProcessRunner for CreatingRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.calls
                .borrow_mut()
                .push((request.program.clone(), request.args.clone()));
            std::fs::create_dir_all(&self.target_path).unwrap();
            std::fs::create_dir_all(&self.fake_gitdir).unwrap();
            std::fs::write(self.target_path.join(".git"), "gitdir: ../my-change.git").unwrap();
            Ok(ProcessOutput {
                exit_code: 0,
                success: true,
                stdout: self.target_path.display().to_string(),
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
        target_path: expected.clone(),
        fake_gitdir: fake_gitdir.clone(),
        calls: RefCell::new(Vec::new()),
    };

    let result =
        ensure_worktree_with_runner(&runner, "my-change", &config, &env, &paths, project_root);

    assert_eq!(result.unwrap(), expected);
    // Marker must be inside the gitdir, not the working tree root.
    assert!(
        fake_gitdir.join(INIT_MARKER).exists(),
        "marker should be in gitdir"
    );
    assert!(
        !expected.join(INIT_MARKER).exists(),
        "marker must not appear in working tree"
    );
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "wt");
    assert!(calls[0].1.contains(&"switch".to_string()));
    assert!(calls[0].1.contains(&"--create".to_string()));
    assert!(calls[0].1.contains(&"my-change".to_string()));
    assert!(calls[0].1.contains(&"--base".to_string()));
    assert!(calls[0].1.contains(&"main".to_string()));
    let config_path = project_root.join(".ito/worktrunk/worktree-path.toml");
    let config = std::fs::read_to_string(config_path).unwrap();
    assert!(config.contains("ito-worktrees"));
    assert!(config.contains("{{ branch | sanitize }}"));
}

#[test]
fn ensure_with_include_files_copies_them() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();
    std::fs::write(main_root.join(".env"), "SECRET=abc").unwrap();

    let mut config = make_embedded_config();
    config.worktrees.init = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let env = make_env(project_root);
    let change_wt_path = worktrees_root.join("my-change");
    let fake_gitdir = worktrees_root.join("my-change.git");
    let paths = make_enabled_paths(worktrees_root, main_root.clone());

    struct CreatingRunner {
        target_path: PathBuf,
        fake_gitdir: PathBuf,
    }

    impl ProcessRunner for CreatingRunner {
        fn run(&self, _request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            std::fs::create_dir_all(&self.target_path).unwrap();
            std::fs::create_dir_all(&self.fake_gitdir).unwrap();
            std::fs::write(self.target_path.join(".git"), "gitdir: ../my-change.git").unwrap();
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
        fake_gitdir,
    };

    let result =
        ensure_worktree_with_runner(&runner, "my-change", &config, &env, &paths, project_root);

    let wt_path = result.unwrap();
    assert!(wt_path.join(".env").exists());
    assert_eq!(
        std::fs::read_to_string(wt_path.join(".env")).unwrap(),
        "SECRET=abc"
    );
}

#[test]
fn ensure_worktrunk_failure_returns_error() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();

    let config = make_embedded_config();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, main_root);
    let runner = StubRunner::with_outputs(vec![fail_output("path occupied")]);

    let result =
        ensure_worktree_with_runner(&runner, "my-change", &config, &env, &paths, project_root);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("my-change"));
    assert!(err_msg.contains("Worktrunk reported"));
    assert!(err_msg.contains("path occupied"));
}
