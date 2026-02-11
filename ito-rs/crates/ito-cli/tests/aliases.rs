use ito_test_support::run_rust_candidate;
use std::path::Path;

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
fn main_command_aliases_work() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test main command aliases with --help to verify resolution
    // ls -> list
    let out = run_rust_candidate(rust_path, &["ls", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "ls alias should work");
    assert!(
        out.stdout.contains("List changes"),
        "ls should resolve to list command"
    );

    // cr -> create
    let out = run_rust_candidate(rust_path, &["cr", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "cr alias should work");
    assert!(
        out.stdout.contains("Create a new module"),
        "cr should resolve to create command"
    );

    // st -> status
    let out = run_rust_candidate(rust_path, &["st", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "st alias should work");
    assert!(
        out.stdout.contains("Check completion status"),
        "st should resolve to status command"
    );

    // sh -> show
    let out = run_rust_candidate(rust_path, &["sh", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "sh alias should work");
    assert!(
        out.stdout.contains("Display details"),
        "sh should resolve to show command"
    );

    // va -> validate
    let out = run_rust_candidate(rust_path, &["va", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "va alias should work");
    assert!(
        out.stdout.contains("Check changes"),
        "va should resolve to validate command"
    );
}

#[test]
fn main_command_aliases_execute() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test actual command execution with aliases
    // ls -> list
    let out = run_rust_candidate(rust_path, &["ls", "--json"], repo.path(), home.path());
    assert_eq!(out.code, 0, "ls --json should execute successfully");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("ls should return valid json");
    assert!(v.get("changes").is_some(), "ls should list changes");

    // st -> status
    let out = run_rust_candidate(
        rust_path,
        &["st", "--change", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "st should execute successfully");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("st should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "st should show status for change"
    );
}

#[test]
fn subcommand_aliases_work() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test create subcommand aliases
    // cr mo -> create module
    let out = run_rust_candidate(rust_path, &["cr", "mo", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "create module alias should work");
    assert!(
        out.stdout.contains("Create a module"),
        "mo should resolve to module"
    );

    // cr ch -> create change
    let out = run_rust_candidate(rust_path, &["cr", "ch", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "create change alias should work");
    assert!(
        out.stdout.contains("Create a change"),
        "ch should resolve to change"
    );
}

#[test]
fn short_flags_work() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Test -c short flag for --change
    let out = run_rust_candidate(
        rust_path,
        &["status", "-c", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "-c flag should work for status");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("status should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "-c should work as alias for --change"
    );

    // Test combining alias and short flag
    let out = run_rust_candidate(
        rust_path,
        &["st", "-c", "000-01_test-change", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "st -c combination should work");
    let v: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("status should return valid json");
    assert_eq!(
        v.get("changeName").and_then(|v| v.as_str()),
        Some("000-01_test-change"),
        "combining alias and short flag should work"
    );
}
