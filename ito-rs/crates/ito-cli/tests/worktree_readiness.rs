#[path = "support/mod.rs"]
mod fixtures;

use std::path::{Path, PathBuf};
use std::process::Command;

use ito_test_support::{CmdOutput, run_rust_candidate};

const CHANGE_ID: &str = "031-02_enforce-main-first-implementation";
const LOCAL_PROPOSAL_BRANCH: &str = "local-only-proposal";
const PROPOSAL: &str = "# Proposal\n\nIntegrate reviewed intent before implementation.\n";
const DESIGN: &str = "# Design\n\nResolve one immutable authority commit.\n";
const DELTA_SPEC: &str = r#"## ADDED Requirements

### Requirement: Main-first implementation
Ito SHALL require accepted proposal history before implementation begins.

#### Scenario: Accepted proposal
- **GIVEN** a reviewed proposal
- **WHEN** implementation readiness is evaluated
- **THEN** the accepted proposal commit is present
"#;
const TASKS: &str = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Implement the accepted proposal
- **Dependencies**: None
- **Updated At**: 2026-07-13
- **Status**: [ ] pending
"#;

#[test]
fn ensure_rejects_local_only_change_without_side_effects() {
    let fixture = init_repository();
    run_git(
        &fixture.repository,
        &["switch", "-c", LOCAL_PROPOSAL_BRANCH],
    );
    write_change(&fixture.repository);
    commit_all(&fixture.repository, "write local-only proposal");
    let expected_worktree = fixture.worktrees_root.join(CHANGE_ID);
    let home = tempfile::tempdir().expect("home");

    assert!(!fixture.worktrees_root.exists());
    assert!(!expected_worktree.exists());
    let before = capture_repository_state(&fixture.repository);

    let output = run_worktree_command(&fixture.repository, home.path(), "ensure");

    assert_eq!(
        output.code, 1,
        "stdout={}\nstderr={}",
        output.stdout, output.stderr
    );
    assert!(!fixture.worktrees_root.exists());
    assert!(!expected_worktree.exists());
    assert_eq!(capture_repository_state(&fixture.repository), before);
}

#[test]
fn setup_rejects_pre_integration_worktree_when_setup_is_unconfigured() {
    let fixture = init_repository();
    let before_integration = git_stdout(&fixture.repository, &["rev-parse", "HEAD"]);
    let worktree = add_change_worktree(&fixture, &before_integration);
    write_change(&fixture.repository);
    commit_all(&fixture.repository, "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");
    let before = capture_repository_state(&worktree);

    let output = run_worktree_command(&fixture.repository, home.path(), "setup");

    assert_eq!(
        output.code, 1,
        "stdout={}\nstderr={}",
        output.stdout, output.stderr
    );
    assert_eq!(capture_repository_state(&worktree), before);
}

#[test]
fn setup_accepts_existing_worktree_that_descends_from_proposal_integration() {
    let fixture = init_repository();
    write_change(&fixture.repository);
    let integration_oid = commit_all(&fixture.repository, "integrate reviewed proposal");
    let worktree = add_change_worktree(&fixture, &integration_oid);
    let home = tempfile::tempdir().expect("home");
    let before = capture_repository_state(&worktree);

    let output = run_worktree_command(&fixture.repository, home.path(), "setup");

    assert_eq!(
        output.code, 0,
        "stdout={}\nstderr={}",
        output.stdout, output.stderr
    );
    assert_eq!(capture_repository_state(&worktree), before);
}

struct RepositoryFixture {
    _root: tempfile::TempDir,
    repository: PathBuf,
    worktrees_root: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
struct RepositoryState {
    head: String,
    branch: String,
    status: String,
    refs: String,
    worktrees: String,
}

fn init_repository() -> RepositoryFixture {
    let root = tempfile::tempdir().expect("fixture root");
    let repository = root.path().join("repository");
    let worktrees_root = root.path().join("repository-ito-worktrees");
    run_git(
        root.path(),
        &[
            "init",
            "--initial-branch=main",
            repository.to_str().expect("repository path"),
        ],
    );
    run_git(&repository, &["config", "user.name", "Ito Test"]);
    run_git(
        &repository,
        &["config", "user.email", "ito@example.invalid"],
    );
    fixtures::write(repository.join("README.md"), "fixture\n");
    fixtures::write(
        repository.join(".ito/config.json"),
        r#"{
  "changes": {
    "proposal": { "integration_mode": "direct_merge" }
  },
  "worktrees": {
    "enabled": true,
    "default_branch": "main",
    "strategy": "checkout_siblings",
    "layout": { "dir_name": "ito-worktrees" },
    "init": {}
  }
}"#,
    );
    commit_all(&repository, "initial fixture");

    RepositoryFixture {
        _root: root,
        repository,
        worktrees_root,
    }
}

fn write_change(repository: &Path) {
    let change = repository.join(".ito/changes").join(CHANGE_ID);
    fixtures::write(change.join(".ito.yaml"), "schema: spec-driven\n");
    fixtures::write(change.join("proposal.md"), PROPOSAL);
    fixtures::write(change.join("design.md"), DESIGN);
    fixtures::write(change.join("tasks.md"), TASKS);
    fixtures::write(change.join("specs/main-first/spec.md"), DELTA_SPEC);
}

fn add_change_worktree(fixture: &RepositoryFixture, start_point: &str) -> PathBuf {
    std::fs::create_dir_all(&fixture.worktrees_root).expect("worktrees root");
    let worktree = fixture.worktrees_root.join(CHANGE_ID);
    run_git(
        &fixture.repository,
        &[
            "worktree",
            "add",
            "-b",
            CHANGE_ID,
            worktree.to_str().expect("worktree path"),
            start_point,
        ],
    );
    worktree
}

fn run_worktree_command(cwd: &Path, home: &Path, command: &str) -> CmdOutput {
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    run_rust_candidate(
        ito,
        &["worktree", command, "--change", CHANGE_ID],
        cwd,
        home,
    )
}

fn capture_repository_state(repository: &Path) -> RepositoryState {
    let status = git_stdout(
        repository,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )
    .lines()
    .filter(|line| !line.ends_with(".ito/session.json"))
    .collect::<Vec<_>>()
    .join("\n");
    RepositoryState {
        head: git_stdout(repository, &["rev-parse", "HEAD"]),
        branch: git_stdout(repository, &["symbolic-ref", "HEAD"]),
        status,
        refs: git_stdout(
            repository,
            &["for-each-ref", "--format=%(refname) %(objectname)"],
        ),
        worktrees: git_stdout(repository, &["worktree", "list", "--porcelain"]),
    }
}

fn commit_all(repository: &Path, message: &str) -> String {
    run_git(repository, &["add", "."]);
    run_git(repository, &["commit", "-m", message]);
    git_stdout(repository, &["rev-parse", "HEAD"])
}

fn run_git(repository: &Path, args: &[&str]) {
    let output = git_command(repository, args)
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git {args:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(repository: &Path, args: &[&str]) -> String {
    let output = git_command(repository, args)
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git {args:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git stdout should be UTF-8")
        .trim()
        .to_string()
}

fn git_command(repository: &Path, args: &[&str]) -> Command {
    let mut command = Command::new("git");
    command
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(repository)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR");
    command
}
