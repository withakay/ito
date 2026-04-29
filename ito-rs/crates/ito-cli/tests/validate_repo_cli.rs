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
        "coordination/branch-name-set",
        "coordination/gitignore-entries",
        "coordination/staged-symlinked-paths",
        "coordination/symlinks-wired",
        "worktrees/layout-consistent",
        "worktrees/no-write-on-control",
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
    assert_eq!(rules.len(), 6, "expected 6 built-in rules, got {rules:#?}");
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
