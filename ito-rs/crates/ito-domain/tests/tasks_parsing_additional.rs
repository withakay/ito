//! Additional tests for task parsing edge cases and boundary conditions.

use ito_domain::tasks;

#[test]
fn parse_empty_file_returns_empty_result() {
    let parsed = tasks::parse_tasks_tracking_file("");
    assert_eq!(parsed.tasks.len(), 0);
    assert_eq!(parsed.waves.len(), 0);
    assert!(parsed.diagnostics.is_empty());
}

#[test]
fn parse_file_with_only_whitespace() {
    let md = "   \n\n\t\t\n   \n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 0);
    assert_eq!(parsed.waves.len(), 0);
}

#[test]
fn parse_file_with_only_non_task_content() {
    let md = "# Header\n\nSome text here.\n\n## Another heading\n\nMore text.\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 0);
}

#[test]
fn checkbox_format_handles_empty_task_text() {
    let md = "- [ ] \n- [x] Task with text\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Checkbox);
    assert_eq!(parsed.tasks.len(), 2);
    assert!(parsed.tasks[0].name.is_empty());
    assert_eq!(parsed.tasks[1].name, "Task with text");
}

#[test]
fn checkbox_format_handles_very_long_task_names() {
    let long_name = "a".repeat(1000);
    let md = format!("- [ ] {}\n", long_name);
    let parsed = tasks::parse_tasks_tracking_file(&md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].name, long_name);
}

#[test]
fn checkbox_format_handles_special_characters_in_task_names() {
    let md = "- [ ] Task with !@#$%^&*() special chars\n- [x] Task with <html> tags\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.tasks[0].name, "Task with !@#$%^&*() special chars");
    assert_eq!(parsed.tasks[1].name, "Task with <html> tags");
}

#[test]
fn checkbox_format_handles_newlines_in_adjacent_lines() {
    let md = "- [ ] First\n\n\n- [ ] Second\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 2);
}

#[test]
fn checkbox_format_ignores_incomplete_checkbox_patterns() {
    let md = "- [] no space\n- [ missing close\n- [?] wrong marker\n- [ ] valid\n";
    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].name, "valid");
}

#[test]
fn enhanced_format_handles_task_without_wave() {
    let md = r#"### Task 1.1: Orphan task
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.format, tasks::TasksFormat::Enhanced);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].wave, None);
    // Should have a warning about being outside wave section
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("outside any Wave section"))
    );
}

#[test]
fn enhanced_format_handles_duplicate_wave_numbers() {
    let md = r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

## Wave 1
- **Depends On**: None

### Task 1.2: Second
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 2);
    assert_eq!(parsed.waves.len(), 1);
    assert_eq!(parsed.tasks[0].wave, Some(1));
    assert_eq!(parsed.tasks[1].wave, Some(1));
}

#[test]
fn enhanced_format_handles_very_large_wave_numbers() {
    let md = r#"## Wave 999
- **Depends On**: None

### Task 999.1: Large wave
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].wave, Some(999));
    assert_eq!(parsed.waves.len(), 1);
    assert_eq!(parsed.waves[0].wave, 999);
}

#[test]
fn enhanced_format_handles_empty_action_block() {
    let md = r#"### Task 1.1: Test
- **Dependencies**: None
- **Action**:
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].action, "");
}

#[test]
fn enhanced_format_handles_multiline_action_with_code() {
    let md = r#"### Task 1.1: Test
- **Action**:
  ```rust
  fn main() {
      println!("Hello");
  }
  ```
  And then do more stuff.
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert!(parsed.tasks[0].action.contains("```rust"));
    assert!(parsed.tasks[0].action.contains("fn main()"));
    assert!(parsed.tasks[0].action.contains("And then do more stuff."));
}

#[test]
fn enhanced_format_handles_very_long_file_paths() {
    let long_path = format!("path/to/{}/file.rs", "sub/".repeat(100));
    let md = format!(
        "### Task 1.1: Test\n- **Files**: `{}`\n- **Updated At**: 2026-01-01\n- **Status**: [ ] pending\n",
        long_path
    );

    let parsed = tasks::parse_tasks_tracking_file(&md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].files[0], long_path);
}

#[test]
fn enhanced_format_handles_multiple_files_with_spaces() {
    let md = r#"### Task 1.1: Test
- **Files**: `file one.rs, file two.rs, file three.rs`
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].files.len(), 3);
    assert_eq!(parsed.tasks[0].files[0], "file one.rs");
    assert_eq!(parsed.tasks[0].files[1], "file two.rs");
    assert_eq!(parsed.tasks[0].files[2], "file three.rs");
}

#[test]
fn enhanced_format_handles_complex_dependency_chains() {
    let md = r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [x] complete

### Task 1.2: Second
- **Dependencies**: Task 1.1
- **Updated At**: 2026-01-01
- **Status**: [x] complete

### Task 1.3: Third
- **Dependencies**: Task 1.1, Task 1.2
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 3);
    assert_eq!(parsed.tasks[2].dependencies.len(), 2);

    let (ready, _blocked) = tasks::compute_ready_and_blocked(&parsed);
    assert_eq!(ready.len(), 1);
    assert_eq!(ready[0].id, "1.3");
}

#[test]
fn enhanced_format_validates_date_format_strictly() {
    let md = r#"### Task 1.1: Test
- **Updated At**: 2026-13-45
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("Invalid Updated At date"))
    );
}

#[test]
fn enhanced_format_validates_missing_required_fields() {
    let md = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Test without status
- **Dependencies**: None
- **Updated At**: 2026-01-01

### Task 1.2: Other task
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("Invalid or missing status"))
    );
}

#[test]
fn enhanced_format_handles_status_marker_mismatch() {
    let md = r#"### Task 1.1: Inconsistent marker
- **Updated At**: 2026-01-01
- **Status**: [x] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    // Should have a warning about marker mismatch
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("Only complete tasks should use [x]"))
    );
}

#[test]
fn enhanced_format_handles_checkpoints() {
    let md = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Regular task
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

## Checkpoints

### Checkpoint: Review
- **Type**: checkpoint
- **Dependencies**: All Wave 1 tasks
- **Done When**: User approves
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 2);
    // First task should have wave, checkpoint should not
    assert_eq!(parsed.tasks[0].wave, Some(1));
    assert_eq!(parsed.tasks[1].wave, None);
}

#[test]
fn wave_dependencies_handle_various_formats() {
    let md = r#"## Wave 1
- **Depends On**: None

## Wave 2
- **Depends On**: Wave 1

## Wave 3
- **Depends On**: 1, 2

## Wave 4
- **Depends On**: Wave 1, Wave 2, Wave 3

### Task 4.1: Test
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.waves.len(), 4);

    assert_eq!(parsed.waves[0].depends_on.len(), 0);
    assert_eq!(parsed.waves[1].depends_on, vec![1]);
    assert_eq!(parsed.waves[2].depends_on, vec![1, 2]);
    assert_eq!(parsed.waves[3].depends_on, vec![1, 2, 3]);
}

#[test]
fn wave_dependencies_detect_forward_references() {
    let md = r#"## Wave 1
- **Depends On**: Wave 2

### Task 1.1: Test
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("depends on missing Wave 2"))
    );
}

#[test]
fn tasks_path_checked_rejects_empty_change_id() {
    let root = std::path::Path::new("/tmp/.ito");
    assert!(tasks::tasks_path_checked(root, "").is_none());
}

#[test]
fn tasks_path_checked_rejects_very_long_change_ids() {
    let root = std::path::Path::new("/tmp/.ito");
    let long_id = "x".repeat(300);
    assert!(tasks::tasks_path_checked(root, &long_id).is_none());
}

#[test]
fn tasks_path_checked_accepts_valid_change_ids() {
    let root = std::path::Path::new("/tmp/.ito");
    assert!(tasks::tasks_path_checked(root, "001-01_test").is_some());
    assert!(tasks::tasks_path_checked(root, "simple").is_some());
    assert!(tasks::tasks_path_checked(root, "with-dashes-and_underscores").is_some());
}

#[test]
fn progress_info_calculates_remaining_correctly() {
    let md = r#"## Wave 1
- **Depends On**: None

### Task 1.1: Complete
- **Updated At**: 2026-01-01
- **Status**: [x] complete

### Task 1.2: Shelved
- **Updated At**: 2026-01-01
- **Status**: [-] shelved

### Task 1.3: In Progress
- **Updated At**: 2026-01-01
- **Status**: [ ] in-progress

### Task 1.4: Pending
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.progress.total, 4);
    assert_eq!(parsed.progress.complete, 1);
    assert_eq!(parsed.progress.shelved, 1);
    assert_eq!(parsed.progress.in_progress, 1);
    assert_eq!(parsed.progress.pending, 1);
    // Remaining = total - (complete + shelved)
    assert_eq!(parsed.progress.remaining, 2);
}

#[test]
fn enhanced_format_handles_uppercase_x_in_complete_marker() {
    let md = r#"### Task 1.1: Test
- **Updated At**: 2026-01-01
- **Status**: [X] complete
"#;

    let parsed = tasks::parse_tasks_tracking_file(md);
    assert_eq!(parsed.tasks.len(), 1);
    assert_eq!(parsed.tasks[0].status, tasks::TaskStatus::Complete);
}

#[test]
fn checkbox_format_progress_info_counts_correctly() {
    let md = "- [x] done\n- [ ] todo\n- [~] doing\n- [x] also done\n";
    let parsed = tasks::parse_tasks_tracking_file(md);

    assert_eq!(parsed.progress.total, 4);
    assert_eq!(parsed.progress.complete, 2);
    assert_eq!(parsed.progress.in_progress, 1);
    assert_eq!(parsed.progress.pending, 1);
    assert_eq!(parsed.progress.remaining, 2);
}
