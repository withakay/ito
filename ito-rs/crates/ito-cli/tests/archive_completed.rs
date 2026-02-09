//! Integration tests for `ito archive --completed` (batch archive).

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

/// Create a repo with multiple changes at various completion states.
fn make_repo_with_mixed_changes() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    fixtures::write(td.path().join("README.md"), "# temp\n");

    // Minimal module.
    fixtures::write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for archive tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_completed-a\n- [ ] 000-02_completed-b\n- [ ] 000-03_in-progress\n",
    );

    // Main spec.
    fixtures::write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Completed change A (all tasks done).
    fixtures::write(
        td.path()
            .join(".ito/changes/000-01_completed-a/proposal.md"),
        "## Why\nTest fixture A\n\n## What Changes\n- Adds something\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path().join(".ito/changes/000-01_completed-a/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    fixtures::write(
        td.path()
            .join(".ito/changes/000-01_completed-a/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha A\nThe system SHALL include alpha A behavior in strict validation.\n\n#### Scenario: A ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    // Completed change B (all tasks done).
    fixtures::write(
        td.path()
            .join(".ito/changes/000-02_completed-b/proposal.md"),
        "## Why\nTest fixture B\n\n## What Changes\n- Adds something else\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path().join(".ito/changes/000-02_completed-b/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do another thing\n",
    );
    fixtures::write(
        td.path()
            .join(".ito/changes/000-02_completed-b/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha B\nThe system SHALL include alpha B behavior in strict validation.\n\n#### Scenario: B ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    // In-progress change (tasks not done).
    fixtures::write(
        td.path()
            .join(".ito/changes/000-03_in-progress/proposal.md"),
        "## Why\nTest fixture C\n\n## What Changes\n- Work in progress\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path().join(".ito/changes/000-03_in-progress/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Not done yet\n",
    );

    td
}

/// `ito archive --completed` with no completed changes prints message and exits 0.
#[test]
fn archive_completed_no_completed_changes() {
    let td = tempfile::tempdir().expect("repo");
    fixtures::write(td.path().join("README.md"), "# temp\n");

    // Module.
    fixtures::write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for archive tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_only-change\n",
    );

    // Only change is in-progress (not completed).
    fixtures::write(
        td.path()
            .join(".ito/changes/000-01_only-change/proposal.md"),
        "## Why\nTest\n\n## What Changes\n- WIP\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path().join(".ito/changes/000-01_only-change/tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Not done\n",
    );

    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), td.path());

    let out = run_rust_candidate(
        rust_path,
        &["archive", "--completed", "-y"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("No completed changes"),
        "Expected 'No completed changes' in output, got: {}",
        combined
    );
}

/// `ito archive --completed -y` archives all completed changes.
#[test]
fn archive_completed_archives_all_completed() {
    let base = make_repo_with_mixed_changes();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["archive", "--completed", "-y"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Both completed changes should be archived.
    let archive_root = repo.path().join(".ito/changes/archive");
    assert!(archive_root.exists(), "archive directory should exist");
    let entries: Vec<String> = std::fs::read_dir(&archive_root)
        .expect("read archive dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    assert!(
        entries.iter().any(|e| e.contains("000-01_completed-a")),
        "000-01_completed-a should be archived, entries: {:?}",
        entries
    );
    assert!(
        entries.iter().any(|e| e.contains("000-02_completed-b")),
        "000-02_completed-b should be archived, entries: {:?}",
        entries
    );

    // In-progress change should still be present in changes/.
    assert!(
        repo.path().join(".ito/changes/000-03_in-progress").exists(),
        "in-progress change should not be archived"
    );

    // Summary should mention archived count.
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("Archived 2"),
        "Expected 'Archived 2' in output, got: {}",
        combined
    );
}

/// `ito archive --completed --skip-specs` skips spec updates for all changes.
#[test]
fn archive_completed_skip_specs() {
    let base = make_repo_with_mixed_changes();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["archive", "--completed", "-y", "--skip-specs"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    // Both changes should be archived.
    let archive_root = repo.path().join(".ito/changes/archive");
    let entries: Vec<String> = std::fs::read_dir(&archive_root)
        .expect("read archive dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(
        entries.len(),
        2,
        "expected 2 archived changes: {:?}",
        entries
    );

    // The main spec should NOT have been updated (since we skipped specs).
    let main_spec =
        std::fs::read_to_string(repo.path().join(".ito/specs/alpha/spec.md")).expect("main spec");
    assert!(
        !main_spec.contains("Alpha A"),
        "Main spec should not contain delta from change A when --skip-specs"
    );
}

/// `ito archive some-change --completed` is rejected (mutual exclusivity).
#[test]
fn archive_completed_conflict_with_positional() {
    let base = make_repo_with_mixed_changes();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["archive", "000-01_completed-a", "--completed"],
        repo.path(),
        home.path(),
    );
    assert_ne!(
        out.code, 0,
        "Should fail when both CHANGE and --completed are provided"
    );
    let combined = format!("{}{}", out.stdout, out.stderr);
    assert!(
        combined.contains("cannot be used with"),
        "Expected conflict error, got: {}",
        combined
    );
}
