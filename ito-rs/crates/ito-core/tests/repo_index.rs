use ito_core::repo_index::RepoIndex;

#[test]
fn repo_index_loads_and_excludes_archive_change_dir() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    std::fs::create_dir_all(ito.join("changes").join("001-01_demo")).unwrap();
    std::fs::create_dir_all(ito.join("changes").join("archive")).unwrap();
    std::fs::create_dir_all(ito.join("modules").join("001_demo")).unwrap();
    std::fs::create_dir_all(ito.join("specs").join("demo")).unwrap();

    let idx = RepoIndex::load(&ito).unwrap();
    assert!(idx.change_dir_names.contains(&"001-01_demo".to_string()));
    assert!(!idx.change_dir_names.contains(&"archive".to_string()));
    assert!(idx.module_dir_names.contains(&"001_demo".to_string()));
    assert!(idx.spec_dir_names.contains(&"demo".to_string()));
}
