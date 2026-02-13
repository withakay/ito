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
        "# Ungrouped\n\n## Purpose\nModule for harness integration tests.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Minimal spec.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nTest spec for harness integration tests.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Minimal change.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture for harness integration\n\n## What Changes\n- Adds harness testing\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    td
}

fn reset_repo(dst: &Path, src: &Path) {
    ito_test_support::reset_dir(dst, src).unwrap();
}

/// Verifies that ralph --harness accepts all user-facing harness names.
///
/// This regression test ensures that the CLI correctly accepts each documented
/// harness name without errors. It uses --status to avoid requiring actual
/// harness binaries to be installed.
#[test]
fn ralph_accepts_all_documented_harness_names() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let harnesses = vec!["opencode", "claude", "codex", "copilot"];

    for harness in harnesses {
        let out = run_rust_candidate(
            rust_path,
            &[
                "ralph",
                "--change",
                "000-01_test-change",
                "--harness",
                harness,
                "--status",
                "--no-interactive",
            ],
            repo.path(),
            home.path(),
        );
        assert_eq!(
            out.code, 0,
            "harness={} should be accepted, stderr={}",
            harness, out.stderr
        );
    }
}

/// Verifies that ralph --harness github-copilot works as an alias for copilot.
///
/// This ensures the canonical internal name can still be used even though it's
/// not in the user-facing documentation.
#[test]
fn ralph_accepts_github_copilot_canonical_name() {
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
            "github-copilot",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
}

/// Verifies that ralph --harness stub still works for testing purposes.
///
/// The stub harness is not user-facing but should remain functional for
/// internal testing and development.
#[test]
fn ralph_accepts_stub_harness_for_testing() {
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
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
}

/// Verifies that ralph rejects completely unknown harness names with a clear error.
///
/// This regression test ensures the error message includes the list of known
/// harnesses to help users correct typos or invalid inputs.
#[test]
fn ralph_rejects_unknown_harness_with_clear_error_listing_known_harnesses() {
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
            "nonexistent-harness",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "stdout={}", out.stdout);
    assert!(
        out.stderr.contains("Unknown harness: nonexistent-harness"),
        "stderr={}",
        out.stderr
    );
    assert!(
        out.stderr.contains("known harnesses:"),
        "error should list known harnesses, stderr={}",
        out.stderr
    );
}

/// Verifies that ralph with --harness flag can be combined with short change flag.
///
/// This integration test ensures the harness selection works correctly when
/// combined with other CLI flag variants.
#[test]
fn ralph_harness_flag_works_with_short_change_flag() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "-c",
            "000-01_test-change",
            "--harness",
            "claude",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
}

/// Verifies that ralph with harness selection works with the ralph alias.
///
/// This ensures the 'ra' alias works correctly with harness selection.
#[test]
fn ralph_alias_works_with_harness_selection() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "ra",
            "--change",
            "000-01_test-change",
            "--harness",
            "codex",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
}

/// Verifies that harness names are case-sensitive.
///
/// This boundary test ensures the system correctly rejects capitalized or
/// mixed-case harness names, which helps catch user typos.
#[test]
fn ralph_harness_names_are_case_sensitive() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let invalid_cases = vec!["Claude", "CLAUDE", "Codex", "OPENCODE"];

    for harness in invalid_cases {
        let out = run_rust_candidate(
            rust_path,
            &[
                "ralph",
                "--change",
                "000-01_test-change",
                "--harness",
                harness,
                "--status",
                "--no-interactive",
            ],
            repo.path(),
            home.path(),
        );
        assert_ne!(
            out.code, 0,
            "harness={} should be rejected (case-sensitive)",
            harness
        );
        assert!(
            out.stderr.contains("Unknown harness"),
            "harness={} stderr={}",
            harness,
            out.stderr
        );
    }
}

/// Verifies that harness selection persists across status checks.
///
/// This ensures the harness configuration is correctly handled when checking
/// the status of a ralph session that was started with a specific harness.
#[test]
fn ralph_status_works_after_running_with_specific_harness() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    // Run one iteration with stub harness
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
    assert_eq!(out.code, 0, "run failed: stderr={}", out.stderr);

    // Check status with a different harness name (should still work)
    let out = run_rust_candidate(
        rust_path,
        &[
            "ralph",
            "--change",
            "000-01_test-change",
            "--harness",
            "claude",
            "--status",
            "--no-interactive",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "status check failed: stderr={}", out.stderr);
}

/// Verifies that ralph --help mentions the harness flag with known harness names.
///
/// This documentation test ensures users can discover available harnesses through
/// the help text.
#[test]
fn ralph_help_documents_harness_flag_with_options() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["ralph", "--help"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    assert!(
        out.stdout.contains("--harness"),
        "help should mention --harness flag"
    );
    assert!(
        out.stdout.contains("opencode")
            || out.stdout.contains("claude")
            || out.stdout.contains("Harness to run"),
        "help should mention harness options"
    );
}

/// Verifies that multiple harness-related flags can be combined correctly.
///
/// This integration test ensures complex flag combinations work as expected.
#[test]
fn ralph_combines_harness_with_other_flags() {
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
            "--skip-validation",
            "--status",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
}