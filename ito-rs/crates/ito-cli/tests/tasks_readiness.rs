#[path = "support/mod.rs"]
mod fixtures;

use std::path::Path;
use std::process::Command;

use ito_test_support::{CmdOutput, run_rust_candidate};
use serde_json::Value;

const CHANGE_ID: &str = "031-02_enforce-main-first-implementation";
const PROPOSAL: &str = "# Proposal\n\nIntegrate reviewed intent before task execution.\n";
const DESIGN: &str = "# Design\n\nGate task mutations on the immutable authority snapshot.\n";
const DELTA_SPEC: &str = r#"## ADDED Requirements

### Requirement: Task mutation readiness
Ito SHALL reject implementation task mutations before the proposal is accepted on main.

#### Scenario: Local-only proposal
- **GIVEN** a complete proposal copied into an implementation checkout
- **WHEN** a task mutation is requested
- **THEN** task state remains unchanged
"#;
const PENDING_TASKS: &str = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Implement the accepted proposal
- **Dependencies**: None
- **Updated At**: 2026-07-13
- **Status**: [ ] pending
"#;
const IN_PROGRESS_TASKS: &str = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Implement the accepted proposal
- **Dependencies**: None
- **Updated At**: 2026-07-13
- **Status**: [>] in-progress
"#;

#[test]
fn task_start_rejects_local_only_proposal_without_mutating_state() {
    assert_failed_mutation_preserves_tasks("start", PENDING_TASKS);
}

#[test]
fn task_complete_rejects_local_only_proposal_without_mutating_state() {
    assert_failed_mutation_preserves_tasks("complete", IN_PROGRESS_TASKS);
}

#[test]
fn task_start_and_complete_succeed_after_main_integration() {
    let repository = init_repository();
    write_change(repository.path(), PENDING_TASKS);
    commit_all(repository.path(), "integrate reviewed proposal");
    run_git(repository.path(), &["switch", "-c", CHANGE_ID]);
    let home = tempfile::tempdir().expect("home");

    let started = run_task(repository.path(), home.path(), "start", false);
    assert_eq!(started.code, 0, "stderr={}", started.stderr);
    let tasks_path = change_dir(repository.path()).join("tasks.md");
    let after_start = std::fs::read_to_string(&tasks_path).expect("tasks after start");
    assert!(after_start.contains("[>] in-progress"), "{after_start}");

    let completed = run_task(repository.path(), home.path(), "complete", false);
    assert_eq!(completed.code, 0, "stderr={}", completed.stderr);
    let after_complete = std::fs::read_to_string(tasks_path).expect("tasks after complete");
    assert!(after_complete.contains("[x] complete"), "{after_complete}");
}

fn assert_failed_mutation_preserves_tasks(action: &str, tasks: &str) {
    let repository = init_repository();
    run_git(repository.path(), &["switch", "-c", CHANGE_ID]);
    write_change(repository.path(), tasks);
    let tasks_path = change_dir(repository.path()).join("tasks.md");
    let before = std::fs::read(&tasks_path).expect("tasks before mutation");
    let home = tempfile::tempdir().expect("home");

    let output = run_task(repository.path(), home.path(), action, true);

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report: Value = serde_json::from_str(&output.stdout).expect("readiness JSON");
    assert_eq!(report["phase"], "execute");
    assert_eq!(report["ready"], false);
    assert_eq!(
        std::fs::read(tasks_path).expect("tasks after rejected mutation"),
        before
    );
}

fn init_repository() -> tempfile::TempDir {
    let repository = fixtures::make_empty_repo();
    run_git(repository.path(), &["init", "--initial-branch=main"]);
    run_git(repository.path(), &["config", "user.name", "Ito Test"]);
    run_git(
        repository.path(),
        &["config", "user.email", "ito@example.invalid"],
    );
    run_git(repository.path(), &["config", "commit.gpgsign", "false"]);
    fixtures::write(
        repository.path().join(".ito/config.json"),
        r#"{
  "changes": {
    "proposal": { "integration_mode": "direct_merge" }
  },
  "worktrees": {
    "enabled": true,
    "default_branch": "main",
    "strategy": "checkout_siblings",
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    );
    commit_all(repository.path(), "configure direct-merge authority");
    repository
}

fn write_change(repository: &Path, tasks: &str) {
    let change = change_dir(repository);
    fixtures::write(change.join(".ito.yaml"), "schema: spec-driven\n");
    fixtures::write(change.join("proposal.md"), PROPOSAL);
    fixtures::write(change.join("design.md"), DESIGN);
    fixtures::write(change.join("tasks.md"), tasks);
    fixtures::write(change.join("specs/task-readiness/spec.md"), DELTA_SPEC);
}

fn change_dir(repository: &Path) -> std::path::PathBuf {
    repository.join(".ito/changes").join(CHANGE_ID)
}

fn run_task(repository: &Path, home: &Path, action: &str, json: bool) -> CmdOutput {
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    let mut args = vec!["tasks", action, CHANGE_ID, "1.1"];
    if json {
        args.push("--json");
    }
    run_rust_candidate(ito, &args, repository, home)
}

fn commit_all(repository: &Path, message: &str) {
    run_git(repository, &["add", "-A"]);
    run_git(
        repository,
        &["commit", "--no-gpg-sign", "--no-verify", "-m", message],
    );
}

fn run_git(repository: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repository)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}
