use ito_domain::changes::{ChangeRepository, ChangeTargetResolution, ResolveTargetOptions};
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;
use ito_test_support::mock_repos::{
    MockChangeRepository, MockModuleRepository, MockTaskRepository, make_change_with_progress,
    make_module, make_tasks_result,
};

#[test]
fn mock_repos_basic_roundtrip() {
    let change_a = make_change_with_progress("001-01_a", Some("001"), 2, 1);
    let change_b = make_change_with_progress("001-02_b", Some("001"), 1, 1);

    let repo = MockChangeRepository::new()
        .with_change(change_a)
        .with_change(change_b);

    assert!(repo.exists("001-01_a"));

    let targets = repo.suggest_targets("001-0", 10);
    assert_eq!(targets.len(), 2);

    let resolution = repo.resolve_target_with_options("001-01", ResolveTargetOptions::default());
    match resolution {
        ChangeTargetResolution::Unique(_) => {}
        other => panic!("expected unique resolution, got: {other:?}"),
    }

    let list = repo.list().unwrap();
    assert_eq!(list.len(), 2);

    let by_module = repo.list_by_module("001").unwrap();
    assert_eq!(by_module.len(), 2);

    let complete = repo.list_complete().unwrap();
    assert_eq!(complete.len(), 1);

    let incomplete = repo.list_incomplete().unwrap();
    assert_eq!(incomplete.len(), 1);

    let summary = repo.get_summary("001-01_a").unwrap();
    assert_eq!(summary.id, "001-01_a");
}

#[test]
fn mock_task_repo_returns_configured_tasks() {
    let tasks = make_tasks_result(3, 1);
    let repo = MockTaskRepository::new().with_tasks("001-01_a", tasks);

    let loaded = repo.load_tasks("001-01_a").unwrap();
    assert_eq!(loaded.progress.total, 3);
    assert_eq!(loaded.progress.complete, 1);

    let missing = repo.load_tasks("missing").unwrap();
    assert_eq!(missing.progress.total, 0);
}

#[test]
fn mock_module_repo_resolves_by_id_or_name() {
    let module = make_module("001", "alpha");
    let repo = MockModuleRepository::new().with_module(module);

    assert!(repo.exists("001"));

    let by_id = repo.get("001").unwrap();
    assert_eq!(by_id.name, "alpha");

    let by_name = repo.get("alpha").unwrap();
    assert_eq!(by_name.id, "001");

    let list = repo.list().unwrap();
    assert_eq!(list.len(), 1);
}
