use super::*;
use std as test_lib;

use ito_common::fs::StdFs;

#[test]
fn list_changes_skips_archive_dir() {
    let td = tempfile::tempdir().unwrap();
    let ito_path = td.path().join(".ito");
    test_lib::fs::create_dir_all(ito_path.join("changes/archive")).unwrap();
    test_lib::fs::create_dir_all(ito_path.join("changes/001-01_test")).unwrap();

    let fs = StdFs;
    let changes = list_changes(&fs, &ito_path).unwrap();
    assert_eq!(changes, vec!["001-01_test".to_string()]);
}

#[test]
fn list_modules_only_returns_directories() {
    let td = tempfile::tempdir().unwrap();
    let ito_path = td.path().join(".ito");
    test_lib::fs::create_dir_all(ito_path.join("modules/001_project-setup")).unwrap();
    test_lib::fs::create_dir_all(ito_path.join("modules/.hidden")).unwrap();
    test_lib::fs::create_dir_all(ito_path.join("modules/not-a-module")).unwrap();
    test_lib::fs::write(ito_path.join("modules/file.txt"), "x").unwrap();

    let fs = StdFs;
    let modules = list_modules(&fs, &ito_path).unwrap();
    assert_eq!(
        modules,
        vec!["001_project-setup".to_string(), "not-a-module".to_string()]
    );
}

#[test]
fn list_module_ids_extracts_numeric_prefixes() {
    let td = tempfile::tempdir().unwrap();
    let ito_path = td.path().join(".ito");
    test_lib::fs::create_dir_all(ito_path.join("modules/001_project-setup")).unwrap();
    test_lib::fs::create_dir_all(ito_path.join("modules/002_tools")).unwrap();
    test_lib::fs::create_dir_all(ito_path.join("modules/not-a-module")).unwrap();

    let fs = StdFs;
    let ids = list_module_ids(&fs, &ito_path).unwrap();
    assert_eq!(
        ids.into_iter().collect::<Vec<_>>(),
        vec!["001".to_string(), "002".to_string()]
    );
}
