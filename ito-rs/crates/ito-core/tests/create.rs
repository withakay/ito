use ito_core::create::{CreateError, create_change, create_module};

fn write(path: impl AsRef<std::path::Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent directories");
    }
    std::fs::write(path, contents).expect("write fixture file");
}

fn create_module_fixture(ito: &std::path::Path, module_id: &str, module_name: &str) {
    let module_dir = ito
        .join("modules")
        .join(format!("{module_id}_{module_name}"));
    write(
        module_dir.join("module.md"),
        &format!(
            "# {module_name}\n\n## Purpose\nfixture\n\n## Scope\n- fixture\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n"
        ),
    );
}

#[test]
fn create_module_creates_directory_and_module_md() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let r = create_module(
        &ito,
        "demo",
        vec!["build".to_string(), "ship".to_string()],
        vec!["platform".to_string()],
        None,
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

    let first = create_module(&ito, "demo", vec![], vec![], None).expect("first create");
    assert!(first.created);

    let second = create_module(&ito, "demo", vec![], vec![], None).expect("second create");
    assert!(!second.created);
    assert_eq!(second.module_id, first.module_id);
    assert_eq!(second.folder_name, first.folder_name);
    assert_eq!(second.module_md, first.module_md);
}

#[test]
fn create_module_writes_description_to_purpose_section() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    let r = create_module(
        &ito,
        "described-module",
        vec!["*".to_string()],
        vec![],
        Some("This module handles described behavior."),
    )
    .expect("create_module should succeed");

    let md = std::fs::read_to_string(&r.module_md).expect("read module.md");
    assert!(
        md.contains("This module handles described behavior."),
        "module.md should include provided description"
    );
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

#[test]
fn create_change_writes_allocation_modules_in_ascending_id_order() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");

    for module_id in ["008", "004", "006", "002", "007", "003", "005", "001"] {
        create_module_fixture(&ito, module_id, &format!("module-{module_id}"));
        let name = format!("change-{module_id}");
        create_change(&ito, &name, "spec-driven", Some(module_id), None)
            .expect("create change in module");
    }

    let state = std::fs::read_to_string(ito.join("workflows/.state/change-allocations.json"))
        .expect("read allocation state");
    let value: serde_json::Value = serde_json::from_str(&state).expect("parse allocation json");
    let modules = value["modules"].as_object().expect("modules object");
    let observed: Vec<String> = modules.keys().cloned().collect();

    let mut expected = observed.clone();
    expected.sort();
    assert_eq!(
        observed, expected,
        "allocation module keys should be sorted"
    );
}

#[test]
fn create_change_rewrites_module_changes_in_ascending_change_id_order() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let ito = td.path().join(".ito");
    write(
        ito.join("modules/001_demo/module.md"),
        "# Demo\n\n## Purpose\nfixture\n\n## Scope\n- fixture\n\n## Changes\n- [ ] 001-03_third\n- [ ] 001-01_first\n",
    );

    let created =
        create_change(&ito, "second", "spec-driven", Some("001"), None).expect("create change");
    assert_eq!(created.change_id, "001-04_second");

    let module_md =
        std::fs::read_to_string(ito.join("modules/001_demo/module.md")).expect("read module.md");
    let idx_01 = module_md
        .find("001-01_first")
        .expect("module list should contain first");
    let idx_03 = module_md
        .find("001-03_third")
        .expect("module list should contain third");
    let idx_04 = module_md
        .find("001-04_second")
        .expect("module list should contain created change");
    assert!(
        idx_01 < idx_03 && idx_03 < idx_04,
        "module change list should be sorted ascending by change ID"
    );
}
