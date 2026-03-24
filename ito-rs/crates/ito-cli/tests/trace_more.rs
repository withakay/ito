//! CLI integration tests for the `ito trace` command.
//!
//! Each test builds a minimal `.ito/` directory structure in a temp dir and
//! invokes the `ito trace` binary, asserting on exit code and output.

#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

/// Writes `contents` to `path`, creating parent directories if necessary.
///
/// Panics if creating parent directories or writing the file fails.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let mut p = std::env::temp_dir();
/// p.push("ito_write_example");
/// p.push("example.txt");
///
/// write(&p, "hello");
/// assert_eq!(std::fs::read_to_string(p).unwrap(), "hello");
/// ```
fn write(path: impl AsRef<std::path::Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

/// Hard-coded delta spec markdown containing two requirements (auth:feature-alpha and auth:feature-beta), each with a `Requirement ID` and a scenario.
///
/// # Examples
///
/// ```
/// let spec = traced_spec();
/// assert!(spec.contains("auth:feature-alpha"));
/// assert!(spec.contains("auth:feature-beta"));
/// ```
fn traced_spec() -> &'static str {
    "## ADDED Requirements\n\n\
### Requirement: Feature Alpha\n\
The system SHALL provide feature alpha.\n\n\
- **Requirement ID**: auth:feature-alpha\n\n\
#### Scenario: Alpha works\n\
- **WHEN** the user triggers alpha\n\
- **THEN** the system performs alpha\n\n\
### Requirement: Feature Beta\n\
The system SHALL provide feature beta.\n\n\
- **Requirement ID**: auth:feature-beta\n\n\
#### Scenario: Beta works\n\
- **WHEN** the user triggers beta\n\
- **THEN** the system performs beta\n"
}

/// Produce a tasks.md document for a change where each requirement is covered by a task.
///
/// The returned string is a markdown-formatted "tasks.md" containing two tasks (Alpha and Beta)
/// whose Requirements lines reference `auth:feature-alpha` and `auth:feature-beta`, with the
/// provided `change_id` placed in the header.
///
/// # Examples
///
/// ```
/// let md = fully_covered_tasks("123-change");
/// assert!(md.contains("Tasks for: 123-change"));
/// assert!(md.contains("auth:feature-alpha"));
/// assert!(md.contains("auth:feature-beta"));
/// ```
fn fully_covered_tasks(change_id: &str) -> String {
    format!(
        "# Tasks for: {change_id}\n\n\
## Wave 1\n\n\
- **Depends On**: None\n\n\
### Task 1.1: Implement Alpha\n\n\
- **Files**: `src/alpha.rs`\n\
- **Dependencies**: None\n\
- **Action**: Implement alpha\n\
- **Verify**: `cargo test`\n\
- **Done When**: Tests pass\n\
- **Requirements**: auth:feature-alpha\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n\n\
### Task 1.2: Implement Beta\n\n\
- **Files**: `src/beta.rs`\n\
- **Dependencies**: None\n\
- **Action**: Implement beta\n\
- **Verify**: `cargo test`\n\
- **Done When**: Tests pass\n\
- **Requirements**: auth:feature-beta\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n"
    )
}

// ---------------------------------------------------------------------------
// Scenario 1: Traced change — all requirements covered (happy path)
// ---------------------------------------------------------------------------

#[test]
fn trace_fully_covered_exits_zero() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-01_traced-happy";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        traced_spec(),
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let out = run_rust_candidate(rust_path, &["trace", change_id], repo.path(), home.path());
    assert_eq!(
        out.code, 0,
        "trace should exit 0 for fully covered change, stderr: {}",
        out.stderr
    );
    assert!(
        out.stdout.contains("ready") || out.stdout.contains("covered"),
        "output should indicate ready/covered status, got: {}",
        out.stdout
    );
}

#[test]
fn trace_fully_covered_json_has_ready_status() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-01_traced-happy";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        traced_spec(),
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let out = run_rust_candidate(
        rust_path,
        &["trace", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(
        out.code, 0,
        "trace --json should exit 0, stderr: {}",
        out.stderr
    );
    let json: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("output should be valid JSON");
    assert_eq!(json["status"], "ready");
    assert_eq!(json["change_id"], change_id);
    assert!(
        json["uncovered"]
            .as_array()
            .map(|a| a.is_empty())
            .unwrap_or(false),
        "uncovered should be empty, got: {}",
        json["uncovered"]
    );
}

// ---------------------------------------------------------------------------
// Scenario 2: Traced change with uncovered requirement
// ---------------------------------------------------------------------------

#[test]
fn trace_uncovered_requirement_shows_uncovered_in_output() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-02_traced-uncovered";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        traced_spec(),
    );
    // Only alpha is covered; beta is not.
    let partial_tasks = format!(
        "# Tasks for: {change_id}\n\n\
## Wave 1\n\n\
- **Depends On**: None\n\n\
### Task 1.1: Implement Alpha\n\n\
- **Files**: `src/alpha.rs`\n\
- **Dependencies**: None\n\
- **Action**: Implement alpha\n\
- **Verify**: `cargo test`\n\
- **Done When**: Tests pass\n\
- **Requirements**: auth:feature-alpha\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n"
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &partial_tasks,
    );

    // `ito trace` is informational — it always exits 0 and reports the coverage.
    let out = run_rust_candidate(rust_path, &["trace", change_id], repo.path(), home.path());
    assert_eq!(
        out.code, 0,
        "trace exits 0 (informational command), stderr: {}",
        out.stderr
    );
    assert!(
        out.stdout.contains("auth:feature-beta"),
        "output should mention the uncovered requirement, got: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Uncovered") || out.stdout.contains("uncovered"),
        "output should indicate uncovered status, got: {}",
        out.stdout
    );
}

#[test]
fn trace_uncovered_requirement_json_shows_uncovered_list() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-02_traced-uncovered";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        traced_spec(),
    );
    let partial_tasks = format!(
        "# Tasks for: {change_id}\n\n\
## Wave 1\n\n\
- **Depends On**: None\n\n\
### Task 1.1: Implement Alpha\n\n\
- **Files**: `src/alpha.rs`\n\
- **Dependencies**: None\n\
- **Action**: Implement alpha\n\
- **Verify**: `cargo test`\n\
- **Done When**: Tests pass\n\
- **Requirements**: auth:feature-alpha\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n"
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &partial_tasks,
    );

    let out = run_rust_candidate(
        rust_path,
        &["trace", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    let json: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("output should be valid JSON");
    assert_eq!(json["status"], "ready");
    let uncovered = json["uncovered"]
        .as_array()
        .expect("uncovered should be array");
    assert!(
        uncovered.iter().any(|v| v == "auth:feature-beta"),
        "uncovered should contain auth:feature-beta, got: {uncovered:?}"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3: Traced change with unresolved task reference
// ---------------------------------------------------------------------------

#[test]
fn trace_unresolved_reference_shows_unresolved_in_output() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-03_traced-unresolved";

    let single_spec = "## ADDED Requirements\n\n\
### Requirement: Feature Alpha\n\
The system SHALL provide feature alpha.\n\n\
- **Requirement ID**: auth:feature-alpha\n\n\
#### Scenario: Alpha works\n\
- **WHEN** the user triggers alpha\n\
- **THEN** the system performs alpha\n";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        single_spec,
    );
    let tasks_with_ghost = format!(
        "# Tasks for: {change_id}\n\n\
## Wave 1\n\n\
- **Depends On**: None\n\n\
### Task 1.1: Implement Alpha\n\n\
- **Files**: `src/alpha.rs`\n\
- **Dependencies**: None\n\
- **Action**: Implement alpha\n\
- **Verify**: `cargo test`\n\
- **Done When**: Tests pass\n\
- **Requirements**: auth:feature-alpha, auth:ghost-requirement\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n"
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &tasks_with_ghost,
    );

    // `ito trace` is informational — it always exits 0 and reports the coverage.
    let out = run_rust_candidate(rust_path, &["trace", change_id], repo.path(), home.path());
    assert_eq!(
        out.code, 0,
        "trace exits 0 (informational command), stderr: {}",
        out.stderr
    );
    assert!(
        out.stdout.contains("auth:ghost-requirement"),
        "output should mention the unresolved requirement, got: {}",
        out.stdout
    );
    assert!(
        out.stdout.contains("Unresolved") || out.stdout.contains("unresolved"),
        "output should indicate unresolved references, got: {}",
        out.stdout
    );
}

// ---------------------------------------------------------------------------
// Scenario 4: Partial IDs (invalid)
// ---------------------------------------------------------------------------

/// Verifies that tracing a change whose spec contains some requirements without a `Requirement ID` yields a JSON `status` of `"invalid"`.
///
/// The test creates a change with one requirement that includes `Requirement ID` and another that does not, runs `ito trace <change_id> --json`, parses the JSON output, and asserts that `json["status"] == "invalid"`.
///
/// # Examples
///
/// ```rust
/// // Prepare a spec where one requirement is missing `Requirement ID`,
/// // run `ito trace <change_id> --json`, parse the output,
/// // and assert `json["status"] == "invalid"`.
/// ```
#[test]
fn trace_partial_ids_json_shows_invalid_status() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-04_partial-ids";

    // One requirement has an ID, one does not.
    let partial_spec = "## ADDED Requirements\n\n\
### Requirement: Feature Alpha\n\
The system SHALL provide feature alpha.\n\n\
- **Requirement ID**: auth:feature-alpha\n\n\
#### Scenario: Alpha works\n\
- **WHEN** the user triggers alpha\n\
- **THEN** the system performs alpha\n\n\
### Requirement: Feature Beta\n\
The system SHALL provide feature beta.\n\n\
#### Scenario: Beta works\n\
- **WHEN** the user triggers beta\n\
- **THEN** the system performs beta\n";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        partial_spec,
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let out = run_rust_candidate(
        rust_path,
        &["trace", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    let json: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("output should be valid JSON");
    assert_eq!(json["status"], "invalid");
}

// ---------------------------------------------------------------------------
// Scenario 5: Legacy checkbox change (no traceability)
// ---------------------------------------------------------------------------

/// Verifies that tracing a legacy checkbox–style change reports `unavailable`.
///
/// Creates a spec that has no `Requirement ID` lines and a `tasks.md` that uses legacy
/// checkbox formatting, runs `ito trace <change_id> --json`, and asserts the produced JSON
/// has `"status": "unavailable"`.
///
/// # Examples
///
/// ```
/// let json: serde_json::Value = serde_json::from_str(r#"{"status":"unavailable"}"#).unwrap();
/// assert_eq!(json["status"], "unavailable");
/// ```
#[test]
fn trace_legacy_checkbox_change_shows_unavailable() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    let change_id = "001-05_legacy-checkbox";

    // No requirement IDs.
    let legacy_spec = "## ADDED Requirements\n\n\
### Requirement: Feature Alpha\n\
The system SHALL provide feature alpha.\n\n\
#### Scenario: Alpha works\n\
- **WHEN** the user triggers alpha\n\
- **THEN** the system performs alpha\n";

    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("specs/auth/spec.md"),
        legacy_spec,
    );
    write(
        repo.path()
            .join(".ito/changes")
            .join(change_id)
            .join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Implement alpha\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["trace", change_id, "--json"],
        repo.path(),
        home.path(),
    );
    let json: serde_json::Value =
        serde_json::from_str(&out.stdout).expect("output should be valid JSON");
    assert_eq!(json["status"], "unavailable");
}

// ---------------------------------------------------------------------------
// Scenario 6: Missing change — error
// ---------------------------------------------------------------------------

/// Verifies that tracing a non-existent change returns a non-zero exit code.
///
/// Creates a repository with a `.ito/changes` directory but without the requested
/// change, runs `ito trace <change_id>`, and expects the process to exit with a
/// non-zero status indicating the change was not found.
///
/// # Examples
///
/// ```no_run
/// let repo = tempfile::tempdir().unwrap();
/// let home = tempfile::tempdir().unwrap();
/// std::fs::create_dir_all(repo.path().join(".ito/changes")).unwrap();
///
/// let out = ito_test_support::run_rust_candidate(
///     assert_cmd::cargo::cargo_bin!("ito"),
///     &["trace", "999-99_does-not-exist"],
///     repo.path(),
///     home.path(),
/// );
/// assert_ne!(out.code, 0);
/// ```
#[test]
fn trace_missing_change_exits_nonzero() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    std::fs::create_dir_all(repo.path().join(".ito/changes")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["trace", "999-99_does-not-exist"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0, "trace should exit non-zero for missing change");
}
