use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn make_base_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    // Minimal module.
    write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for ad-hoc changes. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Minimal spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Minimal change with one valid delta.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha Delta\nThe system SHALL include alpha delta behavior in strict validation.\n\n#### Scenario: Delta ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    td
}

fn reset_repo(dst: &Path, src: &Path) {
    ito_test_support::reset_dir(dst, src).unwrap();
}

#[test]
fn list_show_validate_smoke() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // list
    let out = run_rust_candidate(rust_path, &["list", "--json"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("list json");
    assert!(v.get("changes").is_some());

    // Verify completed field exists and is true (test change has 1/1 tasks done)
    let changes = v["changes"].as_array().expect("changes should be array");
    assert!(!changes.is_empty());
    let first_change = &changes[0];
    assert!(
        first_change.get("completed").is_some(),
        "completed field should exist"
    );
    assert_eq!(
        first_change["completed"].as_bool(),
        Some(true),
        "test change should be completed (1/1 tasks)"
    );

    // Test --completed filter returns the completed change
    let out = run_rust_candidate(
        rust_path,
        &["list", "--completed", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("list completed json");
    let completed_changes = v["changes"].as_array().expect("changes should be array");
    assert_eq!(
        completed_changes.len(),
        1,
        "should have exactly one completed change"
    );
    assert_eq!(
        completed_changes[0]["name"].as_str(),
        Some("000-01_test-change"),
        "completed change should be test-change"
    );

    let out = run_rust_candidate(
        rust_path,
        &["list", "--specs", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("list specs json");
    assert!(v.get("specs").is_some());

    let out = run_rust_candidate(
        rust_path,
        &["list", "--modules", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("list modules json");
    assert!(v.get("modules").is_some());

    // show
    let out = run_rust_candidate(
        rust_path,
        &["show", "--type", "spec", "alpha", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("show spec json");
    assert_eq!(v.get("id").and_then(|v| v.as_str()), Some("alpha"));

    let out = run_rust_candidate(
        rust_path,
        &["show", "--type", "change", "0-1", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("show change json");
    assert_eq!(
        v.get("id").and_then(|v| v.as_str()),
        Some("000-01_test-change")
    );
    assert!(v.get("deltas").is_some());

    let out = run_rust_candidate(
        rust_path,
        &["validate", "--type", "change", "0-1", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("validate change json");
    assert!(v.get("items").is_some());

    // validate
    let out = run_rust_candidate(
        rust_path,
        &["validate", "--all", "--strict", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("validate all json");
    assert!(v.get("items").is_some());
    assert!(v.get("summary").is_some());
}

#[test]
fn agent_instruction_status_archive_smoke() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // agent instruction apply
    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "apply",
            "--change",
            "000-01_test-change",
            "--json",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("agent instruction apply");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change")
    );

    // status
    let out = run_rust_candidate(
        rust_path,
        &["status", "--change", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("status json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change")
    );

    // archive (skip specs + validate to avoid interactive flows)
    let out = run_rust_candidate(
        rust_path,
        &["archive", "0-1", "--skip-specs", "--no-validate", "-y"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let archive_dir = repo
        .path()
        .join(".ito/changes/archive")
        .read_dir()
        .expect("archive dir should exist")
        .flatten()
        .find(|e| {
            e.file_name()
                .to_string_lossy()
                .contains("000-01_test-change")
        });
    assert!(archive_dir.is_some());
}

#[test]
fn create_workflow_plan_state_config_smoke() {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    write(repo.path().join("README.md"), "# temp\n");
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    // create module + change (hits ito-core create paths)
    let out = run_rust_candidate(
        rust_path,
        &["create", "module", "demo", "--scope", "*"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &[
            "create",
            "change",
            "hello",
            "--module",
            "000",
            "--schema",
            "spec-driven",
            "--description",
            "desc",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stderr.contains("Created files:"));
    assert!(out.stderr.contains(".ito.yaml"));
    assert!(out.stderr.contains("README.md"));
    assert!(out.stderr.contains("Module: 000 (from --module)"));
    assert!(out.stderr.contains("Next steps:"));
    assert!(
        out.stderr
            .contains("ito agent instruction proposal --change 000-01_hello")
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "create",
            "change",
            "hello-default",
            "--schema",
            "spec-driven",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stderr.contains("Created files:"));
    assert!(out.stderr.contains(".ito.yaml"));
    assert!(!out.stderr.contains("README.md"));
    assert!(out.stderr.contains("Module: 000 (default)"));
    assert!(out.stderr.contains("Next steps:"));

    // workflow
    let out = run_rust_candidate(rust_path, &["workflow", "init"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(rust_path, &["workflow", "list"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(
        rust_path,
        &["workflow", "show", "research"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    // plan + state
    let out = run_rust_candidate(rust_path, &["plan", "init"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(
        rust_path,
        &["state", "note", "hello"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    // config
    let out = run_rust_candidate(rust_path, &["config", "path"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(
        rust_path,
        &["config", "set", "alpha", "\"beta\""],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "alpha"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.trim() == "beta");
    let out = run_rust_candidate(
        rust_path,
        &["config", "unset", "alpha"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
}
