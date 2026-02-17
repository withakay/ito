use ito_core::tasks::{TaskStatus, complete_task, get_next_task, shelve_task, start_task};
use std::path::Path;

/// Writes `contents` to `path`, creating any missing parent directories.
///
/// This function will panic if `path` has no parent directory or if directory
/// creation or file writing fail.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// let tmp = tempfile::tempdir().unwrap();
/// let p = tmp.path().join("a").join("file.txt");
/// write(&p, "hello");
/// assert_eq!(std::fs::read_to_string(p).unwrap(), "hello");
/// ```
fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

/// Verifies checkbox-format tasks enforce a single in-progress task and next-task selection when tasks have explicit IDs.
///
/// This test writes a tasks.md containing two unchecked tasks with explicit IDs ("1.1" and "1.2"), asserts that
/// get_next_task initially selects "1.1", starting "1.1" marks it InProgress and makes it the current next task,
/// attempting to start "1.2" fails while "1.1" is InProgress, completing "1.1" marks it Complete, and starting "1.2" afterwards succeeds.
///
/// # Examples
///
/// ```
/// // Creates tasks.md with "1.1 First" and "1.2 Second" and exercises get_next_task, start_task, and complete_task.
/// ```
#[test]
fn checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");

    write(
        &tasks_path,
        "## 1. Implementation\n- [ ] 1.1 First\n- [ ] 1.2 Second\n",
    );

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    let next = next.expect("should have a next task");
    assert_eq!(next.id, "1.1");

    let first = start_task(&ito, change_id, "1.1").expect("start 1.1");
    assert_eq!(first.status, TaskStatus::InProgress);

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    let next = next.expect("should have a current task");
    assert_eq!(next.id, "1.1");
    assert_eq!(next.status, TaskStatus::InProgress);

    let err = start_task(&ito, change_id, "1.2").expect_err("should not start 1.2");
    assert!(err.to_string().contains("already in-progress"));

    let done = complete_task(&ito, change_id, "1.1", None).expect("complete 1.1");
    assert_eq!(done.status, TaskStatus::Complete);

    let second = start_task(&ito, change_id, "1.2").expect("start 1.2");
    assert_eq!(second.status, TaskStatus::InProgress);
}

#[test]
fn checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback() {
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
    write(&tasks_path, "- [ ] 1.1 First\n");

    let err = shelve_task(&ito, change_id, "1.1", None).expect_err("shelve should fail");
    assert!(err.to_string().contains("does not support shelving"));
}