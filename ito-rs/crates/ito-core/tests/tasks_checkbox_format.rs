use ito_core::tasks::{TaskStatus, complete_task, get_next_task, shelve_task, start_task};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

#[test]
fn checkbox_tasks_enforce_single_in_progress_and_next_task_logic() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "## 1. Implementation\n- [ ] First\n- [ ] Second\n",
    );

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    let next = next.expect("should have a next task");
    assert_eq!(next.id, "1");

    let first = start_task(&ito, change_id, "1").expect("start 1");
    assert_eq!(first.status, TaskStatus::InProgress);

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    let next = next.expect("should have a current task");
    assert_eq!(next.id, "1");
    assert_eq!(next.status, TaskStatus::InProgress);

    let err = start_task(&ito, change_id, "2").expect_err("should not start 2");
    assert!(err.to_string().contains("already in-progress"));

    let done = complete_task(&ito, change_id, "1", None).expect("complete 1");
    assert_eq!(done.status, TaskStatus::Complete);

    let second = start_task(&ito, change_id, "2").expect("start 2");
    assert_eq!(second.status, TaskStatus::InProgress);
}

#[test]
fn checkbox_tasks_do_not_support_shelving() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(&tasks_path, "- [ ] First\n");

    let err = shelve_task(&ito, change_id, "1", None).expect_err("shelve should fail");
    assert!(err.to_string().contains("does not support shelving"));
}
