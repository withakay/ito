use super::*;

#[test]
fn open_in_memory_creates_schema() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    assert!(!store.project_exists("org", "repo"));
}

#[test]
fn ensure_project_creates_row() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    assert!(store.project_exists("acme", "widgets"));
}

#[test]
fn ensure_project_is_idempotent() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    assert!(store.project_exists("acme", "widgets"));
}

#[test]
fn upsert_and_list_changes() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "001-01_my-change",
            module_id: Some("001"),
            sub_module_id: None,
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done\n- [ ] 1.2 Pending"),
            specs: &[("auth", "## ADDED\n### Requirement: Auth")],
        })
        .unwrap();

    let change_repo = store.change_repository("org", "repo").unwrap();
    let changes = change_repo.list().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].id, "001-01_my-change");
    assert_eq!(changes[0].module_id, Some("001".to_string()));
    assert!(changes[0].has_proposal);
    assert!(!changes[0].has_design);
    assert!(changes[0].has_specs);
}

#[test]
fn get_change_returns_full_data() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "002-01_another",
            module_id: None,
            sub_module_id: None,
            proposal: Some("# My Proposal"),
            design: Some("# Design"),
            tasks_md: None,
            specs: &[("config", "## MODIFIED")],
        })
        .unwrap();

    let change_repo = store.change_repository("org", "repo").unwrap();
    let change = change_repo.get("002-01_another").unwrap();
    assert_eq!(change.id, "002-01_another");
    assert_eq!(change.proposal, Some("# My Proposal".to_string()));
    assert_eq!(change.design, Some("# Design".to_string()));
    assert_eq!(change.specs.len(), 1);
    assert_eq!(change.specs[0].name, "config");
}

#[test]
fn get_missing_change_returns_not_found() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    let change_repo = store.change_repository("org", "repo").unwrap();
    let err = change_repo.get("nonexistent").unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn upsert_and_list_modules() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    store
        .upsert_module("org", "repo", "001", "Backend", Some("Backend module"))
        .unwrap();

    let module_repo = store.module_repository("org", "repo").unwrap();
    let modules = module_repo.list().unwrap();
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].id, "001");
    assert_eq!(modules[0].name, "Backend");
}

#[test]
fn get_module_by_id() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    store
        .upsert_module("org", "repo", "001", "Backend", Some("Desc"))
        .unwrap();

    let module_repo = store.module_repository("org", "repo").unwrap();
    let module = module_repo.get("001").unwrap();
    assert_eq!(module.name, "Backend");
    assert_eq!(module.description, Some("Desc".to_string()));
}

#[test]
fn task_repository_loads_tasks() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "org",
            repo: "repo",
            change_id: "001-01_change",
            module_id: None,
            sub_module_id: None,
            proposal: None,
            design: None,
            tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done\n- [ ] 1.2 Pending"),
            specs: &[],
        })
        .unwrap();

    let task_repo = store.task_repository("org", "repo").unwrap();
    let result = task_repo.load_tasks("001-01_change").unwrap();
    assert!(result.progress.total > 0);
}

#[test]
fn task_repository_missing_change_returns_empty() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org", "repo").unwrap();
    let task_repo = store.task_repository("org", "repo").unwrap();
    let result = task_repo.load_tasks("nonexistent").unwrap();
    assert_eq!(result.progress.total, 0);
}

#[test]
fn two_projects_are_isolated() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("org1", "repo1").unwrap();
    store.ensure_project("org2", "repo2").unwrap();

    store
        .upsert_change(&UpsertChangeParams {
            org: "org1",
            repo: "repo1",
            change_id: "change-a",
            module_id: None,
            sub_module_id: None,
            proposal: None,
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "org2",
            repo: "repo2",
            change_id: "change-b",
            module_id: None,
            sub_module_id: None,
            proposal: None,
            design: None,
            tasks_md: None,
            specs: &[],
        })
        .unwrap();

    let repo1 = store.change_repository("org1", "repo1").unwrap();
    let repo2 = store.change_repository("org2", "repo2").unwrap();

    let changes1 = repo1.list().unwrap();
    let changes2 = repo2.list().unwrap();

    assert_eq!(changes1.len(), 1);
    assert_eq!(changes1[0].id, "change-a");
    assert_eq!(changes2.len(), 1);
    assert_eq!(changes2[0].id, "change-b");
}

#[test]
fn store_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SqliteBackendProjectStore>();
}

#[test]
fn on_disk_database_persists() {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("test.db");

    // Create and populate
    {
        let store = SqliteBackendProjectStore::open(&db_path).unwrap();
        store.ensure_project("org", "repo").unwrap();
        store
            .upsert_change(&UpsertChangeParams {
                org: "org",
                repo: "repo",
                change_id: "change-1",
                module_id: None,
                sub_module_id: None,
                proposal: Some("# P"),
                design: None,
                tasks_md: None,
                specs: &[],
            })
            .unwrap();
    }

    // Re-open and verify
    {
        let store = SqliteBackendProjectStore::open(&db_path).unwrap();
        assert!(store.project_exists("org", "repo"));
        let repo = store.change_repository("org", "repo").unwrap();
        let changes = repo.list().unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].id, "change-1");
    }
}

#[test]
fn push_artifact_bundle_rolls_back_partial_writes_on_failure() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-01_atomic-push",
            module_id: Some("025"),
            sub_module_id: None,
            proposal: Some("# Original Proposal"),
            design: Some("# Original Design"),
            tasks_md: Some("## 1. Tasks\n- [ ] 1.1 Keep original"),
            specs: &[("spec-one", "## ADDED Original")],
        })
        .unwrap();

    let mut bundle = store
        .pull_artifact_bundle("acme", "widgets", "025-01_atomic-push")
        .unwrap();
    bundle.proposal = Some("# Updated Proposal".to_string());
    bundle.specs = vec![
        ("duplicate".to_string(), "## ADDED First".to_string()),
        ("duplicate".to_string(), "## ADDED Second".to_string()),
    ];

    let err = store
        .push_artifact_bundle("acme", "widgets", "025-01_atomic-push", &bundle)
        .unwrap_err();
    assert!(matches!(
        err,
        ito_domain::backend::BackendError::Other(message)
            if message.contains("UNIQUE constraint failed")
    ));

    let current = store
        .pull_artifact_bundle("acme", "widgets", "025-01_atomic-push")
        .unwrap();
    assert_eq!(current.proposal.as_deref(), Some("# Original Proposal"));
    assert_eq!(current.design.as_deref(), Some("# Original Design"));
    assert_eq!(
        current.tasks.as_deref(),
        Some("## 1. Tasks\n- [ ] 1.1 Keep original")
    );
    assert_eq!(
        current.specs,
        vec![("spec-one".to_string(), "## ADDED Original".to_string())]
    );
}

#[test]
fn archive_change_rolls_back_when_spec_promotion_fails() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-02_atomic-archive",
            module_id: Some("025"),
            sub_module_id: None,
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: Some("## 1. Tasks\n- [x] 1.1 Done"),
            specs: &[(
                "spec-one",
                "## ADDED Requirements\n\n### Requirement: Archive me\nArchive behavior.\n",
            )],
        })
        .unwrap();

    {
        let conn = store.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TRIGGER fail_promoted_spec_insert
                 BEFORE INSERT ON promoted_specs
                 BEGIN
                     SELECT RAISE(ABORT, 'promoted spec insert failed');
                 END;",
        )
        .unwrap();
    }

    let err = store
        .archive_change("acme", "widgets", "025-02_atomic-archive")
        .unwrap_err();
    assert!(matches!(
        err,
        ito_domain::backend::BackendError::Other(message)
            if message.contains("promoted spec insert failed")
    ));

    let change_repo = store.change_repository("acme", "widgets").unwrap();
    assert!(
        change_repo.exists_with_filter("025-02_atomic-archive", ChangeLifecycleFilter::Active,)
    );
    assert!(
        !change_repo.exists_with_filter("025-02_atomic-archive", ChangeLifecycleFilter::Archived,)
    );

    let spec_repo = store.spec_repository("acme", "widgets").unwrap();
    assert!(spec_repo.list().unwrap().is_empty());
}

#[test]
fn task_mutation_service_reports_poisoned_connection_without_panicking() {
    let store = SqliteBackendProjectStore::open_in_memory().unwrap();
    store.ensure_project("acme", "widgets").unwrap();
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-03_poisoned-lock",
            module_id: Some("025"),
            sub_module_id: None,
            proposal: None,
            design: None,
            tasks_md: Some("## 1. Tasks\n- [ ] 1.1 Pending"),
            specs: &[],
        })
        .unwrap();

    let service = SqliteTaskMutationService {
        conn: Arc::clone(&store.conn),
        org: "acme".to_string(),
        repo: "widgets".to_string(),
    };

    let poisoned_conn = Arc::clone(&store.conn);
    let result = std::thread::spawn(move || {
        let _guard = poisoned_conn.lock().unwrap();
        panic!("poison sqlite connection mutex");
    })
    .join();
    assert!(result.is_err());

    let init_err = service.init_tasks("025-03_poisoned-lock").unwrap_err();
    assert!(matches!(
        init_err,
        ito_domain::tasks::TaskMutationError::Other(message)
            if message.contains("locking sqlite connection")
    ));

    let mutate_err = service
        .start_task("025-03_poisoned-lock", "1.1")
        .unwrap_err();
    assert!(matches!(
        mutate_err,
        ito_domain::tasks::TaskMutationError::Other(message)
            if message.contains("locking sqlite connection")
    ));
}
