use ito_core::change_repository::FsChangeRepository;
use ito_core::module_repository::FsModuleRepository;
use ito_core::validate::{
    validate_change, validate_module, validate_spec_markdown, validate_tasks_file,
};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn validate_spec_markdown_reports_missing_purpose_and_requirements() {
    let md = r#"
## Purpose

## Requirements
"#;

    let r = validate_spec_markdown(md, false);
    assert!(!r.valid);
    assert!(r.summary.errors >= 1);
}

#[test]
fn validate_spec_markdown_strict_treats_warnings_as_invalid() {
    let md = r#"
## Purpose

Too short.

## Requirements

### Requirement: R
The system SHALL do it.

#### Scenario: S
ok
"#;

    let non_strict = validate_spec_markdown(md, false);
    assert!(non_strict.valid);
    assert!(non_strict.summary.warnings >= 1);

    let strict = validate_spec_markdown(md, true);
    assert!(!strict.valid);
    assert!(strict.summary.warnings >= 1);
}

#[test]
fn validate_change_requires_at_least_one_delta() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(ito.join("changes").join("001-01_demo")).unwrap();
    let change_repo = FsChangeRepository::new(&ito);

    let r = validate_change(&change_repo, "001-01_demo", false).unwrap();
    assert!(!r.valid);
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("at least one delta"))
    );
}

#[test]
fn validate_change_requires_shall_or_must_in_requirement_text() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let change_repo = FsChangeRepository::new(&ito);

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
This requirement has no keywords.

#### Scenario: S
ok
"#,
    );

    let r = validate_change(&change_repo, change_id, false).unwrap();
    assert!(!r.valid);
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("SHALL") || i.message.contains("MUST"))
    );
}

#[test]
fn validate_module_reports_missing_scope_and_short_purpose() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    write(
        &ito.join("modules").join("006_demo").join("module.md"),
        r#"
## Purpose

Too short.

## Scope
"#,
    );

    let (_name, r) = validate_module(&module_repo, &ito, "006_demo", false).unwrap();
    assert!(!r.valid);
    assert!(r.summary.errors >= 1);
}

#[test]
fn validate_tasks_file_returns_empty_for_valid_tasks() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let valid_tasks = "## 1. Implementation\n- [ ] 1.1 Do the thing\n- [x] 1.2 Done thing\n";
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        valid_tasks,
    );

    let issues = validate_tasks_file(&ito, change_id).expect("validate_tasks_file should succeed");
    assert!(
        issues.is_empty(),
        "valid tasks file should produce no issues, got: {issues:?}"
    );
}

#[test]
fn validate_tasks_file_returns_error_for_missing_file() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    let issues = validate_tasks_file(&ito, "nonexistent-change")
        .expect("validate_tasks_file should return Ok with error issues");
    assert!(
        !issues.is_empty(),
        "missing tasks file should produce at least one issue"
    );
    assert!(
        issues[0].level == "ERROR",
        "issue should be an ERROR, got: {}",
        issues[0].level
    );
    assert!(
        issues[0].message.contains("tasks.md"),
        "error message should mention tasks.md, got: {}",
        issues[0].message
    );
}

#[test]
fn validate_tasks_file_returns_diagnostics_for_malformed_content() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_broken";
    // A tasks file with duplicate task IDs should produce diagnostics
    let malformed_tasks = "\
## Wave 1

### Task 1.1: First task
- **Status**: [x] complete

### Task 1.1: Duplicate ID task
- **Status**: [ ] pending
";
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        malformed_tasks,
    );

    let issues = validate_tasks_file(&ito, change_id).expect("validate_tasks_file should succeed");
    assert!(
        !issues.is_empty(),
        "malformed tasks file should produce diagnostics, got empty"
    );
}
