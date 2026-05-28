use super::*;

#[test]
fn ito_path_resolves_correctly() {
    let store = FsBackendProjectStore::new("/data");
    let path = store.ito_path_for("withakay", "ito").unwrap();
    assert_eq!(path, PathBuf::from("/data/projects/withakay/ito/.ito"));
}

#[test]
fn ito_path_rejects_path_traversal() {
    let store = FsBackendProjectStore::new("/data");
    assert!(store.ito_path_for("..", "ito").is_err());
    assert!(store.ito_path_for("org", "..").is_err());
    assert!(store.ito_path_for(".", "repo").is_err());
    assert!(store.ito_path_for("org/evil", "repo").is_err());
    assert!(store.ito_path_for("org", "repo\\evil").is_err());
    assert!(store.ito_path_for("", "repo").is_err());
}

#[test]
fn project_exists_returns_false_for_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let store = FsBackendProjectStore::new(tmp.path());
    assert!(!store.project_exists("noorg", "norepo"));
}

#[test]
fn ensure_project_creates_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let store = FsBackendProjectStore::new(tmp.path());
    store.ensure_project("acme", "widgets").unwrap();
    assert!(store.project_exists("acme", "widgets"));
    assert!(store.ito_path_for("acme", "widgets").unwrap().is_dir());
}

#[test]
fn change_repository_returns_box_trait() {
    let tmp = tempfile::tempdir().unwrap();
    let store = FsBackendProjectStore::new(tmp.path());
    store.ensure_project("org", "repo").unwrap();
    let repo = store.change_repository("org", "repo").unwrap();
    // Should return an empty list for a fresh project
    let changes = repo.list().unwrap();
    assert!(changes.is_empty());
}

#[test]
fn module_repository_returns_box_trait() {
    let tmp = tempfile::tempdir().unwrap();
    let store = FsBackendProjectStore::new(tmp.path());
    store.ensure_project("org", "repo").unwrap();
    let repo = store.module_repository("org", "repo").unwrap();
    let modules = repo.list().unwrap();
    assert!(modules.is_empty());
}

#[test]
fn task_repository_returns_box_trait() {
    let tmp = tempfile::tempdir().unwrap();
    let store = FsBackendProjectStore::new(tmp.path());
    store.ensure_project("org", "repo").unwrap();
    let repo = store.task_repository("org", "repo").unwrap();
    // Loading tasks for a non-existent change should return empty
    let result = repo.load_tasks("nonexistent-change").unwrap();
    assert_eq!(result.progress.total, 0);
}

#[test]
fn store_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FsBackendProjectStore>();
}
