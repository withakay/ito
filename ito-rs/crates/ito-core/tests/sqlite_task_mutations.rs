use ito_core::sqlite_project_store::{SqliteBackendProjectStore, UpsertChangeParams};
use ito_domain::backend::BackendProjectStore;
use ito_domain::tasks::TaskStatus;

#[test]
fn sqlite_task_mutation_service_initializes_missing_tasks() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store
        .ensure_project("acme", "widgets")
        .expect("project row");
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-02_demo",
            module_id: Some("025"),
            sub_module_id: None,
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("seed change");

    let service = store
        .task_mutation_service("acme", "widgets")
        .expect("task mutation service");
    let result = service.init_tasks("025-02_demo").expect("init tasks");

    assert_eq!(result.change_id, "025-02_demo");
    assert!(!result.existed, "tasks should not have existed before init");
    assert!(result.path.is_none(), "sqlite store returns no path");
    assert!(result.revision.is_none());

    let repo = store
        .task_repository("acme", "widgets")
        .expect("task repository");
    let parsed = repo.load_tasks("025-02_demo").expect("load tasks");
    // Template has 1 task + 1 checkpoint = 2 total items
    assert_eq!(
        parsed.progress.total, 2,
        "template should have 2 items (1 task + 1 checkpoint)"
    );
}

#[test]
fn sqlite_task_mutation_service_updates_existing_markdown() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store
        .ensure_project("acme", "widgets")
        .expect("project row");
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-02_demo",
            module_id: Some("025"),
            sub_module_id: None,
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: Some(
                "# Tasks for: 025-02_demo\n\n## Wave 1\n\n- **Depends On**: None\n\n### Task 1.1: First task\n- **Dependencies**: None\n- **Updated At**: 2026-03-01\n- **Status**: [ ] pending\n",
            ),
            specs: &[],
        })
        .expect("seed change");

    let service = store
        .task_mutation_service("acme", "widgets")
        .expect("task mutation service");
    let result = service
        .start_task("025-02_demo", "1.1")
        .expect("start task");

    assert_eq!(result.change_id, "025-02_demo");
    assert!(result.revision.is_none());
    assert_eq!(result.task.id, "1.1");
    assert_eq!(result.task.status, TaskStatus::InProgress);

    let markdown = service
        .load_tasks_markdown("025-02_demo")
        .expect("load tasks markdown")
        .expect("tasks markdown present");
    assert!(
        markdown.contains("- **Status**: [>] in-progress"),
        "{markdown}"
    );
}

#[test]
fn sqlite_task_mutation_service_returns_not_found_for_missing_tasks() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store
        .ensure_project("acme", "widgets")
        .expect("project row");
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-02_no-tasks",
            module_id: None,
            sub_module_id: None,
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .expect("seed change");

    let service = store
        .task_mutation_service("acme", "widgets")
        .expect("task mutation service");
    let err = service
        .start_task("025-02_no-tasks", "1.1")
        .expect_err("should fail when tasks not initialized");

    let msg = err.to_string();
    assert!(
        msg.contains("Run \"ito tasks init 025-02_no-tasks\" first"),
        "error should hint at init: {msg}"
    );
}
