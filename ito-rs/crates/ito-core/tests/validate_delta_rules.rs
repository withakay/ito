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
    assert_eq!(
        ui_warnings.len(),
        1,
        "expected one UI warning, got: {:?}",
        report.issues
    );
    assert!(ui_warnings[0].message.contains("UI mechanics"));
}

#[test]
fn ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error() {
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
      ui_mechanics: error
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
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    let ui_warnings: Vec<_> = report
        .issues
        .iter()
        .filter(|issue| issue.rule_id.as_deref() == Some("ui_mechanics"))
        .collect();
    assert_eq!(
        ui_warnings.len(),
        1,
        "expected one UI warning, got: {:?}",
        report.issues
    );
    assert_eq!(ui_warnings[0].level, "WARNING");
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
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    capabilities_consistency: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Capabilities\n\n### New Capabilities\n\n- `auth`: Add login behavior.\n",
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
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    capabilities_consistency: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Capabilities\n\n### New Capabilities\n\n<!-- None -->\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("notifications")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Notify users\nThe system SHALL notify users.\n\n#### Scenario: Notify\n- **WHEN** an event occurs\n- **THEN** a notification is sent\n",
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
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    capabilities_consistency: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Capabilities\n\n### New Capabilities\n\n- `auth`: Claimed as new.\n\n### Modified Capabilities\n\n- `payments`: Claimed as modified.\n",
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
        "## MODIFIED Requirements\n\n### Requirement: Auth changes\nThe system SHALL modify auth.\n\n#### Scenario: Auth\n- **WHEN** auth changes\n- **THEN** auth works\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("payments")
            .join("spec.md"),
        "## MODIFIED Requirements\n\n### Requirement: Payment changes\nThe system SHALL modify payments.\n\n#### Scenario: Payments\n- **WHEN** payments change\n- **THEN** payments work\n",
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
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    capabilities_consistency: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Capabilities\n\n### New Capabilities\n\n- `<name>`: Placeholder entry.\n- Plain bullet without code token.\n- <!-- ignored -->\n",
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
fn contract_refs_rule_accepts_known_schemes_and_emits_single_advisory() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("schema.yaml"),
        "name: contract-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\n    rules:\n      contract_refs: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: contract-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Contract refs\nThe system SHALL reference external contracts.\n\n- **Contract Refs**: openapi:POST /v1/password-reset, jsonschema:PasswordResetRequest\n\n#### Scenario: Valid refs\n- **WHEN** validation runs\n- **THEN** refs are accepted\n\n### Requirement: Second ref\nThe system SHALL allow more refs.\n\n- **Contract Refs**: asyncapi:user.created\n\n#### Scenario: Async ref\n- **WHEN** validation runs\n- **THEN** refs are accepted\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    let advisory_count = report
        .issues
        .iter()
        .filter(|issue| {
            issue.rule_id.as_deref() == Some("contract_refs")
                && issue.level == "INFO"
                && issue
                    .message
                    .contains("contract resolution is not configured")
        })
        .count();
    assert_eq!(
        advisory_count, 1,
        "expected one advisory, got: {:?}",
        report.issues
    );
    assert!(
        !report.issues.iter().any(
            |issue| issue.rule_id.as_deref() == Some("contract_refs") && issue.level == "ERROR"
        ),
        "known schemes should not error, got issues: {:?}",
        report.issues
    );
}

#[test]
fn contract_refs_rule_rejects_unknown_schemes() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("schema.yaml"),
        "name: contract-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\n    rules:\n      contract_refs: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: contract-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Invalid scheme\nThe system SHALL reject unknown schemes.\n\n- **Contract Refs**: graphql:UserQuery\n\n#### Scenario: Invalid ref\n- **WHEN** validation runs\n- **THEN** it errors\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("contract_refs")
            && issue.level == "ERROR"
            && issue
                .message
                .contains("Unknown contract ref scheme 'graphql'")
    }));
}

#[test]
fn contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("schema.yaml"),
        "name: contract-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("contract-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\n    rules:\n      contract_refs: error\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: contract-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## What Changes\n\n- Introduce a public API.\n\n## Change Shape\n\n- **Type**: feature\n- **Risk**: medium\n- **Stateful**: no\n- **Public Contract**: openapi\n- **Design Needed**: no\n- **Design Reason**: API surface is small.\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Public contract missing anchor\nThe system SHALL declare a public contract.\n\n#### Scenario: No refs\n- **WHEN** validation runs\n- **THEN** it warns\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("contract_refs")
            && issue.level == "WARNING"
            && issue.message.contains("Public Contract facet 'openapi'")
    }));
}
