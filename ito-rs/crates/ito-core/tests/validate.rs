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

    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(!r.valid);
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("SHALL") || i.message.contains("MUST"))
    );
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

    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("ito.delta-specs.v1")),
        "expected delta spec issues to cite validator id, got issues: {:?}",
        r.issues
    );

    let with_meta = r.issues.iter().any(|i| {
        let Some(meta) = i.metadata.as_ref().and_then(|m| m.as_object()) else {
            return false;
        };
        meta.get("validator_id").and_then(|v| v.as_str()) == Some("ito.delta-specs.v1")
            && meta.get("spec_path").and_then(|v| v.as_str())
                == Some(".ito/specs/delta-specs/spec.md")
    });
    assert!(
        with_meta,
        "expected delta spec issues to include metadata references"
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
fn validate_change_uses_apply_tracks_for_legacy_delta_schemas() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("spec-driven")
            .join("schema.yaml"),
        r#"
name: spec-driven
version: 1
artifacts: []
apply:
  tracks: todo.md
        "#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: spec-driven\n",
    );

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("a")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
The system SHALL do it.

#### Scenario: S
- **WHEN** ok
- **THEN** ok
"#,
    );

    write(
        &ito.join("changes").join(change_id).join("todo.md"),
        r#"
## Wave 1

### Task 1.1: First

- **Status**: [ ] pending
- **Updated At**: 2026-02-25

### Task 1.1: Duplicate

- **Status**: [ ] pending
- **Updated At**: 2026-02-25
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        !r.issues.iter().any(|i| i.path.contains("tasks.md")),
        "apply.tracks should override legacy tasks.md validation, got issues: {:?}",
        r.issues
    );
    assert!(
        r.issues
            .iter()
            .any(|i| i.path == "changes/001-01_demo/todo.md"),
        "expected validation to report issues against todo.md, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking() {
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
artifacts: []
apply:
  tracks: ../todo.md
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

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        r.issues
            .iter()
            .any(|i| i.path == "tracking" && i.message.contains("Invalid tracking file path")),
        "expected unsafe apply.tracks to be rejected, got issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("spec-driven")
            .join("schema.yaml"),
        r#"
name: spec-driven
version: 1
artifacts: []
apply:
  tracks: dir/todo.md
        "#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: spec-driven\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("a")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
The system SHALL do it.

#### Scenario: S
- **WHEN** ok
- **THEN** ok
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let r = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        r.issues
            .iter()
            .any(|i| i.path == "tracking" && i.message.contains("Invalid tracking file path")),
        "expected unsafe apply.tracks to be rejected, got issues: {:?}",
        r.issues
    );
}

#[test]
fn empty_tracking_file_is_warning_in_non_strict_and_error_in_strict() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("spec-driven")
            .join("schema.yaml"),
        r#"
name: spec-driven
version: 1
artifacts: []
        "#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: spec-driven\n",
    );

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("a")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
The system SHALL do it.

#### Scenario: S
- **WHEN** ok
- **THEN** ok
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "# just notes\n",
    );

    let change_repo = FsChangeRepository::new(&ito);

    let non_strict = validate_change(&change_repo, &ito, change_id, false).unwrap();
    assert!(
        non_strict
            .issues
            .iter()
            .any(|i| i.path == "changes/001-01_demo/tasks.md" && i.level == "WARNING"),
        "expected warning for empty tracking file, got issues: {:?}",
        non_strict.issues
    );

    let strict = validate_change(&change_repo, &ito, change_id, true).unwrap();
    assert!(
        strict
            .issues
            .iter()
            .any(|i| i.path == "changes/001-01_demo/tasks.md" && i.level == "ERROR"),
        "expected error for empty tracking file in strict mode, got issues: {:?}",
        strict.issues
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
fn validation_yaml_rules_extension_warns_for_unknown_rule_names() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("rules")
            .join("schema.yaml"),
        r#"
name: rules
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
            .join("rules")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      not_a_real_rule: warning
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: rules\n",
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
The system SHALL do it.

#### Scenario: S
- **WHEN** ok
- **THEN** ok
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.path == "schema.validation.artifacts.specs.rules.not_a_real_rule"
                && issue.level == "WARNING"
                && issue.message.contains("Unknown validation rule 'not_a_real_rule'")
        }),
        "expected unknown-rule warning, got issues: {:?}",
        report.issues
    );
}

#[test]
fn validation_yaml_proposal_entry_dispatches_rule_configuration() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        r#"
name: proposal-rules
version: 1
artifacts: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        r#"
version: 1
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    unknown_proposal_rule: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nNeed a proposal validator.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.path == "schema.validation.proposal.rules.unknown_proposal_rule"
                && issue.level == "WARNING"
                && issue.message.contains("Unknown validation rule 'unknown_proposal_rule'")
        }),
        "expected proposal rule warning, got issues: {:?}",
        report.issues
    );
}

#[test]
fn scenario_grammar_rule_reports_missing_when_then_and_given() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("scenario-rules")
            .join("schema.yaml"),
        r#"
name: scenario-rules
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
            .join("scenario-rules")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: scenario-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: Missing when
The system SHALL validate scenario steps.

#### Scenario: Missing when step
- **GIVEN** a request exists
- **THEN** validation fails

### Requirement: Missing then
The system SHALL validate scenario steps.

#### Scenario: Missing then step
- **GIVEN** a request exists
- **WHEN** validation runs

### Requirement: Missing given
The system SHALL validate scenario steps.

#### Scenario: Missing given step
- **WHEN** validation runs
- **THEN** validation warns
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("scenario_grammar")
            && issue.level == "ERROR"
            && issue.message.contains("missing WHEN")
    }));
    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("scenario_grammar")
            && issue.level == "ERROR"
            && issue.message.contains("missing THEN")
    }));
    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("scenario_grammar")
            && issue.level == "WARNING"
            && issue.message.contains("missing GIVEN")
    }));
}

#[test]
fn scenario_grammar_rule_warns_on_excessive_step_count() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("scenario-rules")
            .join("schema.yaml"),
        r#"
name: scenario-rules
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
            .join("scenario-rules")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: scenario-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: Long scenario
The system SHALL keep scenarios concise.

#### Scenario: Too many steps
- **GIVEN** step 1
- **AND** step 2
- **AND** step 3
- **AND** step 4
- **WHEN** step 5
- **AND** step 6
- **AND** step 7
- **AND** step 8
- **THEN** step 9
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("scenario_grammar")
            && issue.level == "WARNING"
            && issue.message.contains("more than 8 steps")
    }));
}

#[test]
fn scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("scenario-rules")
            .join("schema.yaml"),
        r#"
name: scenario-rules
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
            .join("scenario-rules")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: error
      ui_mechanics: warning
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: scenario-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: Non-ui mechanics
The system SHALL focus scenarios on behavior.

#### Scenario: Clicks save
- **GIVEN** a draft exists
- **WHEN** the user click the Save button
- **THEN** the draft is stored

### Requirement: Tagged ui flow
The system SHALL allow UI-tagged steps.

- **Tags**: ui

#### Scenario: Clicks save in ui flow
- **GIVEN** a draft exists
- **WHEN** the user click the Save button
- **THEN** the draft is stored

### Requirement: Code token is safe
The system SHALL ignore code-like tokens.

#### Scenario: Talks about unwrap
- **GIVEN** a code sample exists
- **WHEN** the docs mention .unwrap() and [link](#anchor)
- **THEN** validation does not flag UI mechanics
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    let ui_warnings: Vec<_> = report
        .issues
        .iter()
        .filter(|issue| issue.rule_id.as_deref() == Some("ui_mechanics"))
        .collect();
    assert_eq!(ui_warnings.len(), 1, "expected one UI warning, got: {:?}", report.issues);
    assert!(ui_warnings[0].message.contains("UI mechanics"));
}

#[test]
fn capabilities_consistency_rule_errors_for_listed_capability_without_delta() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        r#"
version: 1
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"
## Capabilities

### New Capabilities

- `auth`: Add login behavior.
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("capabilities_consistency")
            && issue.level == "ERROR"
            && issue.message.contains("listed in the proposal")
            && issue.message.contains("auth")
    }));
}

#[test]
fn capabilities_consistency_rule_errors_for_unlisted_delta_capability() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        r#"
version: 1
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"
## Capabilities

### New Capabilities

<!-- None -->
"#,
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("notifications")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: Notify users
The system SHALL notify users.

#### Scenario: Notify
- **WHEN** an event occurs
- **THEN** a notification is sent
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("capabilities_consistency")
            && issue.level == "ERROR"
            && issue.message.contains("not listed in the proposal")
            && issue.message.contains("notifications")
    }));
}

#[test]
fn capabilities_consistency_rule_checks_new_vs_modified_against_baseline() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        r#"
version: 1
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"
## Capabilities

### New Capabilities

- `auth`: Claimed as new.

### Modified Capabilities

- `payments`: Claimed as modified.
"#,
    );
    write(
        &ito.join("specs").join("auth").join("spec.md"),
        "## Purpose\n\nBaseline auth spec.\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## MODIFIED Requirements

### Requirement: Auth changes
The system SHALL modify auth.

#### Scenario: Auth
- **WHEN** auth changes
- **THEN** auth works
"#,
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("payments")
            .join("spec.md"),
        r#"
## MODIFIED Requirements

### Requirement: Payment changes
The system SHALL modify payments.

#### Scenario: Payments
- **WHEN** payments change
- **THEN** payments work
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("capabilities_consistency")
            && issue.message.contains("listed as new")
            && issue.message.contains("auth")
    }));
    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("capabilities_consistency")
            && issue.message.contains("listed as modified")
            && issue.message.contains("payments")
    }));
}

#[test]
fn capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        r#"
version: 1
proposal:
  required: true
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"
## Capabilities

### New Capabilities

- `<name>`: Placeholder entry.
- Plain bullet without code token.
- <!-- ignored -->
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("capabilities_consistency")
            && issue.level == "WARNING"
            && issue.message.contains("inline-code token")
    }));
    assert!(
        !report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("capabilities_consistency")
                && issue.level == "ERROR"
                && issue.message.contains("<name>")
        }),
        "placeholder entries should be ignored, got issues: {:?}",
        report.issues
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

    let issues =
        validate_tasks_file(&ito, change_id, false).expect("validate_tasks_file should succeed");
    assert!(
        issues.is_empty(),
        "valid tasks file should produce no issues, got: {issues:?}"
    );
}

#[test]
fn validate_tasks_file_returns_error_for_missing_file() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    let issues = validate_tasks_file(&ito, "nonexistent-change", false)
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

    let issues =
        validate_tasks_file(&ito, change_id, false).expect("validate_tasks_file should succeed");
    assert!(
        !issues.is_empty(),
        "malformed tasks file should produce diagnostics, got empty"
    );
}

#[test]
fn validate_tasks_file_issues_cite_tasks_tracking_validator_id() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_broken";
    let malformed_tasks = "\
## Wave 1\n\n\
### Task 1.1: First task\n\
- **Status**: [x] complete\n\
- **Updated At**: 2026-02-25\n\n\
### Task 1.1: Duplicate ID task\n\
- **Status**: [ ] pending\n\
- **Updated At**: 2026-02-25\n";
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        malformed_tasks,
    );

    let issues =
        validate_tasks_file(&ito, change_id, false).expect("validate_tasks_file should succeed");
    assert!(
        !issues.is_empty(),
        "expected tasks diagnostics to be reported"
    );

    let some = issues
        .iter()
        .any(|i| i.message.contains("ito.tasks-tracking.v1"));
    assert!(some, "expected tasks issues to cite validator id");

    let with_meta = issues.iter().any(|i| {
        let Some(meta) = i.metadata.as_ref().and_then(|m| m.as_object()) else {
            return false;
        };
        meta.get("validator_id").and_then(|v| v.as_str()) == Some("ito.tasks-tracking.v1")
            && meta.get("spec_path").and_then(|v| v.as_str())
                == Some(".ito/specs/tasks-tracking/spec.md")
    });
    assert!(
        with_meta,
        "expected tasks issues to include metadata references"
    );
}

#[test]
fn validate_tasks_file_uses_apply_tracks_when_set() {
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
artifacts: []
apply:
  tracks: todo.md
"#,
    );

    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracks\n",
    );

    // Duplicate ids should produce tasks-tracking diagnostics.
    write(
        &ito.join("changes").join(change_id).join("todo.md"),
        "## Wave 1\n\n### Task 1.1: First\n- **Status**: [x] complete\n- **Updated At**: 2026-02-25\n\n### Task 1.1: Duplicate\n- **Status**: [ ] pending\n- **Updated At**: 2026-02-25\n",
    );

    let issues =
        validate_tasks_file(&ito, change_id, false).expect("validate_tasks_file should succeed");
    assert!(
        issues
            .iter()
            .any(|i| i.path == "changes/001-01_demo/todo.md"),
        "expected issues to reference todo.md, got: {issues:?}"
    );
    assert!(
        !issues
            .iter()
            .any(|i| i.message.contains("tasks.md") || i.path.contains("tasks.md")),
        "apply.tracks should override tasks.md validation, got: {issues:?}"
    );
}

// ── Task 4.6: validate_module validates sub-modules ────────────────────────

#[test]
fn validate_module_passes_when_sub_modules_have_valid_module_md() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    // Parent module.
    write(
        &ito.join("modules").join("024_backend").join("module.md"),
        "# Backend\n\n## Purpose\nBackend module for the application.\n\n## Scope\n- *\n\n## Changes\n",
    );

    // Sub-module with valid module.md.
    write(
        &ito.join("modules")
            .join("024_backend")
            .join("sub")
            .join("01_auth")
            .join("module.md"),
        "# Auth\n\n## Purpose\nAuthentication sub-module for the backend.\n\n## Scope\n- *\n\n## Changes\n",
    );

    let (_name, r) = validate_module(&module_repo, &ito, "024", false).unwrap();
    assert!(r.valid, "should be valid; issues: {:?}", r.issues);
}

#[test]
fn validate_module_errors_when_sub_module_missing_module_md() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    // Parent module.
    write(
        &ito.join("modules").join("024_backend").join("module.md"),
        "# Backend\n\n## Purpose\nBackend module for the application.\n\n## Scope\n- *\n\n## Changes\n",
    );

    // Sub-module directory without module.md.
    std::fs::create_dir_all(
        ito.join("modules")
            .join("024_backend")
            .join("sub")
            .join("01_auth"),
    )
    .unwrap();

    let (_name, r) = validate_module(&module_repo, &ito, "024", false).unwrap();
    // In non-strict mode, missing module.md is a warning.
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("missing module.md")),
        "expected missing module.md warning; issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_module_errors_when_sub_module_has_invalid_naming() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    // Parent module.
    write(
        &ito.join("modules").join("024_backend").join("module.md"),
        "# Backend\n\n## Purpose\nBackend module for the application.\n\n## Scope\n- *\n\n## Changes\n",
    );

    // Sub-module with invalid naming (no underscore separator).
    std::fs::create_dir_all(
        ito.join("modules")
            .join("024_backend")
            .join("sub")
            .join("badname"),
    )
    .unwrap();

    let (_name, r) = validate_module(&module_repo, &ito, "024", false).unwrap();
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("SS_name convention")),
        "expected naming convention error; issues: {:?}",
        r.issues
    );
}

#[test]
fn validate_module_warns_when_sub_module_purpose_too_short() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    // Parent module.
    write(
        &ito.join("modules").join("024_backend").join("module.md"),
        "# Backend\n\n## Purpose\nBackend module for the application.\n\n## Scope\n- *\n\n## Changes\n",
    );

    // Sub-module with too-short purpose.
    write(
        &ito.join("modules")
            .join("024_backend")
            .join("sub")
            .join("01_auth")
            .join("module.md"),
        "# Auth\n\n## Purpose\nShort.\n\n## Scope\n- *\n\n## Changes\n",
    );

    let (_name, r) = validate_module(&module_repo, &ito, "024", false).unwrap();
    assert!(
        r.issues
            .iter()
            .any(|i| i.level == "WARNING" && i.message.contains("too brief")),
        "expected too-brief purpose warning; issues: {:?}",
        r.issues
    );
}
