use super::resolve_project_root;
use std::path::Path;

#[test]
fn resolve_project_root_returns_parent_directory() {
    let root = resolve_project_root(Path::new("/tmp/project/.ito")).unwrap();
    assert_eq!(root, Path::new("/tmp/project"));
}

#[test]
fn resolve_project_root_rejects_parentless_paths() {
    let err = resolve_project_root(Path::new("/")).unwrap_err();
    assert!(
        err.to_string()
            .contains("Invalid Ito root without parent directory")
    );
}
