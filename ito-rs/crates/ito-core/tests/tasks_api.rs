use ito_core::tasks::{
    TaskStatus, add_task, complete_task, get_next_task, init_tasks, shelve_task, start_task,
    unshelve_task,
};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

fn enhanced_tasks_fixture(change_id: &str) -> String {
    format!(
        "# Tasks for: {change_id}\n\n## Wave 1: Basics\n- **Depends On**: None\n\n### Task 1.1: First\n- **Files**: `a.rs`\n- **Dependencies**: None\n- **Action**:\n  - Do the thing\n- **Verify**: `cargo test -p ito-core`\n- **Done When**: Done\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n\n### Task 1.2: Second\n- **Files**: `b.rs`\n- **Dependencies**: Task 1.1\n- **Action**:\n  - Do the second thing\n- **Verify**: `cargo test -p ito-core`\n- **Done When**: Done\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n"
    )
}

#[test]
fn init_tasks_creates_file_when_missing() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";

    let (path, existed) = init_tasks(&ito, change_id).expect("init should succeed");
    assert!(!existed);
    assert!(path.exists());

    let contents = std::fs::read_to_string(path).expect("read tasks.md");
    assert!(contents.contains(change_id));
    assert!(contents.contains("# Tasks for:"));
}

#[test]
fn start_and_complete_task_enforced_by_dependencies_for_enhanced_format() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(&tasks_path, &enhanced_tasks_fixture(change_id));

    let err = start_task(&ito, change_id, "1.2").expect_err("1.2 should be blocked");
    let msg = err.to_string();
    assert!(msg.contains("blocked"));
    assert!(msg.contains("1.1"));

    let first = start_task(&ito, change_id, "1.1").expect("start 1.1");
    assert_eq!(first.status, TaskStatus::InProgress);

    let first = complete_task(&ito, change_id, "1.1", None).expect("complete 1.1");
    assert_eq!(first.status, TaskStatus::Complete);

    let second = start_task(&ito, change_id, "1.2").expect("start 1.2");
    assert_eq!(second.status, TaskStatus::InProgress);
}

#[test]
fn shelve_and_unshelve_task_round_trip_for_enhanced_format() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(&tasks_path, &enhanced_tasks_fixture(change_id));

    // Shelve a leaf task to avoid creating a relational error where a pending
    // task depends on a shelved task.
    let shelved = shelve_task(&ito, change_id, "1.2", Some("no longer needed".to_string()))
        .expect("shelve should succeed");
    assert_eq!(shelved.status, TaskStatus::Shelved);

    let unshelved = unshelve_task(&ito, change_id, "1.2").expect("unshelve should succeed");
    assert_eq!(unshelved.status, TaskStatus::Pending);
}

#[test]
fn get_next_task_returns_first_ready_task_for_enhanced_format() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(&tasks_path, &enhanced_tasks_fixture(change_id));

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    let next = next.expect("should have a next task");
    assert_eq!(next.id, "1.1");
}

#[test]
fn add_task_appends_new_task_with_next_id() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Files**: `a.rs`\n- **Dependencies**: None\n- **Action**:\n  - A\n- **Verify**: `true`\n- **Done When**: ok\n- **Updated At**: 2026-02-01\n- **Status**: [x] complete\n\n## Checkpoints\n"
        ),
    );

    let item = add_task(&ito, change_id, "Second", Some(1)).expect("add_task should succeed");
    assert_eq!(item.id, "1.2");
    assert_eq!(item.status, TaskStatus::Pending);

    let contents = std::fs::read_to_string(&tasks_path).expect("read tasks.md");
    assert!(contents.contains("### Task 1.2: Second"));
}
