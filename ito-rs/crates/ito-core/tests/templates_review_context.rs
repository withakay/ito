use ito_config::ConfigContext;
use ito_core::templates::compute_review_context;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

#[test]
fn compute_review_context_collects_artifacts_validation_tasks_and_specs() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    let change = "000-42_review-change";
    let change_dir = ito_path.join("changes").join(change);
    std::fs::create_dir_all(&change_dir).expect("create change dir");

    write(
        &change_dir.join(".ito.yaml"),
        "schema: demo\nmodule: 000_ungrouped\n",
    );
    write(
        &ito_path.join("modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for ad-hoc changes.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-42_review-change\n",
    );

    write(
        &project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        r#"name: demo
version: 1
description: demo
artifacts:
  - id: proposal
    generates: proposal.md
    template: proposal.md
    requires: []
  - id: design
    generates: design.md
    template: design.md
    requires: []
  - id: tasks
    generates: tasks.md
    template: tasks.md
    requires: []
  - id: specs
    generates: specs/**/*.md
    template: spec.md
    requires: []
"#,
    );

    write(
        &ito_path.join("specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nA sufficiently long purpose line for validation.\n\n## Requirements\n\n### Requirement: Alpha baseline\nThe system SHALL support alpha baseline behavior.\n\n#### Scenario: Baseline works\n- **WHEN** alpha is triggered\n- **THEN** baseline behavior occurs\n",
    );
    write(
        &change_dir.join("proposal.md"),
        "## Why\nNeed review context\n\n## What Changes\n- Adds review instruction support\n\n## Impact\n- Better review guidance\n",
    );
    write(
        &change_dir.join("tasks.md"),
        "### Task 1.1: Build context\n- **Status**: [x] complete\n\n### Task 1.2: Wire CLI\n- **Status**: [>] in-progress\n\n### Task 1.3: Add tests\n- **Status**: [ ] pending\n",
    );
    write(
        &change_dir.join("specs/alpha/spec.md"),
        "## MODIFIED Requirements\n\n### Requirement: Alpha baseline\nThe system SHALL include review context output.\n\n#### Scenario: Review instruction generation\n- **WHEN** the user requests review instructions\n- **THEN** the review template is rendered\n",
    );

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    let review =
        compute_review_context(&ito_path, change, Some("demo"), &ctx).expect("review context");

    assert_eq!(review.change_name, change);
    assert_eq!(review.schema_name, "demo");
    assert!(
        review
            .artifacts
            .iter()
            .any(|a| a.id == "proposal" && a.present)
    );
    assert!(
        review
            .artifacts
            .iter()
            .any(|a| a.id == "design" && !a.present)
    );

    let task_summary = review.task_summary.expect("task summary should exist");
    assert_eq!(task_summary.total, 3);
    assert_eq!(task_summary.complete, 1);
    assert_eq!(task_summary.in_progress, 1);
    assert_eq!(task_summary.pending, 1);

    assert!(
        review
            .affected_specs
            .iter()
            .any(|s| s.spec_id == "alpha" && s.operation == "MODIFIED")
    );
    assert!(!review.generated_at.is_empty());
}
