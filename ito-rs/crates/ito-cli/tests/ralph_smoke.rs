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
        "# Ungrouped\n\n## Purpose\nModule for Ralph tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Minimal spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Minimal change.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    td
}

fn write_complete_change(repo: &Path, change_id: &str) {
    write(
        repo.join(".ito/changes")
            .join(change_id)
            .join("proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    write(
        repo.join(".ito/changes").join(change_id).join("tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Done\n",
    );
    write(
        repo.join(".ito/changes")
            .join(change_id)
            .join("specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Delta\nThe system SHALL be testable.\n\n#### Scenario: Ok\n- **WHEN** run\n- **THEN** ok\n",
    );
}

fn reset_repo(dst: &Path, src: &Path) {
    ito_test_support::reset_dir(dst, src).unwrap();
}

#[test]
fn ralph_stub_harness_writes_state_and_status_works() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Status before first run.
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Run one iteration using stub harness (default step returns <promise>COMPLETE</promise>).
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "do",
            "work",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let state_path = repo
        .path()
        .join(".ito/.state/ralph/000-01_test-change/state.json");
    assert!(state_path.exists());

    // Status after run should mention iteration and history count.
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Iteration:"));
    assert!(out.stdout.contains("History entries:"));
}

#[test]
fn ralph_change_flag_supports_shorthand_resolution() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["ralph", "--change", "0-1", "--status", "--no-interactive"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Ralph Status for 000-01_test-change"));
}

#[test]
fn ralph_change_flag_supports_slug_query_resolution() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(
        repo.path()
            .join(".ito/changes/001-12_setup-wizard/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "setup wizard",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Ralph Status for 001-12_setup-wizard"));
}

#[test]
fn ralph_file_flag_requires_readable_file() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "missing-prompt.txt",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr
            .contains("Failed to read prompt file missing-prompt.txt")
    );
}

#[test]
fn ralph_file_flag_allowed_without_change_or_module() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "missing-prompt.txt",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr
            .contains("Failed to read prompt file missing-prompt.txt")
    );
}

#[test]
fn ralph_continue_ready_exits_successfully_when_all_changes_complete() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Ensure the base change is "complete" for work-status purposes.
    write(
        repo.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Delta\nThe system SHALL be testable.\n\n#### Scenario: Ok\n- **WHEN** run\n- **THEN** ok\n",
    );
    // Add a second complete change.
    write_complete_change(repo.path(), "000-02_other");

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--continue-ready",
            "--harness",
            "stub",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("All changes are complete."));
}

#[test]
fn ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Draft change blocks completion: tasks exist, but proposal/specs are missing.
    write(
        repo.path().join(".ito/changes/000-03_draft/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Todo\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--continue-ready",
            "--harness",
            "stub",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr.contains("no eligible changes"),
        "stderr={}",
        out.stderr
    );
    assert!(out.stderr.contains("000-03_draft"), "stderr={}", out.stderr);
}

#[test]
fn ralph_file_flag_runs_without_change_or_module() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());
    write(repo.path().join("prompt.md"), "do work\n");

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--harness",
            "stub",
            "--no-commit",
            "--no-interactive",
            "--skip-validation",
            "--min-iterations",
            "1",
            "--max-iterations",
            "1",
            "--file",
            "prompt.md",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Starting Ralph for unscoped"));

    let state_path = repo.path().join(".ito/.state/ralph/unscoped/state.json");
    assert!(state_path.exists());
}
