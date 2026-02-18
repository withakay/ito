//! Additional tests for ito-core tasks orchestration layer error paths and edge cases.

use ito_core::tasks::{
    add_task, complete_task, get_next_task, get_task_status, init_tasks, shelve_task, start_task,
    unshelve_task,
};
mod support;
use support::write;

#[test]
fn init_tasks_creates_file_when_missing() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";

    let (path, existed) = init_tasks(&ito, change_id).expect("init should succeed");

    assert!(!existed, "file should not have existed");
    assert!(path.exists(), "file should be created");

    let contents = std::fs::read_to_string(&path).expect("read should succeed");
    assert!(contents.contains("# Tasks for: 001-01_test"));
    assert!(contents.contains("## Wave 1"));
}

#[test]
fn init_tasks_does_not_overwrite_existing_file() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "existing content\n");

    let (path, existed) = init_tasks(&ito, change_id).expect("init should succeed");

    assert!(existed, "file should have existed");
    assert_eq!(path, tasks_path);

    let contents = std::fs::read_to_string(&path).expect("read should succeed");
    assert_eq!(contents, "existing content\n");
}

#[test]
fn init_tasks_rejects_invalid_change_id() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    let result = init_tasks(&ito, "../escape");
    assert!(result.is_err(), "should reject path traversal");

    let result = init_tasks(&ito, "");
    assert!(result.is_err(), "should reject empty id");
}

#[test]
fn get_task_status_returns_error_when_file_missing() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";

    let result = get_task_status(&ito, change_id);
    assert!(result.is_err(), "should error when file missing");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("No tasks.md found"));
}

#[test]
fn get_task_status_returns_diagnostics_for_malformed_file() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Missing status\n- **Dependencies**: None\n",
    );

    let status = get_task_status(&ito, change_id).expect("should parse");
    assert!(!status.diagnostics.is_empty());
}

#[test]
fn get_next_task_returns_none_when_all_complete() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [x] done\n- [x] also done\n");

    let next = get_next_task(&ito, change_id).expect("should succeed");
    assert!(next.is_none(), "all tasks complete, should return None");
}

#[test]
fn get_next_task_returns_current_in_progress_for_checkbox() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [x] done\n- [~] current\n- [ ] next\n");

    let next = get_next_task(&ito, change_id)
        .expect("should succeed")
        .expect("should have next task");

    assert_eq!(next.id, "2");
    assert_eq!(next.name, "current");
}

#[test]
fn get_next_task_returns_first_ready_for_enhanced() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [x] complete

### Task 1.2: Second
- **Dependencies**: Task 1.1
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#,
    );

    let next = get_next_task(&ito, change_id)
        .expect("should succeed")
        .expect("should have next task");

    assert_eq!(next.id, "1.2");
}

#[test]
fn start_task_validates_task_is_ready() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending

### Task 1.2: Blocked
- **Dependencies**: Task 1.1
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#,
    );

    let result = start_task(&ito, change_id, "1.2");
    assert!(result.is_err(), "should reject starting blocked task");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("blocked"));
}

#[test]
fn start_task_rejects_shelved_task() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: Shelved
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [-] shelved
"#,
    );

    let result = start_task(&ito, change_id, "1.1");
    assert!(result.is_err(), "should reject starting shelved task");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("shelved"));
}

#[test]
fn start_task_rejects_already_complete() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [x] already done\n");

    let result = start_task(&ito, change_id, "1");
    assert!(result.is_err(), "should reject starting complete task");
}

#[test]
fn complete_task_handles_checkbox_format() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [~] doing\n");

    let task = complete_task(&ito, change_id, "1", None).expect("should complete");
    assert_eq!(task.id, "1");

    let contents = std::fs::read_to_string(&tasks_path).expect("read should succeed");
    assert!(contents.contains("- [x] doing"));
}

#[test]
fn complete_task_handles_enhanced_format() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: Test
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [>] in-progress
"#,
    );

    let task = complete_task(&ito, change_id, "1.1", None).expect("should complete");
    assert_eq!(task.id, "1.1");

    let contents = std::fs::read_to_string(&tasks_path).expect("read should succeed");
    assert!(contents.contains("- **Status**: [x] complete"));
}

#[test]
fn shelve_task_rejects_checkbox_format() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [ ] task\n");

    let result = shelve_task(&ito, change_id, "1", None);
    assert!(result.is_err(), "should reject shelving checkbox task");
}

#[test]
fn shelve_task_rejects_complete_task() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: Done
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [x] complete
"#,
    );

    let result = shelve_task(&ito, change_id, "1.1", None);
    assert!(result.is_err(), "should reject shelving complete task");
}

#[test]
fn unshelve_task_rejects_not_shelved() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: Pending
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#,
    );

    let result = unshelve_task(&ito, change_id, "1.1");
    assert!(result.is_err(), "should reject unshelving non-shelved task");
}

#[test]
fn unshelve_task_transitions_to_pending() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: Shelved
- **Dependencies**: None
- **Updated At**: 2026-01-01
- **Status**: [-] shelved
"#,
    );

    let task = unshelve_task(&ito, change_id, "1.1").expect("should unshelve");
    assert_eq!(task.id, "1.1");

    let contents = std::fs::read_to_string(&tasks_path).expect("read should succeed");
    assert!(contents.contains("- **Status**: [ ] pending"));
}

#[test]
fn add_task_rejects_checkbox_format() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(&tasks_path, "- [ ] task\n");

    let result = add_task(&ito, change_id, "New task", None);
    assert!(result.is_err(), "should reject adding to checkbox format");
}

#[test]
fn add_task_assigns_next_id_in_wave() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#,
    );

    let task = add_task(&ito, change_id, "Second task", Some(1)).expect("should add");
    assert_eq!(task.id, "1.2");

    let contents = std::fs::read_to_string(&tasks_path).expect("read should succeed");
    assert!(contents.contains("### Task 1.2: Second task"));
}

#[test]
fn add_task_creates_wave_when_missing() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None

### Task 1.1: First
- **Updated At**: 2026-01-01
- **Status**: [ ] pending
"#,
    );

    let task = add_task(&ito, change_id, "New wave task", Some(2)).expect("should add");
    assert_eq!(task.id, "2.1");

    let contents = std::fs::read_to_string(&tasks_path).expect("read should succeed");
    assert!(contents.contains("## Wave 2"));
    assert!(contents.contains("### Task 2.1: New wave task"));
}

#[test]
fn add_task_defaults_to_wave_1() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        r#"## Wave 1
- **Depends On**: None
"#,
    );

    let task = add_task(&ito, change_id, "Task with default wave", None).expect("should add");
    assert_eq!(task.wave, Some(1));
}

#[test]
fn start_task_errors_with_parse_errors() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Broken\n- **Status**: [?] unknown\n",
    );

    let result = start_task(&ito, change_id, "1.1");
    assert!(result.is_err(), "should reject file with errors");
}

#[test]
fn complete_task_errors_with_parse_errors() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Broken\n- **Status**: [?] unknown\n",
    );

    let result = complete_task(&ito, change_id, "1.1", None);
    assert!(result.is_err(), "should reject file with errors");
}

#[test]
fn shelve_task_errors_with_parse_errors() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Broken\n- **Status**: [?] unknown\n",
    );

    let result = shelve_task(&ito, change_id, "1.1", None);
    assert!(result.is_err(), "should reject file with errors");
}

#[test]
fn unshelve_task_errors_with_parse_errors() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Broken\n- **Status**: [?] unknown\n",
    );

    let result = unshelve_task(&ito, change_id, "1.1");
    assert!(result.is_err(), "should reject file with errors");
}

#[test]
fn add_task_errors_with_parse_errors() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");
    let change_id = "001-01_test";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "### Task 1.1: Broken\n- **Status**: [?] unknown\n",
    );

    let result = add_task(&ito, change_id, "New task", Some(1));
    assert!(result.is_err(), "should reject file with errors");
}
