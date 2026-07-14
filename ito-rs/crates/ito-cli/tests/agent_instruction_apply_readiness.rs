#[path = "support/mod.rs"]
mod fixtures;

use std::path::Path;
#[cfg(unix)]
use std::path::PathBuf;
use std::process::Command;

use ito_test_support::{CmdOutput, run_rust_candidate};
use serde_json::Value;
#[cfg(unix)]
use serde_json::json;

const CHANGE_ID: &str = "031-02_enforce-main-first-implementation";
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
fn direct_apply_json_rejects_complete_local_only_artifacts() {
    let repository = init_direct_merge_repository();
    let authority_oid = git_stdout(repository.path(), &["rev-parse", "HEAD"]);
    write_change(repository.path());
    let home = tempfile::tempdir().expect("home");

    let output = run_apply(repository.path(), home.path(), &[]);

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["change_id"], CHANGE_ID);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], false);
    assert_eq!(report["authority"]["oid"], authority_oid);
    assert!(report.get("changeName").is_none());
    let condition = assert_condition(&report, "change_target", false);
    assert!(
        !condition["remediation"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
}

#[test]
#[cfg(unix)]
fn direct_apply_sync_rejects_before_coordination_fetch() {
    let repository = tempfile::tempdir().expect("repository");
    let home = tempfile::tempdir().expect("home");
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    let _remote = setup_worktree_backed_local_only_repository(repository.path(), home.path(), ito);
    let fake_git = tempfile::tempdir().expect("fake git");
    let fetch_log = fake_git.path().join("fetch.log");
    let real_git = real_git_path();
    write_fetch_logging_git(fake_git.path(), &real_git);

    let output = run_candidate_with_fetch_logging(
        ito,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            CHANGE_ID,
            "--json",
            "--sync",
        ],
        repository.path(),
        home.path(),
        fake_git.path(),
        &fetch_log,
        &real_git,
    );

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["ready"], false);
    assert_condition(&report, "change_target", false);
    let fetches = std::fs::read_to_string(&fetch_log).unwrap_or_default();
    assert!(
        fetches.trim().is_empty(),
        "readiness rejection must precede coordination fetches: {fetches:?}"
    );
}

#[test]
fn direct_apply_json_succeeds_for_committed_authoritative_proposal() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    commit_all(repository.path(), "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");

    let output = run_apply(repository.path(), home.path(), &[]);

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    let response: Value = serde_json::from_str(&output.stdout).expect("apply JSON response");
    assert_eq!(response["changeName"], CHANGE_ID);
    assert!(response.get("ready").is_none());
}

#[test]
fn direct_apply_resolves_authoritative_change_prefix_before_rendering() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    commit_all(repository.path(), "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");

    let output = run_apply_change(repository.path(), home.path(), "031-02");

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    let response: Value = serde_json::from_str(&output.stdout).expect("apply JSON response");
    assert_eq!(response["changeName"], CHANGE_ID);
}

#[test]
fn direct_apply_renders_captured_authority_instead_of_local_tampering() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    commit_all(repository.path(), "integrate reviewed proposal");
    let change = repository.path().join(".ito/changes").join(CHANGE_ID);
    fixtures::write(
        change.join("tasks.md"),
        &TASKS.replace(
            "Implement the accepted proposal",
            "TAMPERED LOCAL IMPLEMENTATION",
        ),
    );
    let home = tempfile::tempdir().expect("home");

    let output = run_apply(repository.path(), home.path(), &[]);

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    assert!(output.stdout.contains("Implement the accepted proposal"));
    assert!(!output.stdout.contains("TAMPERED LOCAL IMPLEMENTATION"));
    let response: Value = serde_json::from_str(&output.stdout).expect("apply JSON response");
    assert_eq!(response["changeDir"], format!(".ito/changes/{CHANGE_ID}"));
    assert_eq!(
        response["tracksPath"],
        format!(".ito/changes/{CHANGE_ID}/tasks.md")
    );
    for path in response["contextFiles"]
        .as_object()
        .expect("context files")
        .values()
    {
        let path = path.as_str().expect("context path");
        assert!(path.starts_with(&format!(".ito/changes/{CHANGE_ID}/")));
        assert!(!path.starts_with(repository.path().to_str().unwrap()));
    }
}

#[test]
fn direct_apply_overlays_live_progress_from_execute_ready_worktree() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    let integration_oid = commit_all(repository.path(), "integrate reviewed proposal");
    let worktree_root = tempfile::tempdir().expect("worktree root");
    let worktree = worktree_root.path().join(CHANGE_ID);
    run_git(
        repository.path(),
        &[
            "worktree",
            "add",
            "-b",
            CHANGE_ID,
            worktree.to_str().unwrap(),
            &integration_oid,
        ],
    );
    fixtures::write(
        worktree
            .join(".ito/changes")
            .join(CHANGE_ID)
            .join("tasks.md"),
        &TASKS.replace("[ ] pending", "[x] complete"),
    );
    let home = tempfile::tempdir().expect("home");

    let output = run_apply(&worktree, home.path(), &[]);

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    let response: Value = serde_json::from_str(&output.stdout).expect("apply JSON response");
    assert_eq!(response["state"], "all_done");
    assert_eq!(response["progress"]["total"], 1);
    assert_eq!(response["progress"]["complete"], 1);
    assert_eq!(response["progress"]["remaining"], 0);
}

#[test]
fn direct_apply_rejects_schema_override() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    commit_all(repository.path(), "integrate reviewed proposal");
    let home = tempfile::tempdir().expect("home");

    let output = run_apply(repository.path(), home.path(), &["--schema", "basic"]);

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("do not accept --schema"));
    assert!(output.stderr.contains("authoritative .ito.yaml"));
}

#[test]
fn manifesto_explicit_apply_rejects_complete_local_only_artifacts() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    let home = tempfile::tempdir().expect("home");
    let ito = assert_cmd::cargo::cargo_bin!("ito");

    let output = run_rust_candidate(
        ito,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            CHANGE_ID,
            "--variant",
            "full",
            "--operation",
            "apply",
            "--json",
        ],
        repository.path(),
        home.path(),
    );

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["change_id"], CHANGE_ID);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], false);
    assert!(report.get("artifact").is_none());
    assert_condition(&report, "change_target", false);
}

#[test]
fn manifesto_light_default_profile_requires_prepare_readiness() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    let home = tempfile::tempdir().expect("home");
    let ito = assert_cmd::cargo::cargo_bin!("ito");

    let output = run_rust_candidate(
        ito,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            CHANGE_ID,
            "--json",
        ],
        repository.path(),
        home.path(),
    );

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], false);
    assert_condition(&report, "change_target", false);
}

#[test]
fn manifesto_full_profile_requires_prepare_readiness_without_operation_selector() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    let home = tempfile::tempdir().expect("home");
    let ito = assert_cmd::cargo::cargo_bin!("ito");

    let output = run_rust_candidate(
        ito,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            CHANGE_ID,
            "--variant",
            "full",
            "--profile",
            "full",
            "--json",
        ],
        repository.path(),
        home.path(),
    );

    assert_eq!(output.code, 1, "stderr={}", output.stderr);
    assert!(output.stderr.is_empty(), "stderr={}", output.stderr);
    let report = parse_pure_json(&output);
    assert_eq!(report["phase"], "prepare");
    assert_eq!(report["ready"], false);
    assert_condition(&report, "change_target", false);
}

#[test]
fn manifesto_proposal_only_remains_available_before_integration() {
    let repository = init_direct_merge_repository();
    write_change(repository.path());
    let home = tempfile::tempdir().expect("home");
    let ito = assert_cmd::cargo::cargo_bin!("ito");

    let output = run_rust_candidate(
        ito,
        &[
            "agent",
            "instruction",
            "manifesto",
            "--change",
            CHANGE_ID,
            "--profile",
            "proposal-only",
            "--json",
        ],
        repository.path(),
        home.path(),
    );

    assert_eq!(output.code, 0, "stderr={}", output.stderr);
    let response: Value = serde_json::from_str(&output.stdout).expect("manifesto JSON response");
    assert_eq!(response["artifact"], "manifesto");
    assert_eq!(response["profile"], "proposal-only");
    assert!(
        response["instruction"]
            .as_str()
            .unwrap_or_default()
            .contains("Active profile: `proposal-only`")
    );
}

fn init_direct_merge_repository() -> tempfile::TempDir {
    let repository = fixtures::make_empty_repo();
    init_git_repository(repository.path());
    write_direct_merge_config(repository.path());
    commit_all(repository.path(), "configure direct-merge authority");
    repository
}

fn init_git_repository(repository: &Path) {
    run_git(repository, &["init", "--initial-branch=main"]);
    run_git(repository, &["config", "user.name", "Ito Test"]);
    run_git(repository, &["config", "user.email", "ito@example.invalid"]);
    run_git(repository, &["config", "commit.gpgsign", "false"]);
    commit_all(repository, "initial fixture");
}

fn write_direct_merge_config(repository: &Path) {
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
}

fn write_change(repository: &Path) {
    let change = repository.join(".ito/changes").join(CHANGE_ID);
    fixtures::write(change.join(".ito.yaml"), "schema: spec-driven\n");
    fixtures::write(change.join("proposal.md"), PROPOSAL);
    fixtures::write(change.join("design.md"), DESIGN);
    fixtures::write(change.join("tasks.md"), TASKS);
    fixtures::write(change.join("specs/main-first/spec.md"), DELTA_SPEC);
}

fn run_apply(repository: &Path, home: &Path, extra_args: &[&str]) -> CmdOutput {
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    let mut args = vec![
        "agent",
        "instruction",
        "apply",
        "--change",
        CHANGE_ID,
        "--json",
    ];
    args.extend_from_slice(extra_args);
    run_rust_candidate(ito, &args, repository, home)
}

fn run_apply_change(repository: &Path, home: &Path, change: &str) -> CmdOutput {
    let ito = assert_cmd::cargo::cargo_bin!("ito");
    run_rust_candidate(
        ito,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            change,
            "--json",
        ],
        repository,
        home,
    )
}

fn parse_pure_json(output: &CmdOutput) -> Value {
    assert!(output.stdout.trim_start().starts_with('{'));
    assert!(output.stdout.trim_end().ends_with('}'));
    serde_json::from_str(&output.stdout).expect("stdout should contain one JSON document")
}

fn assert_condition<'a>(report: &'a Value, code: &str, passed: bool) -> &'a Value {
    report["conditions"]
        .as_array()
        .expect("conditions array")
        .iter()
        .find(|condition| condition["code"] == code && condition["passed"] == passed)
        .unwrap_or_else(|| panic!("missing condition '{code}' with passed={passed}: {report:#}"))
}

#[cfg(unix)]
fn setup_worktree_backed_local_only_repository(
    repository: &Path,
    home: &Path,
    ito: &Path,
) -> tempfile::TempDir {
    fixtures::write(repository.join("README.md"), "fixture\n");
    init_git_repository(repository);
    let remote = fixtures::make_bare_remote();
    fixtures::add_origin(repository, remote.path());
    run_git(repository, &["push", "origin", "HEAD:main"]);
    fixtures::write(
        repository.join(".ito/config.json"),
        r#"{
  "backend": {
    "project": { "org": "testorg", "repo": "testrepo" }
  }
}"#,
    );
    let initialized = run_rust_candidate(
        ito,
        &[
            "init",
            repository.to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--update",
        ],
        repository,
        home,
    );
    assert_eq!(
        initialized.code, 0,
        "init failed: stderr={} stdout={}",
        initialized.stderr, initialized.stdout
    );
    set_direct_merge_in_existing_config(repository);
    write_change(repository);
    remote
}

#[cfg(unix)]
fn set_direct_merge_in_existing_config(repository: &Path) {
    let path = repository.join(".ito/config.json");
    let contents = std::fs::read_to_string(&path).expect("Ito config");
    let mut config: Value = serde_json::from_str(&contents).expect("valid Ito config");
    let root = config.as_object_mut().expect("config object");
    let changes = root.entry("changes").or_insert_with(|| json!({}));
    let changes = changes.as_object_mut().expect("changes object");
    let proposal = changes.entry("proposal").or_insert_with(|| json!({}));
    let proposal = proposal.as_object_mut().expect("proposal object");
    proposal.insert("integration_mode".to_string(), json!("direct_merge"));
    fixtures::write(
        path,
        &(serde_json::to_string_pretty(&config).expect("serialize config") + "\n"),
    );
}

#[cfg(unix)]
fn real_git_path() -> PathBuf {
    let Some(path) = std::env::var_os("PATH") else {
        panic!("PATH is not set");
    };
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join("git");
        if candidate.is_file() {
            return candidate;
        }
    }
    panic!("git executable not found on PATH");
}

#[cfg(unix)]
fn write_fetch_logging_git(fake_git_dir: &Path, _real_git: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let script = fake_git_dir.join("git");
    fixtures::write(
        &script,
        "#!/bin/sh\nif [ \"$1\" = \"fetch\" ]; then\n  printf '%s\\n' \"$*\" >> \"$GIT_FETCH_LOG\"\nfi\nexec \"$REAL_GIT\" \"$@\"\n",
    );
    let mut permissions = std::fs::metadata(&script)
        .expect("fake git metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&script, permissions).expect("chmod fake git");
}

#[cfg(unix)]
#[allow(clippy::too_many_arguments)]
fn run_candidate_with_fetch_logging(
    program: &Path,
    args: &[&str],
    cwd: &Path,
    home: &Path,
    fake_git_dir: &Path,
    fetch_log: &Path,
    real_git: &Path,
) -> CmdOutput {
    let path = std::env::var_os("PATH").unwrap_or_default();
    let path = std::env::join_paths(
        std::iter::once(fake_git_dir.to_path_buf()).chain(std::env::split_paths(&path)),
    )
    .expect("join PATH");
    let mut command = ito_test_support::rust_candidate_command(program);
    command
        .args(args)
        .current_dir(cwd)
        .env("CI", "1")
        .env("NO_COLOR", "1")
        .env("ITO_INTERACTIVE", "0")
        .env("TERM", "dumb")
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_DATA_HOME", home)
        .env("PATH", path)
        .env("REAL_GIT", real_git)
        .env("GIT_FETCH_LOG", fetch_log)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_QUARANTINE_PATH")
        .env_remove("GIT_PREFIX");
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("failed to execute {command:?}: {error}"));
    CmdOutput {
        code: output.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

fn commit_all(repository: &Path, message: &str) -> String {
    run_git(repository, &["add", "."]);
    run_git(repository, &["commit", "--no-verify", "-m", message]);
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
