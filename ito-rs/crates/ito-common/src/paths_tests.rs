use super::*;

#[test]
fn default_ito_root_is_dot_ito() {
    let root = PathBuf::from("/repo");
    assert_eq!(default_ito_root(&root), PathBuf::from("/repo/.ito"));
}

#[test]
fn builders_join_expected_paths() {
    let ito = PathBuf::from("/repo/.ito");

    assert_eq!(changes_dir(&ito), PathBuf::from("/repo/.ito/changes"));
    assert_eq!(
        change_dir(&ito, "001-01_test"),
        PathBuf::from("/repo/.ito/changes/001-01_test")
    );
    assert_eq!(
        change_meta_path(&ito, "001-01_test"),
        PathBuf::from("/repo/.ito/changes/001-01_test/.ito.yaml")
    );
    assert_eq!(
        change_specs_dir(&ito, "001-01_test"),
        PathBuf::from("/repo/.ito/changes/001-01_test/specs")
    );
    assert_eq!(
        changes_archive_dir(&ito),
        PathBuf::from("/repo/.ito/changes/archive")
    );
    assert_eq!(
        archive_changes_dir(&ito),
        PathBuf::from("/repo/.ito/archive/changes")
    );
    assert_eq!(modules_dir(&ito), PathBuf::from("/repo/.ito/modules"));
    assert_eq!(specs_dir(&ito), PathBuf::from("/repo/.ito/specs"));
    assert_eq!(
        spec_markdown_path(&ito, "cli-tasks"),
        PathBuf::from("/repo/.ito/specs/cli-tasks/spec.md")
    );
}
