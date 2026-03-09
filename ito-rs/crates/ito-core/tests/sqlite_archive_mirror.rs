use ito_core::sqlite_project_store::{SqliteBackendProjectStore, UpsertChangeParams};
use ito_domain::backend::BackendProjectStore;
use ito_domain::changes::ChangeLifecycleFilter;

#[test]
fn sqlite_archive_promotes_specs_and_marks_change_archived() {
    let store = SqliteBackendProjectStore::open_in_memory().expect("sqlite store");
    store
        .ensure_project("acme", "widgets")
        .expect("project row");
    store
        .upsert_change(&UpsertChangeParams {
            org: "acme",
            repo: "widgets",
            change_id: "025-05_archive-me",
            module_id: Some("025"),
            proposal: Some("# Proposal"),
            design: None,
            tasks_md: Some("- [x] done\n"),
            specs: &[("spec-one", "## ADDED Requirements\n")],
        })
        .expect("seed change");

    let result = store
        .archive_change("acme", "widgets", "025-05_archive-me")
        .expect("archive change");
    assert_eq!(result.change_id, "025-05_archive-me");

    let spec_repo = store.spec_repository("acme", "widgets").expect("spec repo");
    let spec = spec_repo.get("spec-one").expect("promoted spec");
    assert!(spec.markdown.contains("## ADDED Requirements"));

    let change_repo = store
        .change_repository("acme", "widgets")
        .expect("change repo");
    assert!(
        change_repo
            .list_with_filter(ChangeLifecycleFilter::Archived)
            .expect("archived list")
            .iter()
            .any(|change| change.id == "025-05_archive-me")
    );
    assert!(!change_repo.exists_with_filter("025-05_archive-me", ChangeLifecycleFilter::Active));
}
