//! End-to-end integration tests for requirement traceability.
//!
//! These tests exercise the full pipeline:
//!   delta spec parsing → traceability computation → validation → trace output
//!
//! Each test builds a minimal `.ito/` directory structure in a temp dir,
//! then calls `validate_change` or `compute_trace_output` and asserts on the result.

use ito_core::change_repository::FsChangeRepository;
use ito_core::trace::compute_trace_output;
use ito_core::validate::validate_change;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
}

/// A valid delta spec with requirement IDs on all requirements.
fn traced_spec() -> &'static str {
    r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha

### Requirement: Feature Beta
The system SHALL provide feature beta.

- **Requirement ID**: auth:feature-beta

#### Scenario: Beta works
- **WHEN** the user triggers beta
- **THEN** the system performs beta
"#
}

/// An enhanced tasks.md where all requirements are covered.
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

/// An enhanced tasks.md where only alpha is covered (beta is uncovered).
fn partially_covered_tasks(change_id: &str) -> String {
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
- **Status**: [ ] pending\n"
    )
}

/// An enhanced tasks.md with a task referencing a non-existent requirement ID.
fn tasks_with_unresolved_ref(change_id: &str) -> String {
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
- **Requirements**: auth:feature-alpha, auth:does-not-exist\n\
- **Updated At**: 2026-01-01\n\
- **Status**: [ ] pending\n"
    )
}

/// An enhanced tasks.md where the only covering task is shelved.
fn shelved_covering_tasks(change_id: &str) -> String {
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
- **Status**: [-] shelved\n"
    )
}

// ---------------------------------------------------------------------------
// Scenario 1: Traced change — all requirements covered (happy path)
// ---------------------------------------------------------------------------

#[test]
fn traced_change_all_covered_validate_passes() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_traced-happy";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    // No traceability errors or warnings.
    let trace_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability")
        .collect();
    assert!(
        trace_issues.iter().all(|i| i.level == "INFO"),
        "expected no traceability errors/warnings, got: {trace_issues:?}"
    );
}

#[test]
fn traced_change_all_covered_trace_output_is_ready() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_traced-happy";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "ready");
    assert_eq!(out.declared_requirements.len(), 2);
    assert!(
        out.uncovered.is_empty(),
        "expected no uncovered, got: {:?}",
        out.uncovered
    );
    assert!(
        out.unresolved.is_empty(),
        "expected no unresolved, got: {:?}",
        out.unresolved
    );
    assert_eq!(out.covered.len(), 2);
}

// ---------------------------------------------------------------------------
// Scenario 2: Traced change with uncovered requirement
// ---------------------------------------------------------------------------

#[test]
fn traced_change_uncovered_req_is_warning_in_non_strict() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-02_traced-uncovered";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &partially_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    let uncovered_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("not covered"))
        .collect();
    assert!(
        !uncovered_issues.is_empty(),
        "expected uncovered requirement warning, got issues: {:?}",
        r.issues
    );
    assert!(
        uncovered_issues.iter().all(|i| i.level == "WARNING"),
        "uncovered requirement should be WARNING in non-strict mode, got: {uncovered_issues:?}"
    );
}

#[test]
fn traced_change_uncovered_req_is_error_in_strict() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-02_traced-uncovered";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &partially_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, true).unwrap();

    let uncovered_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("not covered"))
        .collect();
    assert!(
        !uncovered_issues.is_empty(),
        "expected uncovered requirement error in strict mode, got issues: {:?}",
        r.issues
    );
    assert!(
        uncovered_issues.iter().all(|i| i.level == "ERROR"),
        "uncovered requirement should be ERROR in strict mode, got: {uncovered_issues:?}"
    );
}

#[test]
fn traced_change_uncovered_req_trace_output_shows_uncovered() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-02_traced-uncovered";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &partially_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "ready");
    assert_eq!(out.uncovered, vec!["auth:feature-beta".to_string()]);
    assert_eq!(out.covered.len(), 1);
    assert_eq!(out.covered[0].requirement_id, "auth:feature-alpha");
}

// ---------------------------------------------------------------------------
// Scenario 3: Traced change with unresolved task reference
// ---------------------------------------------------------------------------

#[test]
fn traced_change_unresolved_ref_is_error_in_validate() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-03_traced-unresolved";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &tasks_with_unresolved_ref(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    let unresolved_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("unknown requirement ID"))
        .collect();
    assert!(
        !unresolved_issues.is_empty(),
        "expected unresolved reference error, got issues: {:?}",
        r.issues
    );
    assert!(
        unresolved_issues.iter().all(|i| i.level == "ERROR"),
        "unresolved reference should always be ERROR, got: {unresolved_issues:?}"
    );
}

#[test]
fn traced_change_unresolved_ref_trace_output_shows_unresolved() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-03_traced-unresolved";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        traced_spec(),
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &tasks_with_unresolved_ref(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "ready");
    assert_eq!(out.unresolved.len(), 1);
    assert_eq!(out.unresolved[0].task_id, "1.1");
    assert_eq!(out.unresolved[0].requirement_id, "auth:does-not-exist");
}

// ---------------------------------------------------------------------------
// Scenario 4: Partial IDs (invalid — some requirements missing IDs)
// ---------------------------------------------------------------------------

#[test]
fn partial_ids_validate_reports_error() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-04_partial-ids";

    // One requirement has an ID, one does not.
    let partial_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha

### Requirement: Feature Beta
The system SHALL provide feature beta.

#### Scenario: Beta works
- **WHEN** the user triggers beta
- **THEN** the system performs beta
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        partial_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    let partial_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("no Requirement ID"))
        .collect();
    assert!(
        !partial_issues.is_empty(),
        "expected partial-ID error, got issues: {:?}",
        r.issues
    );
    assert!(
        partial_issues.iter().all(|i| i.level == "ERROR"),
        "partial IDs should be ERROR, got: {partial_issues:?}"
    );
}

#[test]
fn partial_ids_trace_output_is_invalid() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-04_partial-ids";

    let partial_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha

### Requirement: Feature Beta
The system SHALL provide feature beta.

#### Scenario: Beta works
- **WHEN** the user triggers beta
- **THEN** the system performs beta
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        partial_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "invalid");
    // The reason contains the requirement text (which includes "feature beta").
    assert!(
        out.reason
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .contains("feature beta")
            || out.reason.as_deref().unwrap_or("").contains("missing"),
        "reason should mention the requirement missing an ID, got: {:?}",
        out.reason
    );
}

// ---------------------------------------------------------------------------
// Scenario 5: Legacy checkbox change (no traceability)
// ---------------------------------------------------------------------------

#[test]
fn legacy_checkbox_change_validate_passes_without_traceability_checks() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-05_legacy-checkbox";

    // No requirement IDs in the spec.
    let legacy_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        legacy_spec,
    );
    // Checkbox-format tasks.md.
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Implement alpha\n",
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    // No traceability errors or warnings — only INFO at most.
    let trace_errors: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && (i.level == "ERROR" || i.level == "WARNING"))
        .collect();
    assert!(
        trace_errors.is_empty(),
        "legacy change should have no traceability errors/warnings, got: {trace_errors:?}"
    );
}

#[test]
fn legacy_checkbox_change_trace_output_is_unavailable() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-05_legacy-checkbox";

    let legacy_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        legacy_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## 1. Implementation\n- [ ] 1.1 Implement alpha\n",
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "unavailable");
}

// ---------------------------------------------------------------------------
// Scenario 6: Shelved task — requirement shows as uncovered
// ---------------------------------------------------------------------------

#[test]
fn shelved_task_leaves_requirement_uncovered() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-06_shelved-task";

    // Only one requirement, only one task — and it's shelved.
    let single_req_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        single_req_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &shelved_covering_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "ready");
    assert!(
        out.covered.is_empty(),
        "shelved task must not count as coverage, got covered: {:?}",
        out.covered
    );
    assert_eq!(out.uncovered, vec!["auth:feature-alpha".to_string()]);
}

#[test]
fn shelved_task_uncovered_req_is_warning_in_validate() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-06_shelved-task";

    let single_req_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        single_req_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &shelved_covering_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    let uncovered_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("not covered"))
        .collect();
    assert!(
        !uncovered_issues.is_empty(),
        "shelved-only coverage should produce uncovered warning, got issues: {:?}",
        r.issues
    );
    assert!(
        uncovered_issues.iter().all(|i| i.level == "WARNING"),
        "uncovered (shelved) should be WARNING in non-strict, got: {uncovered_issues:?}"
    );
}

// ---------------------------------------------------------------------------
// Scenario 7: Duplicate requirement IDs
// ---------------------------------------------------------------------------

#[test]
fn duplicate_requirement_ids_produce_error_in_validate() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-07_duplicate-ids";

    // Two requirements share the same ID.
    let dup_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha

### Requirement: Feature Alpha Duplicate
The system SHALL also provide feature alpha (duplicate ID).

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha dup works
- **WHEN** the user triggers alpha again
- **THEN** the system performs alpha again
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        dup_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let r = validate_change(&repo, &ito, change_id, false).unwrap();

    let dup_issues: Vec<_> = r
        .issues
        .iter()
        .filter(|i| i.path == "traceability" && i.message.contains("Duplicate"))
        .collect();
    assert!(
        !dup_issues.is_empty(),
        "expected duplicate ID error, got issues: {:?}",
        r.issues
    );
    assert!(
        dup_issues.iter().all(|i| i.level == "ERROR"),
        "duplicate IDs should be ERROR, got: {dup_issues:?}"
    );
}

#[test]
fn duplicate_requirement_ids_trace_output_has_diagnostics() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-07_duplicate-ids";

    let dup_spec = r#"## ADDED Requirements

### Requirement: Feature Alpha
The system SHALL provide feature alpha.

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha works
- **WHEN** the user triggers alpha
- **THEN** the system performs alpha

### Requirement: Feature Alpha Duplicate
The system SHALL also provide feature alpha (duplicate ID).

- **Requirement ID**: auth:feature-alpha

#### Scenario: Alpha dup works
- **WHEN** the user triggers alpha again
- **THEN** the system performs alpha again
"#;

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        dup_spec,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        &fully_covered_tasks(change_id),
    );

    let repo = FsChangeRepository::new(&ito);
    let out = compute_trace_output(&repo, change_id).unwrap();

    assert_eq!(out.status, "ready");
    assert!(
        out.diagnostics
            .iter()
            .any(|d| d.contains("auth:feature-alpha")),
        "expected duplicate diagnostic, got: {:?}",
        out.diagnostics
    );
}
