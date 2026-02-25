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

    let r = validate_change(&change_repo, &ito, "001-01_demo", false).unwrap();
    assert!(!r.valid);
    assert!(r
        .issues
        .iter()
        .any(|i| i.message.contains("at least one delta")));
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

    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(!r.valid);
    assert!(r
        .issues
        .iter()
        .any(|i| i.message.contains("SHALL") || i.message.contains("MUST")));
}

#[test]
fn validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    // Project-local schema without validation.yaml.
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("unknown")
            .join("schema.yaml"),
        r#"
name: unknown
version: 1
artifacts:
  - id: notes
    generates: notes.md
    template: notes.md
    requires: []
apply:
  requires: [notes]
"#,
    );

    // Change selects the unknown schema.
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: unknown\n",
    );

    // Create the apply-required artifact so we only test delta validation behavior.
    write(
        &ito.join("changes").join(change_id).join("notes.md"),
        "ok\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        !r.issues
            .iter()
            .any(|i| i.message.contains("at least one delta")),
        "unknown schema should not require deltas, got issues: {:?}",
        r.issues
    );
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("manual validation required")),
        "unknown schema should emit manual validation issue, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_uses_validation_yaml_delta_specs_validator_when_configured() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    // Project-local schema with validation.yaml.
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("delta")
            .join("schema.yaml"),
        r#"
name: delta
version: 1
artifacts:
  - id: specs
    generates: specs/**/*.md
    template: specs/spec.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("delta")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
"#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: delta\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
No keywords here.

#### Scenario: S
ok
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("SHALL") || i.message.contains("MUST")),
        "delta validator should run, got issues: {:?}",
        r.issues
    );
    assert!(
        !r.issues
            .iter()
            .any(|i| i.message.contains("manual validation required")),
        "schema with validation.yaml should not emit manual-validation issue, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("nodeltas")
            .join("schema.yaml"),
        r#"
name: nodeltas
version: 1
artifacts:
  - id: notes
    generates: notes.md
    template: notes.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("nodeltas")
            .join("validation.yaml"),
        "version: 1\n",
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: nodeltas\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        !r.issues
            .iter()
            .any(|i| i.message.contains("at least one delta")),
        "schema validation without delta validator should not require deltas, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_validates_apply_tracks_file_when_configured() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracks")
            .join("schema.yaml"),
        r#"
name: tracks
version: 1
artifacts:
  - id: proposal
    generates: proposal.md
    template: proposal.md
    requires: []
apply:
  tracks: todo.md
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracks")
            .join("validation.yaml"),
        r#"
version: 1
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
"#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracks\n",
    );
    write(
        &ito.join("changes").join(change_id).join("todo.md"),
        r#"
# Tasks for: 001-01_demo

## Wave 1

### Task 1.1: Demo

- **Dependencies**: Task 2.1
- **Status**: [ ] pending
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        !r.issues.iter().any(|i| i.message.contains("tasks.md")),
        "apply.tracks validation should not require tasks.md, got issues: {:?}",
        r.issues
    );
    assert!(
        r.issues
            .iter()
            .any(|i| i.path == "changes/001-01_demo/todo.md" && i.level == "ERROR"),
        "apply.tracks validation should run and report todo.md diagnostics, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_skips_optional_validator_when_artifact_is_missing() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("optional")
            .join("schema.yaml"),
        r#"
name: optional
version: 1
artifacts:
  - id: tasks
    generates: tasks.md
    template: tasks.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("optional")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  tasks:
    required: false
    validate_as: ito.tasks-tracking.v1
"#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: optional\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        !r.issues
            .iter()
            .any(|i| i.message.contains("Failed to read") && i.path.contains("tasks.md")),
        "optional missing artifact should skip validators, got issues: {:?}",
        r.issues
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
