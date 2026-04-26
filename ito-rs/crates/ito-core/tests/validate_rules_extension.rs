use ito_core::change_repository::FsChangeRepository;
use ito_core::validate::validate_change;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
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
                && issue
                    .message
                    .contains("Unknown validation rule 'not_a_real_rule'")
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
                && issue
                    .message
                    .contains("Unknown validation rule 'unknown_proposal_rule'")
        }),
        "expected proposal rule warning, got issues: {:?}",
        report.issues
    );
}
