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
fn task_quality_rule_errors_on_missing_status() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("schema.yaml"),
        "name: tracking-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\napply:\n  tracks: tasks.md\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\ntracking:\n  source: apply_tracks\n  required: true\n  validate_as: ito.tasks-tracking.v1\n  rules:\n    task_quality: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracking-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Known requirement\nThe system SHALL track tasks.\n\n- **Requirement ID**: auth:known\n\n#### Scenario: Track tasks\n- **WHEN** validation runs\n- **THEN** task requirements resolve\n",
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Missing status\n- **Files**: `src/lib.rs`\n- **Dependencies**: None\n- **Action**:\n  Update validator behavior.\n- **Verify**: `cargo test -p ito-core --test validate task_quality_rule`\n- **Done When**: Status is present.\n- **Requirements**: auth:known\n- **Updated At**: 2026-04-25\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("task_quality")
            && issue.level == "ERROR"
            && issue.message.contains("Missing Status")
    }));
}

#[test]
fn task_quality_rule_enforces_done_when_and_verify_for_impl_tasks() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("schema.yaml"),
        "name: tracking-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\napply:\n  tracks: tasks.md\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\ntracking:\n  source: apply_tracks\n  required: true\n  validate_as: ito.tasks-tracking.v1\n  rules:\n    task_quality: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracking-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Known requirement\nThe system SHALL track tasks.\n\n- **Requirement ID**: auth:known\n\n#### Scenario: Track tasks\n- **WHEN** validation runs\n- **THEN** task requirements resolve\n",
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Missing quality fields\n- **Files**: `src/lib.rs`\n- **Dependencies**: None\n- **Status**: [ ] pending\n- **Updated At**: 2026-04-25\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "ERROR"
                && issue.message.contains("Missing Done When"))
    );
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "ERROR"
                && issue.message.contains("Missing Verify"))
    );
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "WARNING"
                && issue.message.contains("Missing Action"))
    );
}

#[test]
fn task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("schema.yaml"),
        "name: tracking-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\napply:\n  tracks: tasks.md\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\ntracking:\n  source: apply_tracks\n  required: true\n  validate_as: ito.tasks-tracking.v1\n  rules:\n    task_quality: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracking-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Known requirement\nThe system SHALL track tasks.\n\n- **Requirement ID**: auth:known\n\n#### Scenario: Track tasks\n- **WHEN** validation runs\n- **THEN** task requirements resolve\n",
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Vague verify\n- **Dependencies**: None\n- **Verify**: `Run Tests`\n- **Done When**: The docs task is reviewed.\n- **Status**: [ ] pending\n- **Updated At**: 2026-04-25\n\n### Task 1.2: Docs task without verify\n- **Files**: `docs/schema-customization.md`\n- **Dependencies**: None\n- **Action**:\n  Document the workflow.\n- **Done When**: Docs are updated.\n- **Status**: [ ] pending\n- **Updated At**: 2026-04-25\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "WARNING"
                && issue.message.contains("Vague Verify"))
    );
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "WARNING"
                && issue.message.contains("Missing Files"))
    );
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "WARNING"
                && issue.message.contains("Missing Action"))
    );
    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("task_quality")
                && issue.level == "WARNING"
                && issue.message.contains("Missing Verify")
                && issue.message.contains("1.2"))
    );
}

#[test]
fn task_quality_rule_errors_on_unknown_requirement_ids() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("schema.yaml"),
        "name: tracking-rules\nversion: 1\nartifacts:\n  - id: specs\n    generates: specs/**/*.md\n    template: specs/spec.md\n    requires: []\napply:\n  tracks: tasks.md\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("tracking-rules")
            .join("validation.yaml"),
        "version: 1\nartifacts:\n  specs:\n    required: true\n    validate_as: ito.delta-specs.v1\ntracking:\n  source: apply_tracks\n  required: true\n  validate_as: ito.tasks-tracking.v1\n  rules:\n    task_quality: error\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: tracking-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Known requirement\nThe system SHALL track tasks.\n\n- **Requirement ID**: auth:known\n\n#### Scenario: Track tasks\n- **WHEN** validation runs\n- **THEN** task requirements resolve\n",
    );
    write(
        &ito.join("changes").join(change_id).join("tasks.md"),
        "## Wave 1\n- **Depends On**: None\n\n### Task 1.1: Unknown requirement id\n- **Files**: `src/lib.rs`\n- **Dependencies**: None\n- **Action**:\n  Update validator behavior.\n- **Verify**: `cargo test -p ito-core --test validate task_quality_rule`\n- **Done When**: Requirement ids resolve.\n- **Requirements**: auth:missing\n- **Status**: [ ] pending\n- **Updated At**: 2026-04-25\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(report.issues.iter().any(|issue| {
        issue.rule_id.as_deref() == Some("task_quality")
            && issue.level == "ERROR"
            && issue
                .message
                .contains("unknown requirement ID 'auth:missing'")
    }));
}
