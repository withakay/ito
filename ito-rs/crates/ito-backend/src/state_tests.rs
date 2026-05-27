use super::*;
use ito_core::fs_project_store::FsBackendProjectStore;

#[test]
fn ito_path_for_resolves_to_expected_path() {
    let store = Arc::new(FsBackendProjectStore::new("/data"));
    let state = AppState::new(
        PathBuf::from("/data"),
        store,
        BackendAllowlistConfig::default(),
        BackendAuthConfig::default(),
    );
    let path = state.ito_path_for("withakay", "ito").unwrap();
    assert_eq!(path, PathBuf::from("/data/projects/withakay/ito/.ito"));
}

#[test]
fn ito_path_for_rejects_path_traversal() {
    let store = Arc::new(FsBackendProjectStore::new("/data"));
    let state = AppState::new(
        PathBuf::from("/data"),
        store,
        BackendAllowlistConfig::default(),
        BackendAuthConfig::default(),
    );
    assert!(state.ito_path_for("..", "ito").is_err());
    assert!(state.ito_path_for("org", "..").is_err());
    assert!(state.ito_path_for(".", "repo").is_err());
    assert!(state.ito_path_for("org/evil", "repo").is_err());
    assert!(state.ito_path_for("org", "repo\\evil").is_err());
    assert!(state.ito_path_for("", "repo").is_err());
}

#[test]
fn ensure_project_dir_creates_directories() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Arc::new(FsBackendProjectStore::new(tmp.path()));
    let state = AppState::new(
        tmp.path().to_path_buf(),
        store,
        BackendAllowlistConfig::default(),
        BackendAuthConfig::default(),
    );
    state.ensure_project_dir("acme", "repo1").unwrap();
    assert!(state.ito_path_for("acme", "repo1").unwrap().is_dir());
}
