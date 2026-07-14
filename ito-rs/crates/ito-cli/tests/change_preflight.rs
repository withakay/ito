#[path = "support/mod.rs"]
mod fixtures;

use std::path::{Path, PathBuf};
use std::process::Command;

use ito_test_support::{CmdOutput, run_rust_candidate};
use serde_json::Value;

const CHANGE_ID: &str = "031-02_enforce-main-first-implementation";
const MISSING_CHANGE_ID: &str = "031-03_missing-reviewed-change";
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
fn prepare_json_passes_with_exact_authority_and_integration_oids() {
    let (_fixture, repository) = init_repository();
    write_change(&repository);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");

    let output = run_preflight(&repository, home.path(), CHANGE_ID, "prepare");

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["change_id"], CHANGE_ID);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], true);
    assert_eq!(report["authority"]["integration_mode"], "direct_merge");
    assert_eq!(report["authority"]["target_ref"], "refs/heads/main");
    assert_eq!(report["authority"]["oid"], integration_oid);
    assert_eq!(report["proposal_integration_oid"], integration_oid);
    assert_condition(&report, "authoritative_artifacts", true);
    assert_condition(&report, "authoritative_validation", true);
    assert_condition(&report, "proposal_integration", true);
}

#[test]
fn prepare_json_reports_missing_authoritative_change_and_exits_nonzero() {
    let (_fixture, repository) = init_repository();
    let authority_oid = git_stdout(&repository, &["rev-parse", "HEAD"]);
    let home = tempfile::tempdir().expect("home");

    let output = run_preflight(&repository, home.path(), MISSING_CHANGE_ID, "prepare");

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["change_id"], MISSING_CHANGE_ID);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], false);
    assert_eq!(report["authority"]["oid"], authority_oid);
    assert!(report["proposal_integration_oid"].is_null());
    let condition = assert_condition(&report, "change_target", false);
    assert!(
        condition["message"]
            .as_str()
            .unwrap_or_default()
            .contains(MISSING_CHANGE_ID)
    );
    assert!(
        !condition["remediation"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
}

#[test]
fn prepare_json_uses_the_tracked_upstream_in_pull_request_mode() {
    let (fixture, repository) = init_repository();
    let config_path = repository.join(".ito/config.json");
    let config = std::fs::read_to_string(&config_path)
        .expect("config")
        .replace("direct_merge", "pull_request");
    fixtures::write(&config_path, &config);
    write_change(&repository);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let remote = fixture.path().join("authority.git");
    run_git(
        fixture.path(),
        &["init", "--bare", remote.to_str().expect("remote path")],
    );
    run_git(
        &repository,
        &[
            "remote",
            "add",
            "origin",
            remote.to_str().expect("remote path"),
        ],
    );
    run_git(&repository, &["push", "--set-upstream", "origin", "main"]);
    let home = tempfile::tempdir().expect("home");

    let output = run_preflight(&repository, home.path(), CHANGE_ID, "prepare");

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["authority"]["integration_mode"], "pull_request");
    assert_eq!(
        report["authority"]["target_ref"],
        "refs/remotes/origin/main"
    );
    assert_eq!(report["authority"]["oid"], integration_oid);
    assert_eq!(report["proposal_integration_oid"], integration_oid);
}

#[test]
fn execute_json_passes_from_post_integration_suffixed_worktree() {
    let (fixture, repository) = init_repository();
    write_change(&repository);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let branch = format!("{CHANGE_ID}-review");
    let worktree = add_linked_worktree(fixture.path(), &repository, &branch, &integration_oid);
    let home = tempfile::tempdir().expect("home");

    let output = run_preflight(&worktree, home.path(), CHANGE_ID, "execute");

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["phase"], "execute");
    assert_eq!(report["ready"], true);
    assert_eq!(report["authority"]["oid"], integration_oid);
    assert_eq!(report["proposal_integration_oid"], integration_oid);
    assert_condition(&report, "implementation_ancestry", true);
    assert_condition(&report, "checkout_identity", true);
}

#[test]
fn execute_json_rejects_authoritative_main_checkout() {
    let (_fixture, repository) = init_repository();
    write_change(&repository);
    let integration_oid = commit_all(&repository, "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");

    let output = run_preflight(&repository, home.path(), CHANGE_ID, "execute");

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["phase"], "execute");
    assert_eq!(report["ready"], false);
    assert_eq!(report["authority"]["oid"], integration_oid);
    assert_eq!(report["proposal_integration_oid"], integration_oid);
    assert_condition(&report, "implementation_ancestry", true);
    let condition = assert_condition(&report, "checkout_identity", false);
    assert!(
        condition["message"]
            .as_str()
            .unwrap_or_default()
            .contains("main")
    );
    assert!(
        !condition["remediation"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
}

fn init_repository() -> (tempfile::TempDir, PathBuf) {
    let fixture = tempfile::tempdir().expect("fixture root");
    let repository = fixture.path().join("repository");
    run_git(
        fixture.path(),
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
    "layout": { "dir_name": "ito-worktrees" }
  }
}"#,
    );
    commit_all(&repository, "initial fixture");
    (fixture, repository)
}

fn write_change(repository: &Path) {
    let change = repository.join(".ito/changes").join(CHANGE_ID);
    fixtures::write(change.join(".ito.yaml"), "schema: spec-driven\n");
    fixtures::write(change.join("proposal.md"), PROPOSAL);
    fixtures::write(change.join("design.md"), DESIGN);
    fixtures::write(change.join("tasks.md"), TASKS);
    fixtures::write(change.join("specs/main-first/spec.md"), DELTA_SPEC);
}

fn add_linked_worktree(
    fixture_root: &Path,
    repository: &Path,
    branch: &str,
    start_point: &str,
) -> PathBuf {
    let worktrees = fixture_root.join("ito-worktrees");
    std::fs::create_dir_all(&worktrees).expect("worktrees root");
    let worktree = worktrees.join(branch);
    run_git(
        repository,
        &[
            "worktree",
            "add",
            "-b",
            branch,
            worktree.to_str().expect("worktree path"),
            start_point,
        ],
    );
    worktree
}

fn run_preflight(cwd: &Path, home: &Path, change_id: &str, phase: &str) -> CmdOutput {
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    run_rust_candidate(
        ito,
        &["change", "preflight", change_id, "--for", phase, "--json"],
        cwd,
        home,
    )
}

fn parse_pure_json(output: &CmdOutput) -> Value {
    assert!(output.stdout.trim_start().starts_with('{'));
    assert!(output.stdout.trim_end().ends_with('}'));
    serde_json::from_str(&output.stdout).expect("stdout should contain only one JSON report")
}

fn assert_condition<'a>(report: &'a Value, code: &str, passed: bool) -> &'a Value {
    report["conditions"]
        .as_array()
        .expect("conditions array")
        .iter()
        .find(|condition| condition["code"] == code && condition["passed"] == passed)
        .unwrap_or_else(|| panic!("missing condition '{code}' with passed={passed}: {report:#}"))
}

fn commit_all(repository: &Path, message: &str) -> String {
    run_git(repository, &["add", "."]);
    run_git(repository, &["commit", "-m", message]);
    git_stdout(repository, &["rev-parse", "HEAD"])
}

fn run_git(repository: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(repository)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
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
    let output = Command::new("git")
        .args(["-c", "commit.gpgSign=false"])
        .args(args)
        .current_dir(repository)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
        .output()
        .expect("git should run");
    assert!(output.status.success(), "git {args:?} should succeed");
    String::from_utf8(output.stdout)
        .expect("git stdout should be UTF-8")
        .trim()
        .to_string()
}
