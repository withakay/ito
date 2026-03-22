use ito_core::create::{CreateError, create_change, create_change_in_sub_module, create_module};

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

// ── Task 3.1: Allocation state sort order ────────────────────────────────────

#[test]
fn allocation_state_sub_module_keys_sort_after_parent() {
    // BTreeMap sorts lexicographically. '.' (0x2E) < '0' (0x30), so "024"
    // sorts before "024.01" which sorts before "024.02".
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    // Create parent module 024.
    create_module_fixture(&ito, "024", "backend");
    // Create sub-module 024/sub/01_auth.
    let sub_dir = ito.join("modules/024_backend/sub/01_auth");
    std::fs::create_dir_all(&sub_dir).expect("create sub dir");
    write(
        sub_dir.join("module.md"),
        "# Auth\n\n## Purpose\nAuth sub-module\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );

    // Create a direct module change (namespace "024").
    create_change(&ito, "direct-change", "spec-driven", Some("024"), None)
        .expect("create direct change");

    // Create a sub-module change (namespace "024.01").
    create_change_in_sub_module(&ito, "sub-change", "spec-driven", "024.01", None)
        .expect("create sub-module change");

    let state = std::fs::read_to_string(ito.join("workflows/.state/change-allocations.json"))
        .expect("read allocation state");
    let value: serde_json::Value = serde_json::from_str(&state).expect("parse json");
    let modules = value["modules"].as_object().expect("modules object");
    let keys: Vec<String> = modules.keys().cloned().collect();

    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted, "allocation keys must be in ascending sort order");

    // Verify that "024" appears before "024.01".
    let pos_024 = keys.iter().position(|k| k == "024").expect("024 key");
    let pos_024_01 = keys.iter().position(|k| k == "024.01").expect("024.01 key");
    assert!(pos_024 < pos_024_01, "\"024\" must sort before \"024.01\"");
}

// ── Task 3.3 + 3.4: Sub-module change creation ───────────────────────────────

#[test]
fn create_change_in_sub_module_uses_composite_id_format() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    create_module_fixture(&ito, "024", "backend");
    let sub_dir = ito.join("modules/024_backend/sub/01_auth");
    std::fs::create_dir_all(&sub_dir).expect("create sub dir");
    write(
        sub_dir.join("module.md"),
        "# Auth\n\n## Purpose\nAuth sub-module\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );

    let r = create_change_in_sub_module(&ito, "add-jwt", "spec-driven", "024.01", None)
        .expect("create sub-module change");

    // Change id must use NNN.SS-NN_name canonical form.
    assert_eq!(r.change_id, "024.01-01_add-jwt");
    assert!(r.change_dir.exists());
    assert!(r.change_dir.join(".ito.yaml").exists());
}

#[test]
fn create_change_in_sub_module_allocates_independent_sequence() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    create_module_fixture(&ito, "024", "backend");
    let sub_dir = ito.join("modules/024_backend/sub/01_auth");
    std::fs::create_dir_all(&sub_dir).expect("create sub dir");
    write(
        sub_dir.join("module.md"),
        "# Auth\n\n## Purpose\nAuth\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );

    // Create two direct module changes (namespace "024").
    create_change(&ito, "direct-one", "spec-driven", Some("024"), None).expect("direct 1");
    create_change(&ito, "direct-two", "spec-driven", Some("024"), None).expect("direct 2");

    // Sub-module sequence starts at 01, independent of the parent.
    let r = create_change_in_sub_module(&ito, "sub-one", "spec-driven", "024.01", None)
        .expect("sub-module change");
    assert_eq!(r.change_id, "024.01-01_sub-one");

    let r2 = create_change_in_sub_module(&ito, "sub-two", "spec-driven", "024.01", None)
        .expect("sub-module change 2");
    assert_eq!(r2.change_id, "024.01-02_sub-two");
}

// ── Task 3.5: Checklist targets sub-module's module.md ───────────────────────

#[test]
fn create_change_in_sub_module_writes_checklist_to_sub_module_md() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    create_module_fixture(&ito, "024", "backend");
    let sub_dir = ito.join("modules/024_backend/sub/01_auth");
    std::fs::create_dir_all(&sub_dir).expect("create sub dir");
    write(
        sub_dir.join("module.md"),
        "# Auth\n\n## Purpose\nAuth sub-module\n\n## Scope\n- *\n\n## Changes\n<!-- Changes will be listed here as they are created -->\n",
    );

    create_change_in_sub_module(&ito, "add-jwt", "spec-driven", "024.01", None)
        .expect("create sub-module change");

    // The sub-module's module.md should contain the new change entry.
    let sub_md = std::fs::read_to_string(sub_dir.join("module.md")).expect("read sub module.md");
    assert!(
        sub_md.contains("- [ ] 024.01-01_add-jwt"),
        "sub-module module.md should contain the new change entry; got:\n{sub_md}"
    );

    // The parent module's module.md should NOT contain the sub-module change.
    let parent_md =
        std::fs::read_to_string(ito.join("modules/024_backend/module.md")).expect("read parent");
    assert!(
        !parent_md.contains("024.01-01_add-jwt"),
        "parent module.md should not contain sub-module change; got:\n{parent_md}"
    );
}

// ── Task 3.6: Checklist ordering ─────────────────────────────────────────────

#[test]
fn create_change_in_sub_module_checklist_is_sorted_ascending() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    create_module_fixture(&ito, "024", "backend");
    let sub_dir = ito.join("modules/024_backend/sub/01_auth");
    std::fs::create_dir_all(&sub_dir).expect("create sub dir");
    // Pre-populate with an out-of-order checklist.
    write(
        sub_dir.join("module.md"),
        "# Auth\n\n## Purpose\nAuth\n\n## Scope\n- *\n\n## Changes\n- [ ] 024.01-03_third\n- [ ] 024.01-01_first\n",
    );

    let r = create_change_in_sub_module(&ito, "second", "spec-driven", "024.01", None)
        .expect("create change");
    // Allocation sees max=3 from existing entries, so next is 4.
    assert_eq!(r.change_id, "024.01-04_second");

    let sub_md = std::fs::read_to_string(sub_dir.join("module.md")).expect("read sub module.md");
    let idx_01 = sub_md.find("024.01-01_first").expect("first entry");
    let idx_03 = sub_md.find("024.01-03_third").expect("third entry");
    let idx_04 = sub_md.find("024.01-04_second").expect("fourth entry");
    assert!(
        idx_01 < idx_03 && idx_03 < idx_04,
        "sub-module checklist should be sorted ascending by change ID"
    );
}

// ── Error cases ───────────────────────────────────────────────────────────────

#[test]
fn create_change_in_sub_module_rejects_missing_parent_module() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    let err = create_change_in_sub_module(&ito, "my-change", "spec-driven", "099.01", None)
        .expect_err("should fail when parent module missing");
    assert!(
        matches!(err, CreateError::ModuleNotFound(_)),
        "expected ModuleNotFound, got {err:?}"
    );
}

#[test]
fn create_change_in_sub_module_rejects_missing_sub_module_dir() {
    let td = tempfile::tempdir().expect("tempdir");
    let ito = td.path().join(".ito");

    // Parent module exists but sub-module directory does not.
    create_module_fixture(&ito, "024", "backend");

    let err = create_change_in_sub_module(&ito, "my-change", "spec-driven", "024.01", None)
        .expect_err("should fail when sub-module dir missing");
    assert!(
        matches!(err, CreateError::SubModuleNotFound(_)),
        "expected SubModuleNotFound, got {err:?}"
    );
}
