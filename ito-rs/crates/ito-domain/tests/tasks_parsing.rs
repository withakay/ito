use chrono::TimeZone;
use ito_domain::tasks;

#[test]
fn detect_tasks_format_enhanced_vs_checkbox() {
    let enhanced = "### Task 1.1: Hello\n- **Status**: [ ] pending\n";
    assert_eq!(
        tasks::detect_tasks_format(enhanced),
        tasks::TasksFormat::Enhanced
    );

    let checkbox = "- [ ] one\n- [x] two\n";
    assert_eq!(
        tasks::detect_tasks_format(checkbox),
        tasks::TasksFormat::Checkbox
    );

    let checkbox_in_progress = "- [~] one\n";
    assert_eq!(
        tasks::detect_tasks_format(checkbox_in_progress),
        tasks::TasksFormat::Checkbox
    );

    let unknown = "# Just text\n";
    assert_eq!(
        tasks::detect_tasks_format(unknown),
        tasks::TasksFormat::Checkbox
    );
}

#[test]
fn parse_checkbox_tasks_supports_dash_and_star() {
    let md = "- [x] done\n* [~] doing\n* [ ] todo\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 3);
    assert_eq!(parsed.tasks[0].status, tasks::TaskStatus::Complete);
    assert_eq!(parsed.tasks[1].status, tasks::TaskStatus::InProgress);
    assert_eq!(parsed.tasks[2].status, tasks::TaskStatus::Pending);

    let (ready, blocked) = tasks::compute_ready_and_blocked(&parsed);
    assert_eq!(blocked.len(), 0);
    assert_eq!(ready.len(), 0);
}

#[test]
fn parse_checkbox_tasks_accepts_right_arrow_in_progress_marker() {
    let md = "- [>] doing\n- [ ] todo\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.tasks[0].status, tasks::TaskStatus::InProgress);
    assert_eq!(parsed.tasks[1].status, tasks::TaskStatus::Pending);
}

#[test]
fn update_checkbox_task_status_sets_marker_and_preserves_text() {
    let md = "## Tasks\n- [ ] first\n- [x] done\n";
    let out = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::InProgress).unwrap();
    assert!(out.contains("- [~] first"));
    assert!(out.contains("- [x] done"));
}

#[test]
fn parse_enhanced_tasks_parses_fields_and_action_block() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: Do it
- **Files**: `a.rs, b.rs`
- **Dependencies**: None
- **Action**:
  line one
  line two
- **Verify**: `cargo test`
- **Done When**: it works
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Enhanced);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.waves.len(), 1);
    assert!(parsed.diagnostics.is_empty());

    let t = &parsed.tasks[0];
    assert_eq!(t.id, "1.1");
    assert_eq!(t.name, "Do it");
    assert_eq!(t.wave, Some(1));
    assert_eq!(t.status, tasks::TaskStatus::Pending);
    assert_eq!(t.files, vec!["a.rs".to_string(), "b.rs".to_string()]);
    assert_eq!(t.action, "line one\nline two");
    assert_eq!(t.verify.as_deref(), Some("cargo test"));
    assert_eq!(t.done_when.as_deref(), Some("it works"));
    assert_eq!(t.updated_at.as_deref(), Some("2026-01-28"));
}

#[test]
fn parse_enhanced_tasks_accepts_all_prior_tasks_dependency_shorthand() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: A
- **Files**: `a.rs`
- **Dependencies**: All prior tasks
- **Action**:
  do the thing
- **Verify**: `cargo test`
- **Done When**: done
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].dependencies, Vec::<String>::new());
}

#[test]
fn parse_enhanced_tasks_accepts_wave_heading_titles() {
    let md = r#"
## Wave 1: Foundations
- **Depends On**: None

### Task 1.1: A
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending

## Wave 2 - Next
- **Depends On**: Wave 1

### Task 2.1: B
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.waves.len(), 2);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.tasks[0].wave, Some(1));
    assert_eq!(parsed.tasks[1].wave, Some(2));
}

#[test]
fn enhanced_tasks_diagnostics_cover_common_errors() {
    let md = r#"
## Wave 1

### Task 1.1: Missing fields
- **Dependencies**: Task 9.9
- **Updated At**: 2026-99-99
- **Status**: [ ] pending

## Wave 2
- **Depends On**: bananas

### Task 2.1: Bad status
- **Dependencies**: None
- **Updated At**: 2026-01-28
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Enhanced);
    assert_eq!(parsed.tasks.len(), 2);

    let mut has_missing_dep = false;
    let mut has_invalid_date = false;
    let mut has_invalid_depends_on = false;
    let mut has_missing_status = false;
    let mut has_missing_depends_on_line = false;
    for d in &parsed.diagnostics {
        if d.message.contains("Missing dependency") {
            has_missing_dep = true;
        }
        if d.message.contains("Invalid Updated At date") {
            has_invalid_date = true;
        }
        if d.message.contains("invalid Depends On entry") {
            has_invalid_depends_on = true;
        }
        if d.message.contains("Invalid or missing status") {
            has_missing_status = true;
        }
        if d.message.contains("missing Depends On line") {
            has_missing_depends_on_line = true;
        }
    }

    assert!(has_missing_dep);
    assert!(has_invalid_date);
    assert!(has_invalid_depends_on);
    assert!(has_missing_status);
    assert!(has_missing_depends_on_line);
}

#[test]
fn enhanced_tasks_wave_gating_blocks_later_waves() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: A
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending

## Wave 2
- **Depends On**: Wave 1

### Task 2.1: B
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    let (ready, blocked) = tasks::compute_ready_and_blocked(&parsed);

    assert!(ready.iter().any(|t| t.id == "1.1"));
    assert!(blocked.iter().any(|(t, _)| t.id == "2.1"));
}

#[test]
fn enhanced_tasks_cycles_and_shelved_deps_are_reported() {
    let md = r#"
## Wave 1
- **Depends On**: Wave 1

### Task 1.1: A
- **Dependencies**: Task 1.2
- **Updated At**: 2026-01-28
- **Status**: [ ] pending

### Task 1.2: B
- **Dependencies**: Task 1.1
- **Updated At**: 2026-01-28
- **Status**: [-] shelved
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    let mut has_wave_self_dep = false;
    let mut has_dep_cycle = false;
    let mut has_shelved_dep = false;
    for d in &parsed.diagnostics {
        if d.message.contains("cannot depend on itself") {
            has_wave_self_dep = true;
        }
        if d.message.contains("Dependency cycle detected") {
            has_dep_cycle = true;
        }
        if d.message.contains("Dependency is shelved") {
            has_shelved_dep = true;
        }
    }

    assert!(has_wave_self_dep);
    assert!(has_dep_cycle);
    assert!(has_shelved_dep);
}

#[test]
fn update_enhanced_task_status_inserts_missing_fields() {
    let md = "## Wave 1\n\n### Task 1.1: A\n- **Dependencies**: None\n\n";
    let now = chrono::Local
        .with_ymd_and_hms(2026, 1, 28, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::Complete, now);
    assert!(out.contains("- **Updated At**: 2026-01-28"));
    assert!(out.contains("- **Status**: [x] complete"));
}

#[test]
fn tasks_path_checked_rejects_traversal_like_change_ids() {
    let root = std::path::Path::new("/tmp/repo/.ito");
    assert!(tasks::tasks_path_checked(root, "../escape").is_none());
    assert!(tasks::tasks_path_checked(root, "a/b").is_none());
    assert!(tasks::tasks_path_checked(root, "a\\b").is_none());
}

#[test]
fn tasks_path_uses_safe_fallback_for_invalid_change_id() {
    let root = std::path::Path::new("/tmp/repo/.ito");
    let path = tasks::tasks_path(root, "../escape");
    assert!(path.ends_with("changes/invalid-change-id/tasks.md"));
}

#[test]
fn parse_checkbox_tasks_assigns_sequential_ids_when_not_explicit() {
    let md = "- [ ] First task\n- [x] Second task\n- [ ] Third task\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 3);
    assert_eq!(parsed.tasks[0].id, "1");
    assert_eq!(parsed.tasks[1].id, "2");
    assert_eq!(parsed.tasks[2].id, "3");
}

#[test]
fn parse_checkbox_tasks_preserves_explicit_ids() {
    let md = "- [ ] 1.1 First\n- [x] 1.2 Second\n- [ ] 1.3 Third\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 3);
    assert_eq!(parsed.tasks[0].id, "1.1");
    assert_eq!(parsed.tasks[1].id, "1.2");
    assert_eq!(parsed.tasks[2].id, "1.3");
}

#[test]
fn parse_checkbox_tasks_handles_mixed_explicit_and_implicit_ids() {
    let md = "- [ ] 1.1 Explicit\n- [x] Implicit\n- [ ] 2.3 Another explicit\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 3);
    assert_eq!(parsed.tasks[0].id, "1.1");
    assert_eq!(parsed.tasks[1].id, "2"); // Sequential ID
    assert_eq!(parsed.tasks[2].id, "2.3");
}

#[test]
fn parse_checkbox_tasks_handles_empty_lines_and_non_checkbox_content() {
    let md = "# My Tasks\n\n- [ ] Task one\n\nSome text\n\n- [x] Task two\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.tasks[0].name, "Task one");
    assert_eq!(parsed.tasks[1].name, "Task two");
}

#[test]
fn parse_checkbox_tasks_uppercase_x_marks_complete() {
    let md = "- [X] Done\n- [x] Also done\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.tasks[0].status, tasks::TaskStatus::Complete);
    assert_eq!(parsed.tasks[1].status, tasks::TaskStatus::Complete);
}

#[test]
fn parse_enhanced_tasks_handles_task_without_optional_prefix() {
    let md = r#"
## Wave 1
- **Depends On**: None

### 1.1: Do it
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Enhanced);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].id, "1.1");
}

#[test]
fn parse_enhanced_tasks_handles_multiple_files() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: Multi-file task
- **Files**: `a.rs, b.rs, c.rs`
- **Dependencies**: None
- **Action**:
  Update multiple files
- **Verify**: `cargo test`
- **Done When**: All updated
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].files, vec!["a.rs", "b.rs", "c.rs"]);
}

#[test]
fn parse_enhanced_tasks_handles_multiline_action() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: Complex action
- **Dependencies**: None
- **Action**:
  First line
  Second line
  Third line
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(
        parsed.tasks[0].action,
        "First line\nSecond line\nThird line"
    );
}

#[test]
fn parse_enhanced_tasks_progress_counts_all_statuses() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: Complete
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [x] complete

### Task 1.2: InProgress
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [>] in-progress

### Task 1.3: Pending
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending

### Task 1.4: Shelved
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [-] shelved
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.progress.total, 4);
    assert_eq!(parsed.progress.complete, 1);
    assert_eq!(parsed.progress.in_progress, 1);
    assert_eq!(parsed.progress.pending, 1);
    assert_eq!(parsed.progress.shelved, 1);
    assert_eq!(parsed.progress.remaining, 2); // total - (complete + shelved)
}

#[test]
fn parse_enhanced_tasks_handles_empty_dependencies_field() {
    let md = r#"
## Wave 1
- **Depends On**: None

### Task 1.1: No deps
- **Dependencies**:
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert!(parsed.tasks[0].dependencies.is_empty());
}

#[test]
fn parse_enhanced_tasks_handles_wave_with_comma_in_title() {
    let md = r#"
## Wave 1: Setup, Configuration, and Initialization
- **Depends On**: None

### Task 1.1: First
- **Dependencies**: None
- **Updated At**: 2026-01-28
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.waves.len(), 1);
    assert_eq!(parsed.waves[0].wave, 1);
}

#[test]
fn update_enhanced_task_status_preserves_existing_fields() {
    let md = r#"
## Wave 1

### Task 1.1: Test
- **Files**: `a.rs, b.rs`
- **Dependencies**: None
- **Action**:
  Do the thing
- **Verify**: `cargo test`
- **Done When**: Done
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;
    let now = chrono::Local
        .with_ymd_and_hms(2026, 2, 15, 0, 0, 0)
        .unwrap();
    let out = tasks::update_enhanced_task_status(md, "1.1", tasks::TaskStatus::InProgress, now);

    // Should update status and date but preserve other fields
    assert!(out.contains("- **Files**: `a.rs, b.rs`"));
    assert!(out.contains("- **Dependencies**: None"));
    assert!(out.contains("Do the thing"));
    assert!(out.contains("- **Verify**: `cargo test`"));
    assert!(out.contains("- **Done When**: Done"));
    assert!(out.contains("- **Updated At**: 2026-02-15"));
    assert!(out.contains("- **Status**: [>] in-progress"));
}

#[test]
fn update_checkbox_task_status_by_explicit_id() {
    let md = "- [ ] 1.1 First\n- [ ] 1.2 Second\n";
    let out = tasks::update_checkbox_task_status(md, "1.2", tasks::TaskStatus::Complete).unwrap();
    assert!(out.contains("- [ ] 1.1 First"));
    assert!(out.contains("- [x] 1.2 Second"));
}

#[test]
fn update_checkbox_task_status_preserves_bullet_style() {
    let md = "- [ ] first\n* [ ] second\n";
    let out1 = tasks::update_checkbox_task_status(md, "1", tasks::TaskStatus::Complete).unwrap();
    assert!(out1.contains("- [x] first"));

    let out2 = tasks::update_checkbox_task_status(md, "2", tasks::TaskStatus::Complete).unwrap();
    assert!(out2.contains("* [x] second"));
}
