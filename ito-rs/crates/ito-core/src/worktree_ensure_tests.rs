use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::PathBuf;

use ito_config::types::{
    CoordinationStorage, ItoConfig, WorktreeInitConfig, WorktreeSetupConfig, WorktreeStrategy,
};

use super::*;
use crate::process::{
    ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner, SystemProcessRunner,
};
use crate::repo_paths::{GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature};

const AUTHORITY_OID: &str = "1111111111111111111111111111111111111111";

struct AlwaysReady;

impl WorktreeReadinessEvaluator for AlwaysReady {
    fn evaluate(&self, request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport {
        ready_report(request, config)
    }

    fn execute_from_prepare(
        &self,
        _prepare: &ReadinessReport,
        request: &ReadinessRequest,
        config: &ItoConfig,
    ) -> ReadinessReport {
        ready_report(request, config)
    }
}

struct RejectExecute;

impl WorktreeReadinessEvaluator for RejectExecute {
    fn evaluate(&self, request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport {
        ready_report(request, config)
    }

    fn execute_from_prepare(
        &self,
        _prepare: &ReadinessReport,
        request: &ReadinessRequest,
        config: &ItoConfig,
    ) -> ReadinessReport {
        let mut report = ready_report(request, config);
        report.ready = false;
        report
            .conditions
            .push(crate::implementation_readiness::ReadinessCondition {
                code: "checkout_identity".to_string(),
                passed: false,
                message: "Worktrunk created an unexpected checkout.".to_string(),
                remediation: Some("Recreate it from the verified authority OID.".to_string()),
                path: None,
                validator_code: None,
            });
        report
    }
}

fn ready_report(request: &ReadinessRequest, config: &ItoConfig) -> ReadinessReport {
    ReadinessReport {
        change_id: request.change_id.clone(),
        phase: request.phase,
        ready: true,
        authority: crate::implementation_readiness::AuthorityEvidence {
            integration_mode: config.changes.proposal.integration_mode,
            target_ref: Some(format!("refs/heads/{}", config.worktrees.default_branch)),
            oid: Some(AUTHORITY_OID.to_string()),
        },
        proposal_integration_oid: Some(AUTHORITY_OID.to_string()),
        conditions: Vec::new(),
    }
}

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

    let result = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
        "my-change",
        &config,
        &env,
        &paths,
        cwd,
    );
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

    let result = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
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
            if request.program == "git" {
                return fail_output("");
            }
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

    let result = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );

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
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, "git");
    assert!(calls[0].1.contains(&"show-ref".to_string()));
    assert_eq!(calls[1].0, "wt");
    assert!(calls[1].1.contains(&"switch".to_string()));
    assert!(calls[1].1.contains(&"--create".to_string()));
    assert!(calls[1].1.contains(&"my-change".to_string()));
    assert!(calls[1].1.contains(&"--base".to_string()));
    assert!(calls[1].1.contains(&AUTHORITY_OID.to_string()));
    let config_path = project_root.join(".ito/worktrunk/worktree-path.toml");
    let config = std::fs::read_to_string(config_path).unwrap();
    assert!(config.contains("ito-worktrees"));
    assert!(config.contains("{{ branch | sanitize }}"));
}

#[test]
fn execute_failure_rolls_back_created_worktree_branch_and_config() {
    struct TransactionalRunner {
        target: PathBuf,
        calls: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl ProcessRunner for TransactionalRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.calls
                .borrow_mut()
                .push((request.program.clone(), request.args.clone()));
            if request.args.iter().any(|arg| arg == "show-ref") {
                return if self.calls.borrow().len() == 1 {
                    fail_output("")
                } else {
                    ok_output()
                };
            } else if request.program == "wt" {
                std::fs::create_dir_all(self.target.join(".git")).unwrap();
            } else if request.args.iter().any(|arg| arg == "worktree") {
                std::fs::remove_dir_all(&self.target).unwrap();
            }
            ok_output()
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!()
        }
    }

    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let target = worktrees_root.join("my-change");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();
    let runner = TransactionalRunner {
        target: target.clone(),
        calls: RefCell::new(Vec::new()),
    };
    let config = make_embedded_config();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, main_root);

    let error = ensure_worktree_with_runner(
        &runner,
        &RejectExecute,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    )
    .unwrap_err();

    assert!(error.to_string().contains("checkout_identity"));
    assert!(!target.exists());
    assert!(
        !project_root
            .join(".ito/worktrunk/worktree-path.toml")
            .exists()
    );
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 5);
    assert!(calls[0].1.contains(&"show-ref".to_string()));
    assert_eq!(calls[1].0, "wt");
    assert!(
        calls[2]
            .1
            .windows(2)
            .any(|args| args == ["worktree", "remove"])
    );
    assert!(calls[3].1.contains(&"show-ref".to_string()));
    assert!(calls[4].1.windows(2).any(|args| args == ["branch", "-D"]));
}

struct PostCreationFailureRunner {
    target: PathBuf,
    fake_gitdir: PathBuf,
    valid_gitdir: bool,
    fail_setup: bool,
    branch_exists: Cell<bool>,
    calls: RefCell<Vec<(String, Vec<String>)>>,
}

impl ProcessRunner for PostCreationFailureRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.calls
            .borrow_mut()
            .push((request.program.clone(), request.args.clone()));
        if request.args.iter().any(|arg| arg == "show-ref") {
            return if self.branch_exists.get() {
                ok_output()
            } else {
                fail_output("")
            };
        }
        if request.program == "wt" {
            std::fs::create_dir_all(&self.target).unwrap();
            if self.valid_gitdir {
                std::fs::create_dir_all(&self.fake_gitdir).unwrap();
                std::fs::write(self.target.join(".git"), "gitdir: ../my-change.git").unwrap();
            } else {
                std::fs::write(self.target.join(".git"), "gitdir: ../missing.git").unwrap();
            }
            self.branch_exists.set(true);
            return ok_output();
        }
        if request.args.iter().any(|arg| arg == "worktree") {
            std::fs::remove_dir_all(&self.target).unwrap();
            return ok_output();
        }
        if request.args.windows(2).any(|args| args == ["branch", "-D"]) {
            self.branch_exists.set(false);
            return ok_output();
        }
        if self.fail_setup {
            return fail_output("configured setup failed");
        }
        ok_output()
    }

    fn run_with_timeout(
        &self,
        _request: &ProcessRequest,
        _timeout: std::time::Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        unreachable!()
    }
}

fn assert_post_creation_failure_rolls_back(
    config: ItoConfig,
    valid_gitdir: bool,
    fail_setup: bool,
    expected_error: &str,
) {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let target = worktrees_root.join("my-change");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();
    let worktrunk_config = project_root.join(".ito/worktrunk/worktree-path.toml");
    std::fs::create_dir_all(worktrunk_config.parent().unwrap()).unwrap();
    std::fs::write(&worktrunk_config, "original = true\n").unwrap();
    let runner = PostCreationFailureRunner {
        target: target.clone(),
        fake_gitdir: worktrees_root.join("my-change.git"),
        valid_gitdir,
        fail_setup,
        branch_exists: Cell::new(false),
        calls: RefCell::new(Vec::new()),
    };

    let error = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
        "my-change",
        &config,
        &make_env(project_root),
        &make_enabled_paths(worktrees_root, main_root),
        project_root,
    )
    .unwrap_err();

    assert!(error.to_string().contains(expected_error), "{error}");
    assert!(!target.exists());
    assert!(!runner.branch_exists.get());
    assert_eq!(
        std::fs::read_to_string(worktrunk_config).unwrap(),
        "original = true\n"
    );
    let calls = runner.calls.borrow();
    assert!(
        calls
            .iter()
            .any(|(_, args)| args.windows(2).any(|pair| pair == ["worktree", "remove"]))
    );
    assert!(
        calls
            .iter()
            .any(|(_, args)| args.windows(2).any(|pair| pair == ["branch", "-D"]))
    );
}

#[test]
fn coordination_repair_failure_rolls_back_created_worktree() {
    let mut config = make_embedded_config();
    config.changes.coordination_branch.storage = CoordinationStorage::Worktree;
    config.changes.coordination_branch.worktree_path = Some("missing-coordination".to_string());
    assert_post_creation_failure_rolls_back(config, true, false, "Coordination worktree not found");
}

#[test]
fn setup_failure_rolls_back_created_worktree() {
    let mut config = make_embedded_config();
    config.worktrees.init.setup = Some(WorktreeSetupConfig::Single("false".to_string()));
    assert_post_creation_failure_rolls_back(config, true, true, "configured setup failed");
}

#[test]
fn marker_failure_rolls_back_created_worktree() {
    assert_post_creation_failure_rolls_back(
        make_embedded_config(),
        false,
        false,
        "Cannot resolve gitdir",
    );
}

#[test]
fn real_worktrunk_uses_captured_oid_when_main_moves_after_prepare() {
    fn git(repo: &std::path::Path, args: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .args(args)
            .current_dir(repo)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    struct MoveMainBeforeWorktrunk {
        project_root: PathBuf,
        moved: Cell<bool>,
        wt_args: RefCell<Vec<String>>,
    }

    impl ProcessRunner for MoveMainBeforeWorktrunk {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            if request.program == "wt" && !self.moved.replace(true) {
                std::fs::write(self.project_root.join("main-moved.txt"), "later\n").unwrap();
                git(&self.project_root, &["add", "main-moved.txt"]);
                git(
                    &self.project_root,
                    &["commit", "-m", "move main after prepare"],
                );
                self.wt_args.replace(request.args.clone());
            }
            SystemProcessRunner.run(request)
        }

        fn run_with_timeout(
            &self,
            request: &ProcessRequest,
            timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            SystemProcessRunner.run_with_timeout(request, timeout)
        }
    }

    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path().join("repo");
    let change_id = "031-02_captured-base";
    std::fs::create_dir_all(
        project_root
            .join(".ito/changes")
            .join(change_id)
            .join("specs/base"),
    )
    .unwrap();
    git(&project_root, &["init", "--initial-branch=main"]);
    git(&project_root, &["config", "user.name", "Ito Test"]);
    git(
        &project_root,
        &["config", "user.email", "ito@example.invalid"],
    );
    let change = project_root.join(".ito/changes").join(change_id);
    std::fs::write(change.join(".ito.yaml"), "schema: spec-driven\n").unwrap();
    std::fs::write(
        change.join("proposal.md"),
        "# Proposal\n\nCreate the worktree from the captured authority commit.\n",
    )
    .unwrap();
    std::fs::write(
        change.join("design.md"),
        "# Design\n\nMove main between prepare and Worktrunk creation.\n",
    )
    .unwrap();
    std::fs::write(
        change.join("tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Work\n- **Dependencies**: None\n- **Updated At**: 2026-07-13\n- **Status**: [ ] pending\n",
    )
    .unwrap();
    std::fs::write(
        change.join("specs/base/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Captured base\nIto SHALL use the captured authority OID.\n\n#### Scenario: Moving main\n- **WHEN** main advances after prepare\n- **THEN** the worktree still uses the captured OID\n",
    )
    .unwrap();
    git(&project_root, &["add", ".ito"]);
    git(&project_root, &["commit", "-m", "integrate proposal"]);
    let captured_oid = git(&project_root, &["rev-parse", "HEAD"]);

    let mut config = make_embedded_config();
    config.changes.proposal.integration_mode =
        ito_config::types::ProposalIntegrationMode::DirectMerge;
    config.worktrees.enabled = true;
    config.worktrees.strategy = WorktreeStrategy::CheckoutSiblings;
    config.worktrees.init = WorktreeInitConfig::default();
    let worktrees_root = tmp.path().join("ito-worktrees");
    let target = worktrees_root.join(change_id);
    let env = make_env(&project_root);
    let paths = make_enabled_paths(worktrees_root, project_root.clone());
    let runner = MoveMainBeforeWorktrunk {
        project_root: project_root.clone(),
        moved: Cell::new(false),
        wt_args: RefCell::new(Vec::new()),
    };

    let created = ensure_worktree_with_runner(
        &runner,
        &SystemWorktreeReadiness,
        change_id,
        &config,
        &env,
        &paths,
        &project_root,
    )
    .unwrap();

    assert_eq!(created, target);
    assert_ne!(git(&project_root, &["rev-parse", "HEAD"]), captured_oid);
    assert_eq!(git(&created, &["rev-parse", "HEAD"]), captured_oid);
    let args = runner.wt_args.borrow();
    let base_index = args.iter().position(|arg| arg == "--base").unwrap();
    assert_eq!(args[base_index + 1], captured_oid);
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
            if _request.program == "git" {
                return fail_output("");
            }
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

    let result = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
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
fn ensure_worktrunk_failure_returns_error() {
    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();

    let config = make_embedded_config();
    let env = make_env(project_root);
    let paths = make_enabled_paths(worktrees_root, main_root);
    let runner = StubRunner::with_outputs(vec![
        fail_output(""),
        fail_output("path occupied"),
        fail_output(""),
    ]);

    let result = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
        "my-change",
        &config,
        &env,
        &paths,
        project_root,
    );
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("my-change"));
    assert!(err_msg.contains("Worktrunk reported"));
    assert!(err_msg.contains("path occupied"));
}

#[test]
fn worktrunk_late_failure_rolls_back_new_worktree_branch_and_config() {
    struct LateFailureRunner {
        target: PathBuf,
        branch_exists: Cell<bool>,
        calls: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl ProcessRunner for LateFailureRunner {
        fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.calls
                .borrow_mut()
                .push((request.program.clone(), request.args.clone()));
            if request.args.iter().any(|arg| arg == "show-ref") {
                return if self.branch_exists.get() {
                    ok_output()
                } else {
                    fail_output("")
                };
            }
            if request.program == "wt" {
                std::fs::create_dir_all(self.target.join(".git")).unwrap();
                self.branch_exists.set(true);
                return fail_output("failed after creating checkout");
            }
            if request.args.iter().any(|arg| arg == "worktree") {
                std::fs::remove_dir_all(&self.target).unwrap();
                return ok_output();
            }
            if request.args.windows(2).any(|args| args == ["branch", "-D"]) {
                self.branch_exists.set(false);
                return ok_output();
            }
            panic!("unexpected request: {request:?}");
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!()
        }
    }

    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path();
    let worktrees_root = project_root.join("ito-worktrees");
    let target = worktrees_root.join("my-change");
    let main_root = project_root.join("main");
    std::fs::create_dir_all(&main_root).unwrap();
    let worktrunk_config = project_root.join(".ito/worktrunk/worktree-path.toml");
    std::fs::create_dir_all(worktrunk_config.parent().unwrap()).unwrap();
    std::fs::write(&worktrunk_config, "original = true\n").unwrap();
    let runner = LateFailureRunner {
        target: target.clone(),
        branch_exists: Cell::new(false),
        calls: RefCell::new(Vec::new()),
    };

    let error = ensure_worktree_with_runner(
        &runner,
        &AlwaysReady,
        "my-change",
        &make_embedded_config(),
        &make_env(project_root),
        &make_enabled_paths(worktrees_root, main_root),
        project_root,
    )
    .unwrap_err();

    assert!(error.to_string().contains("failed after creating checkout"));
    assert!(!target.exists());
    assert!(!runner.branch_exists.get());
    assert_eq!(
        std::fs::read_to_string(worktrunk_config).unwrap(),
        "original = true\n"
    );
    let calls = runner.calls.borrow();
    assert!(
        calls
            .iter()
            .any(|(_, args)| args.windows(2).any(|pair| pair == ["worktree", "remove"]))
    );
    assert!(
        calls
            .iter()
            .any(|(_, args)| args.windows(2).any(|pair| pair == ["branch", "-D"]))
    );
}
