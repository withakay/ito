//! Integration tests for `ito validate repo`.
//!
//! Cover the user-visible CLI surface added by change 011-05:
//!
//! - Subcommand discoverable via `ito validate --help`.
//! - `--list-rules` enumerates all built-in rules with active flag and gate.
//! - `--explain <id>` prints rule metadata.
//! - `--json` emits the standard `ValidationReport` envelope.
//! - Documented exit codes: 0 (clean), 1 (rule errors), 2 (usage / config).

use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

/// Minimal initialised project: `.ito/` directory with an empty config file.
/// Sufficient for `ito validate repo --list-rules` and similar non-rule paths.
fn make_minimal_project() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("project");
    write(td.path().join("README.md"), "# tmp\n");
    write(td.path().join(".ito/config.json"), "{}\n");
    td
}

#[test]
fn validate_help_lists_repo_subcommand() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--help"],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "validate --help should succeed");
    assert!(
        out.stdout.contains("repo"),
        "validate --help should mention `repo`; got:\n{}",
        out.stdout,
    );
}

#[test]
fn validate_repo_help_lists_documented_flags() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--help"],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    for flag in &[
        "--staged",
        "--strict",
        "--json",
        "--rule",
        "--no-rule",
        "--list-rules",
        "--explain",
    ] {
        assert!(
            out.stdout.contains(flag),
            "validate repo --help should document `{flag}`; got:\n{}",
            out.stdout,
        );
    }
}

#[test]
fn validate_repo_list_rules_enumerates_built_in_rules() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--list-rules"],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "--list-rules must exit 0");
    for id in &[
        // 011-05
        "coordination/branch-name-set",
        "coordination/gitignore-entries",
        "coordination/staged-symlinked-paths",
        "coordination/symlinks-wired",
        "worktrees/layout-consistent",
        "worktrees/no-write-on-control",
        // 011-06
        "audit/mirror-branch-distinct-from-coordination",
        "audit/mirror-branch-set",
        "backend/project-org-repo-set",
        "backend/token-not-committed",
        "backend/url-scheme-valid",
        "repository/sqlite-db-not-committed",
        "repository/sqlite-db-path-set",
    ] {
        assert!(
            out.stdout.contains(id),
            "--list-rules should mention `{id}`; got:\n{}",
            out.stdout,
        );
    }
}

#[test]
fn validate_repo_list_rules_json_returns_array() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--list-rules", "--json"],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("--list-rules --json");
    let rules = v
        .get("rules")
        .and_then(|r| r.as_array())
        .expect("rules array");
    // Exact count is intentional: when more rules are added, this
    // assertion fails loudly so the test (and any docs that quote the
    // rule count) get updated together. Currently 13 rules ship: 6 from
    // 011-05 + 7 from 011-06.
    assert_eq!(
        rules.len(),
        13,
        "expected 13 built-in rules, got {rules:#?}"
    );
}

#[test]
fn validate_repo_explain_prints_metadata() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &[
            "validate",
            "repo",
            "--explain",
            "coordination/branch-name-set",
        ],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("rule: coordination/branch-name-set"));
    assert!(out.stdout.contains("severity: WARNING"));
    assert!(out.stdout.contains("description:"));
}

#[test]
fn validate_repo_explain_unknown_rule_exits_2() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--explain", "bogus/rule"],
        project.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 2,
        "unknown rule must exit 2; stdout: {}",
        out.stdout
    );
}

#[test]
fn validate_repo_rule_and_no_rule_mutually_exclusive_exit_2() {
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &[
            "validate",
            "repo",
            "--rule",
            "coordination/branch-name-set",
            "--no-rule",
            "coordination/branch-name-set",
        ],
        project.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 2,
        "exclusive flags must exit 2; stderr: {}",
        out.stderr
    );
}

#[test]
fn validate_repo_json_emits_validation_report_envelope() {
    // Configure embedded storage + worktrees disabled so all gated rules
    // are inactive; only `coordination/branch-name-set` runs and the
    // default branch name (`ito/internal/changes`) passes it.
    let project = tempfile::tempdir().expect("project");
    write(
        project.path().join(".ito/config.json"),
        r#"{
  "changes": { "coordination_branch": { "storage": "embedded" } },
  "worktrees": { "enabled": false }
}
"#,
    );

    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--json"],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "clean run must exit 0; stdout: {}", out.stdout);

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("--json output");
    assert_eq!(v.get("valid"), Some(&serde_json::Value::Bool(true)));
    assert!(
        v.get("issues").and_then(|i| i.as_array()).map(Vec::len) == Some(0),
        "expected empty issues array; got: {v:#?}",
    );
    let summary = v.get("summary").expect("summary");
    assert_eq!(summary.get("errors"), Some(&serde_json::json!(0)));
    assert_eq!(summary.get("warnings"), Some(&serde_json::json!(0)));
    assert_eq!(summary.get("info"), Some(&serde_json::json!(0)));
}

#[test]
fn validate_repo_explain_audit_mirror_distinct_rule() {
    // 011-06: explain output for one of the new rules, demonstrating
    // that the registry surfaces description and gate metadata.
    let project = make_minimal_project();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &[
            "validate",
            "repo",
            "--explain",
            "audit/mirror-branch-distinct-from-coordination",
        ],
        project.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("severity: ERROR"));
    assert!(out.stdout.contains("audit.mirror.enabled"));
    assert!(out.stdout.contains("coordination_branch.storage"));
}

#[test]
fn validate_repo_backend_token_not_committed_fails_when_token_in_config() {
    // 011-06 security check: write a project that has backend.enabled =
    // true and a backend.token set in `.ito/config.json`. The CLI
    // command should exit 1 with the token-not-committed rule firing.
    let project = tempfile::tempdir().expect("project");
    write(
        project.path().join(".ito/config.json"),
        r#"{
  "changes": { "coordination_branch": { "storage": "embedded" } },
  "worktrees": { "enabled": false },
  "backend": {
    "enabled": true,
    "url": "https://api.example.com",
    "token": "leaked-secret",
    "project": { "org": "withakay", "repo": "ito" }
  }
}
"#,
    );
    // Initialise a real git repo so `git ls-files --error-unmatch` can
    // actually classify the config file as tracked.
    let _ = std::process::Command::new("git")
        .args(["init", "--initial-branch=main"])
        .current_dir(project.path())
        .output();
    let _ = std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(project.path())
        .output();
    let _ = std::process::Command::new("git")
        .args(["config", "user.name", "test"])
        .current_dir(project.path())
        .output();
    let _ = std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(project.path())
        .output();
    let _ = std::process::Command::new("git")
        .args(["add", ".ito/config.json"])
        .current_dir(project.path())
        .output();
    let _ = std::process::Command::new("git")
        .args(["commit", "--no-verify", "-m", "init"])
        .current_dir(project.path())
        .output();

    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--json"],
        project.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 1,
        "expected exit 1 (validation failure); stdout: {}",
        out.stdout,
    );

    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("--json output");
    let issues = v
        .get("issues")
        .and_then(|i| i.as_array())
        .expect("issues array");
    let token_issues: Vec<_> = issues
        .iter()
        .filter(|i| {
            i.get("rule_id").and_then(|r| r.as_str()) == Some("backend/token-not-committed")
        })
        .collect();
    assert_eq!(
        token_issues.len(),
        1,
        "expected one token-not-committed issue; got: {issues:#?}",
    );
    assert_eq!(
        token_issues[0].get("level").and_then(|l| l.as_str()),
        Some("ERROR"),
    );
}

#[test]
fn validate_repo_url_scheme_valid_fails_for_non_http_scheme() {
    // 011-06: enabling the backend with an ftp URL should fail with the
    // url-scheme-valid rule.
    let project = tempfile::tempdir().expect("project");
    write(
        project.path().join(".ito/config.json"),
        r#"{
  "changes": { "coordination_branch": { "storage": "embedded" } },
  "worktrees": { "enabled": false },
  "backend": {
    "enabled": true,
    "url": "ftp://files.example.com",
    "project": { "org": "withakay", "repo": "ito" }
  }
}
"#,
    );

    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["validate", "repo"],
        project.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 1,
        "expected exit 1 for ftp scheme; stdout: {}",
        out.stdout,
    );
    assert!(
        out.stdout.contains("backend/url-scheme-valid"),
        "stdout should mention the failing rule; got:\n{}",
        out.stdout,
    );
}

#[test]
fn validate_repo_strict_promotes_branch_name_warning_to_failure() {
    // Non-conventional coordination branch name + embedded storage =>
    // only `coordination/branch-name-set` fires (a WARNING). Without
    // strict, exit 0; with strict, exit 1.
    let project = tempfile::tempdir().expect("project");
    write(
        project.path().join(".ito/config.json"),
        r#"{
  "changes": {
    "coordination_branch": {
      "storage": "embedded",
      "name": "coordination/foo"
    }
  },
  "worktrees": { "enabled": false }
}
"#,
    );

    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let lenient = run_rust_candidate(
        rust_path,
        &["validate", "repo"],
        project.path(),
        home.path(),
    );
    assert_eq!(
        lenient.code, 0,
        "warning should not fail without --strict; stdout: {}",
        lenient.stdout,
    );

    let strict = run_rust_candidate(
        rust_path,
        &["validate", "repo", "--strict"],
        project.path(),
        home.path(),
    );
    assert_eq!(
        strict.code, 1,
        "warning should fail with --strict; stdout: {}",
        strict.stdout,
    );
}
