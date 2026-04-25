use ito_domain::tasks;

#[test]
fn quality_fields_round_trip_when_present() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.0: Prepare parser
- **Verify**: `cargo test -p ito-domain quality_fields`
- **Done When**: Parser baseline is ready.
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.2: Review validator inputs
- **Verify**: `cargo test -p ito-domain quality_fields`
- **Done When**: Downstream validation has input coverage.
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.1: Capture quality metadata
- **Files**: `src/lib.rs, Cargo.toml`
- **Dependencies**: Task 1.0, Task 1.2
- **Action**:
  Update the parser.
  Keep task metadata structured.
- **Verify**: `cargo test -p ito-domain tasks::enhanced::quality_fields`
- **Done When**: All quality fields are available to validators.
- **Requirements**: tasks-tracking:quality-critical-fields, tasks-tracking:concrete-verification
- **Updated At**: 2026-04-25
- **Status**: [>] in-progress
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(parsed.diagnostics.is_empty(), "unexpected diagnostics: {:?}", parsed.diagnostics);
    assert_eq!(parsed.tasks.len(), 3);

    let task = parsed
        .tasks
        .iter()
        .find(|task| task.id == "1.1")
        .expect("task 1.1 should be present");
    assert_eq!(task.files, vec!["src/lib.rs", "Cargo.toml"]);
    assert_eq!(task.dependencies, vec!["1.0", "1.2"]);
    assert_eq!(task.action, "Update the parser.\nKeep task metadata structured.");
    assert_eq!(
        task.verify.as_deref(),
        Some("cargo test -p ito-domain tasks::enhanced::quality_fields")
    );
    assert_eq!(
        task.done_when.as_deref(),
        Some("All quality fields are available to validators.")
    );
    assert_eq!(
        task.requirements,
        vec![
            "tasks-tracking:quality-critical-fields",
            "tasks-tracking:concrete-verification",
        ]
    );
    assert_eq!(task.status, tasks::TaskStatus::InProgress);
    assert_eq!(task.updated_at.as_deref(), Some("2026-04-25"));
}

#[test]
fn quality_fields_allow_missing_optional_metadata() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: Minimal metadata
- **Verify**: `cargo test -p ito-domain tasks::enhanced::quality_fields`
- **Done When**: Parsing succeeds without optional metadata.
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(parsed.diagnostics.is_empty(), "unexpected diagnostics: {:?}", parsed.diagnostics);
    assert_eq!(parsed.tasks.len(), 1);

    let task = &parsed.tasks[0];
    assert!(task.files.is_empty());
    assert!(task.dependencies.is_empty());
    assert!(task.action.is_empty());
    assert_eq!(
        task.verify.as_deref(),
        Some("cargo test -p ito-domain tasks::enhanced::quality_fields")
    );
    assert_eq!(
        task.done_when.as_deref(),
        Some("Parsing succeeds without optional metadata.")
    );
    assert!(task.requirements.is_empty());
    assert_eq!(task.status, tasks::TaskStatus::Pending);
    assert_eq!(task.updated_at.as_deref(), Some("2026-04-25"));
}
