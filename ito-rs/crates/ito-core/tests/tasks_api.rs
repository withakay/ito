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
        "# Tasks for: {change_id}\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First\n- **Files**: `a.rs`\n- **Dependencies**: None\n- **Action**:\n  - Do the thing\n- **Verify**: `cargo test -p ito-core`\n- **Done When**: Done\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n\n### Task 1.2: Second\n- **Files**: `b.rs`\n- **Dependencies**: Task 1.1\n- **Action**:\n  - Do the second thing\n- **Verify**: `cargo test -p ito-core`\n- **Done When**: Done\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n"
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

    // Use a single-task file so shelving cannot create an invalid dependency
    // graph (pending task depending on a shelved task).
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First\n\n- **Files**: `a.rs`\n- **Dependencies**: None\n- **Action**:\n  - Do the thing\n- **Verify**: `cargo test -p ito-core`\n- **Done When**: Done\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n"
        ),
    );

    let shelved = shelve_task(&ito, change_id, "1.1", Some("no longer needed".to_string()))
        .expect("shelve should succeed");
    assert_eq!(shelved.status, TaskStatus::Shelved);

    let unshelved = unshelve_task(&ito, change_id, "1.1").expect("unshelve should succeed");
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

#[test]
fn init_tasks_returns_true_when_file_already_exists() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";

    let (_, existed1) = init_tasks(&ito, change_id).expect("first init should succeed");
    assert!(!existed1);

    let (_, existed2) = init_tasks(&ito, change_id).expect("second init should succeed");
    assert!(existed2);
}

#[test]
fn get_next_task_returns_none_when_all_tasks_complete() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [x] complete\n"
        ),
    );

    let next = get_next_task(&ito, change_id).expect("get_next_task");
    assert!(next.is_none());
}

#[test]
fn start_task_rejects_starting_shelved_task_directly() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [-] shelved\n"
        ),
    );

    let err = start_task(&ito, change_id, "1.1").expect_err("should fail for shelved task");
    assert!(err.to_string().contains("shelved"));
}

#[test]
fn complete_task_accepts_note_parameter() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [ ] in-progress\n"
        ),
    );

    let result = complete_task(&ito, change_id, "1.1", Some("Done early".to_string()))
        .expect("complete with note should succeed");
    assert_eq!(result.status, TaskStatus::Complete);
}

#[test]
fn shelve_task_accepts_reason_parameter() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [ ] pending\n"
        ),
    );

    let result = shelve_task(&ito, change_id, "1.1", Some("No longer needed".to_string()))
        .expect("shelve with reason should succeed");
    assert_eq!(result.status, TaskStatus::Shelved);
}

#[test]
fn shelve_task_rejects_shelving_complete_task() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [x] complete\n"
        ),
    );

    let err = shelve_task(&ito, change_id, "1.1", None).expect_err("should fail for complete task");
    assert!(err.to_string().contains("already complete"));
}

#[test]
fn add_task_creates_wave_if_not_exists() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let tasks_path = ito.join("changes").join(change_id).join("tasks.md");
    write(
        &tasks_path,
        &format!(
            "# Tasks for: {change_id}\n\n## Wave 1\n- **Depends On**: None\n\n### Task 1.1: First\n- **Dependencies**: None\n- **Updated At**: 2026-02-01\n- **Status**: [x] complete\n\n## Checkpoints\n"
        ),
    );

    let item = add_task(&ito, change_id, "New wave task", Some(2))
        .expect("add_task to new wave should succeed");
    assert_eq!(item.id, "2.1");
    assert_eq!(item.wave, Some(2));

    let contents = std::fs::read_to_string(&tasks_path).expect("read tasks.md");
    assert!(contents.contains("## Wave 2"));
    assert!(contents.contains("### Task 2.1: New wave task"));
}

#[test]
fn list_ready_tasks_across_changes_handles_empty_repo() {
    use ito_core::change_repository::FsChangeRepository;
    use ito_core::tasks::list_ready_tasks_across_changes;

    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).expect("create ito dir");

    let change_repo = FsChangeRepository::new(&ito);
    let ready = list_ready_tasks_across_changes(&change_repo, &ito)
        .expect("list should succeed on empty repo");

    assert!(ready.is_empty());
}
