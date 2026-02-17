use ito_core::create::{CreateError, create_change, create_module};

#[test]
fn create_module_creates_directory_and_module_md() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let r = create_module(
        &ito,
        "demo",
        vec!["build".to_string(), "ship".to_string()],
        vec!["platform".to_string()],
    )
    .expect("create_module should succeed");

    assert!(r.created);
    assert_eq!(r.module_id, "001");
    assert!(r.module_dir.exists());
    assert!(r.module_md.exists());

    let md = std::fs::read_to_string(&r.module_md).expect("read module.md");
    assert!(md.contains("# Demo"));
    assert!(md.contains("## Scope"));
    assert!(md.contains("- build"));
    assert!(md.contains("- ship"));
    assert!(md.contains("## Depends On"));
    assert!(md.contains("- platform"));
}

#[test]
fn create_module_returns_existing_module_when_name_matches() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let first = create_module(&ito, "demo", vec![], vec![]).expect("first create");
    assert!(first.created);

    let second = create_module(&ito, "demo", vec![], vec![]).expect("second create");
    assert!(!second.created);
    assert_eq!(second.module_id, first.module_id);
    assert_eq!(second.folder_name, first.folder_name);
    assert_eq!(second.module_md, first.module_md);
}

#[test]
fn create_change_creates_change_dir_and_updates_module_md() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let r = create_change(
        &ito,
        "add-thing",
        "spec-driven",
        None,
        Some("Create a thing"),
    )
    .expect("create_change should succeed");

    assert_eq!(r.change_id, "000-01_add-thing");
    assert!(r.change_dir.exists());
    assert!(r.change_dir.join(".ito.yaml").exists());
    assert!(r.change_dir.join("README.md").exists());

    let meta = std::fs::read_to_string(r.change_dir.join(".ito.yaml")).expect("read .ito.yaml");
    assert!(meta.contains("schema: spec-driven"));

    let module_md = ito.join("modules/000_ungrouped/module.md");
    let module_contents = std::fs::read_to_string(&module_md).expect("read module.md");
    assert!(
        module_contents.contains("- [ ] 000-01_add-thing"),
        "module.md should include new change id"
    );
}

#[test]
fn create_change_allocates_next_number_from_existing_change_dirs() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    std::fs::create_dir_all(ito.join("changes").join("000-02_prev")).expect("create prev");

    let r = create_change(&ito, "next", "spec-driven", None, None).expect("create_change");
    assert_eq!(r.change_id, "000-03_next");
}

#[test]
fn create_change_rejects_uppercase_names() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let err = create_change(&ito, "BadName", "spec-driven", None, None)
        .expect_err("uppercase should be rejected");
    let CreateError::InvalidChangeName(msg) = err else {
        panic!("unexpected error: {err:?}");
    };
    assert!(
        msg.contains("lowercase"),
        "message should mention lowercase"
    );
}
