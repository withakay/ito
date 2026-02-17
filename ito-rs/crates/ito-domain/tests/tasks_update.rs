use ito_domain::tasks;

#[test]
fn update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting() {
    let md = "# Tasks\n\n  - [ ] first\n\t* [ ] second\n- [x] third\n";

    let out = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::InProgress)
        .expect("expected update to succeed");

    assert!(out.ends_with('\n'));
    assert!(out.contains("  - [ ] first"));
    assert!(out.contains("\t* [~] second"));
    assert!(out.contains("- [x] third"));
}

#[test]
fn update_checkbox_task_status_rejects_shelving() {
    let md = "- [ ] one\n";
    let err = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Shelved)
        .expect_err("expected error");
    assert!(err.to_lowercase().contains("does not support"));
}

#[test]
fn update_checkbox_task_status_errors_for_invalid_or_missing_task_id() {
    let md = "- [ ] one\n";

    assert!(tasks::update_checkbox_task_status(md, "nope", tasks::TaskStatus::Complete).is_err());
    assert!(tasks::update_checkbox_task_status(md, "0", tasks::TaskStatus::Complete).is_err());
    assert!(tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Complete).is_err());
}

#[test]
fn update_checkbox_task_status_handles_deeply_indented_items() {
    let md = "        - [ ] deeply indented\n";
    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Complete)
        .expect("should succeed");
    assert!(out.contains("        - [x] deeply indented"));
}

#[test]
fn update_checkbox_task_status_handles_mixed_spaces_and_tabs() {
    let md = "  \t  - [ ] mixed whitespace\n";
    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Complete)
        .expect("should succeed");
    assert!(out.contains("  \t  - [x] mixed whitespace"));
}

#[test]
fn update_checkbox_task_status_handles_task_with_trailing_whitespace() {
    let md = "- [ ] task with spaces    \n";
    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Complete)
        .expect("should succeed");
    assert!(out.contains("- [x] task with spaces    "));
}

#[test]
fn update_checkbox_task_status_transitions_all_statuses() {
    let md = "- [ ] task\n";

    // Pending -> InProgress
    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::InProgress).unwrap();
    assert!(out.contains("- [~] task"));

    // InProgress -> Complete
    let out = tasks::update_checkbox_task_status(&out, "1", tasks::TaskStatus::Complete).unwrap();
    assert!(out.contains("- [x] task"));

    // Complete -> Pending (unusual but valid)
    let out = tasks::update_checkbox_task_status(&out, "1", tasks::TaskStatus::Pending).unwrap();
    assert!(out.contains("- [ ] task"));
}

#[test]
fn update_enhanced_task_status_handles_task_with_minimal_fields() {
    let md = "### Task 1.1: Minimal\n";
    let now = chrono::Local
        .with_ymd_and_hms(2026, 1, 28, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.contains("- **Updated At**: 2026-01-28"));
    assert!(out.contains("- **Status**: [x] complete"));
}

#[test]
fn update_enhanced_task_status_handles_multiple_tasks() {
    let md = r#"
### Task 1.1: First
- **Updated At**: 2026-01-28
- **Status**: [ ] pending

### Task 1.2: Second
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;
    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.2", tasks::TaskStatus::Complete, now);

    // First task should be unchanged
    assert!(out.contains("### Task 1.1: First"));
    // Second task should be updated
    assert!(out.contains("### Task 1.2: Second"));
    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [x] complete"));
}

#[test]
fn update_enhanced_task_status_with_all_status_markers() {
    let md = "### Task 1.1: Test\n- **Updated At**: 2026-01-01\n- **Status**: [ ] pending\n";
    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    // Test each status marker
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::InProgress, now);
    assert!(out.contains("- **Status**: [ ] in-progress"));

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.contains("- **Status**: [x] complete"));

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Shelved, now);
    assert!(out.contains("- **Status**: [-] shelved"));

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Pending, now);
    assert!(out.contains("- **Status**: [ ] pending"));
}

#[test]
fn update_enhanced_task_status_preserves_trailing_newline() {
    let md = "### Task 1.1: Test\n- **Status**: [ ] pending\n";
    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.ends_with('\n'));
}

#[test]
fn update_enhanced_task_status_handles_task_before_wave_heading() {
    // Edge case: task appears before wave heading ends
    let md = r#"
### Task 1.1: Task
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

## Wave 2
"#;
    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.contains("- **Status**: [x] complete"));
    assert!(out.contains("## Wave 2"));
}