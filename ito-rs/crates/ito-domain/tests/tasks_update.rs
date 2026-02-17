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
fn update_checkbox_task_status_matches_explicit_ids_over_index() {
    let md = "- [ ] 1.1 First task\n- [ ] 1.2 Second task\n- [ ] 1.3 Third task\n";

    let out = tasks::update_checkbox_task_status(md, "1.2", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");

    assert!(out.contains("- [ ] 1.1 First task"));
    assert!(out.contains("- [x] 1.2 Second task"));
    assert!(out.contains("- [ ] 1.3 Third task"));
}

#[test]
fn update_checkbox_task_status_handles_mixed_explicit_and_implicit_ids() {
    let md = "- [ ] 1 First\n- [ ] Second\n- [ ] 3 Third\n";

    // Update by explicit ID
    let out = tasks::update_checkbox_task_status(md, "3", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out.contains("- [x] 3 Third"));

    // Update by index (second item)
    let out = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::InProgress)
        .expect("expected update to succeed");
    assert!(out.contains("- [~] Second"));
}

#[test]
fn update_checkbox_task_status_preserves_bullet_style() {
    let md = "- [ ] dash\n* [ ] star\n";

    let out1 = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out1.contains("- [x] dash"));

    let out2 = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out2.contains("* [x] star"));
}

#[test]
fn update_checkbox_task_status_handles_various_markers() {
    let md = "- [ ] pending\n- [x] complete\n- [X] complete_upper\n- [~] in_progress\n";

    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::InProgress)
        .expect("expected update to succeed");
    assert!(out.contains("- [~] pending"));

    let out = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Pending)
        .expect("expected update to succeed");
    assert!(out.contains("- [ ] complete"));

    let out = tasks::update_checkbox_task_status(md, "4", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out.contains("- [x] in_progress"));
}

#[test]
fn update_checkbox_task_status_handles_unicode_in_task_text() {
    let md = "- [ ] 测试任务\n- [ ] Tâche française\n- [ ] 🚀 Rocket\n";

    let out = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out.contains("- [x] Tâche française"));
}

#[test]
fn update_checkbox_task_status_with_id_suffix_colon() {
    let md = "- [ ] 1.1: First task\n- [ ] 2.1: Second task\n";

    let out = tasks::update_checkbox_task_status(md, "1.1", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out.contains("- [x] 1.1: First task"));
}

#[test]
fn update_checkbox_task_status_with_id_suffix_dot() {
    let md = "- [ ] 1.1. First task\n- [ ] 2.1. Second task\n";

    let out = tasks::update_checkbox_task_status(md, "2.1", tasks::TaskStatus::Complete)
        .expect("expected update to succeed");
    assert!(out.contains("- [x] 2.1. Second task"));
}

#[test]
fn update_enhanced_task_status_updates_status_and_date() {
    let md = r#"## Wave 1

### Task 1.1: Do something
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [x] complete"));
}

#[test]
fn update_enhanced_task_status_handles_in_progress() {
    let md = r#"### Task 1.1: Do something
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::InProgress, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [ ] in-progress"));
}

#[test]
fn update_enhanced_task_status_handles_shelved() {
    let md = r#"### Task 1.1: Do something
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Shelved, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [-] shelved"));
}

#[test]
fn update_enhanced_task_status_inserts_missing_fields() {
    let md = "### Task 1.1: Do something\n- **Dependencies**: None\n";

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [x] complete"));
}

#[test]
fn update_enhanced_task_status_preserves_other_fields() {
    let md = r#"### Task 1.1: Do something
- **Files**: `a.rs, b.rs`
- **Dependencies**: Task 1.2
- **Action**:
  Do this and that
- **Verify**: `cargo test`
- **Done When**: It works
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

### Task 1.2: Other task
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);

    assert!(out.contains("- **Files**: `a.rs, b.rs`"));
    assert!(out.contains("- **Dependencies**: Task 1.2"));
    assert!(out.contains("Do this and that"));
    assert!(out.contains("- **Verify**: `cargo test`"));
    assert!(out.contains("- **Done When**: It works"));
    assert!(out.contains("### Task 1.2: Other task"));
}

#[test]
fn update_enhanced_task_status_only_updates_specified_task() {
    let md = r#"### Task 1.1: First
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

### Task 1.2: Second
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);

    assert!(out.contains("### Task 1.1: First"));
    assert!(out.contains("- **Status**: [x] complete"));

    // Second task should be unchanged
    let lines: Vec<&str> = out.lines().collect();
    let second_task_start = lines.iter().position(|&l| l.contains("### Task 1.2")).unwrap();
    let second_status_line = lines[second_task_start..]
        .iter()
        .find(|&&l| l.contains("- **Status**:"))
        .unwrap();
    assert!(second_status_line.contains("[ ] pending"));
}

#[test]
fn update_enhanced_task_status_handles_task_prefix_optional() {
    let md = r#"### 1.1: Without Task prefix
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [x] complete"));
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
fn update_enhanced_task_status_handles_complex_task_ids() {
    let md = r#"### Task 10.20: Complex ID
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();

    let out = tasks::update_enhanced_task_status(md, "10.20", tasks::TaskStatus::Complete, now);

    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [x] complete"));
}