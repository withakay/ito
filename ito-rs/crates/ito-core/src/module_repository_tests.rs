use std::fs;
use std::path::Path;

use tempfile::TempDir;

use super::{FsModuleRepository, ModuleRepository};

fn setup_test_ito(tmp: &TempDir) -> std::path::PathBuf {
    let ito_path = tmp.path().join(".ito");
    fs::create_dir_all(ito_path.join("modules")).unwrap();
    fs::create_dir_all(ito_path.join("changes")).unwrap();
    ito_path
}

fn create_module(ito_path: &Path, id: &str, name: &str) {
    let module_dir = ito_path.join("modules").join(format!("{}_{}", id, name));
    fs::create_dir_all(&module_dir).unwrap();
}

fn create_change(ito_path: &Path, id: &str) {
    let change_dir = ito_path.join("changes").join(id);
    fs::create_dir_all(&change_dir).unwrap();
}

#[test]
fn test_exists() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_module(&ito_path, "005", "dev-tooling");

    let repo = ModuleRepository::new(&ito_path);
    assert!(repo.exists("005"));
    assert!(!repo.exists("999"));
}

#[test]
fn test_get() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_module(&ito_path, "005", "dev-tooling");

    let repo = ModuleRepository::new(&ito_path);
    let module = repo.get("005").unwrap();

    assert_eq!(module.id, "005");
    assert_eq!(module.name, "dev-tooling");
}

#[test]
fn test_get_not_found() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);

    let repo = ModuleRepository::new(&ito_path);
    let result = repo.get("999");
    assert!(result.is_err());
}

#[test]
fn test_list() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_module(&ito_path, "005", "dev-tooling");
    create_module(&ito_path, "003", "qa-testing");
    create_module(&ito_path, "001", "workflow");

    let repo = ModuleRepository::new(&ito_path);
    let modules = repo.list().unwrap();

    assert_eq!(modules.len(), 3);
    assert_eq!(modules[0].id, "001");
    assert_eq!(modules[1].id, "003");
    assert_eq!(modules[2].id, "005");
}

#[test]
fn test_list_with_change_counts() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_module(&ito_path, "005", "dev-tooling");
    create_module(&ito_path, "003", "qa-testing");

    create_change(&ito_path, "005-01_first");
    create_change(&ito_path, "005-02_second");
    create_change(&ito_path, "003-01_test");

    let repo = ModuleRepository::new(&ito_path);
    let modules = repo.list().unwrap();

    let module_005 = modules.iter().find(|m| m.id == "005").unwrap();
    let module_003 = modules.iter().find(|m| m.id == "003").unwrap();

    assert_eq!(module_005.change_count, 2);
    assert_eq!(module_003.change_count, 1);
}

#[test]
fn test_get_uses_full_name_input() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);
    create_module(&ito_path, "005", "dev-tooling");

    let repo = FsModuleRepository::new(&ito_path);
    let module = repo.get("005_dev-tooling").unwrap();
    assert_eq!(module.id, "005");
    assert_eq!(module.name, "dev-tooling");
}

// ── Task 2.7: Regression tests for sub-module support ─────────────────

/// Set up a sub-module directory under a parent module.
fn create_sub_module(
    ito_path: &Path,
    parent_id: &str,
    parent_name: &str,
    sub_num: &str,
    sub_name: &str,
) {
    let module_dir = ito_path
        .join("modules")
        .join(format!("{parent_id}_{parent_name}"));
    let sub_dir = module_dir.join("sub").join(format!("{sub_num}_{sub_name}"));
    fs::create_dir_all(&sub_dir).unwrap();
    fs::write(
        sub_dir.join("module.md"),
        format!("# {sub_name}\n\n## Purpose\n{sub_name} sub-module\n"),
    )
    .unwrap();
}

#[test]
fn regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes() {
    // Regression test: a parent module can have direct changes (024-07_*)
    // while a sub-module (024.01) owns its own changes (024.01-01_*).
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);

    // Set up module 024 with sub-module 01_auth.
    create_module(&ito_path, "024", "ito-backend");
    create_sub_module(&ito_path, "024", "ito-backend", "01", "auth");

    // Direct change on the parent module.
    create_change(&ito_path, "024-07_health-check");
    // Sub-module change.
    create_change(&ito_path, "024.01-01_add-jwt");

    let repo = ModuleRepository::new(&ito_path);

    // --- Module listing ---
    let modules = repo.list().unwrap();
    assert_eq!(modules.len(), 1);
    let m = &modules[0];
    assert_eq!(m.id, "024");
    // The parent module's change_count should include only the direct change
    // (024-07_health-check). The sub-module change (024.01-01_add-jwt) is
    // attributed to the sub-module, not the parent.
    assert_eq!(
        m.change_count, 1,
        "parent module should count only direct changes"
    );
    // Sub-module summary should be populated.
    assert_eq!(m.sub_modules.len(), 1);
    assert_eq!(m.sub_modules[0].id, "024.01");
    assert_eq!(m.sub_modules[0].name, "auth");
    assert_eq!(
        m.sub_modules[0].change_count, 1,
        "sub-module should count its own change"
    );

    // --- Module get ---
    let module = repo.get("024").unwrap();
    assert_eq!(module.id, "024");
    assert_eq!(module.sub_modules.len(), 1);
    assert_eq!(module.sub_modules[0].id, "024.01");
    assert_eq!(module.sub_modules[0].parent_module_id, "024");
    assert_eq!(module.sub_modules[0].sub_id, "01");
    assert_eq!(module.sub_modules[0].name, "auth");
    assert_eq!(module.sub_modules[0].change_count, 1);

    // --- list_sub_modules ---
    use ito_domain::modules::ModuleRepository as DomainModuleRepository;
    let sub_summaries = DomainModuleRepository::list_sub_modules(&repo, "024").unwrap();
    assert_eq!(sub_summaries.len(), 1);
    assert_eq!(sub_summaries[0].id, "024.01");

    // --- get_sub_module ---
    let sub = DomainModuleRepository::get_sub_module(&repo, "024.01").unwrap();
    assert_eq!(sub.id, "024.01");
    assert_eq!(sub.parent_module_id, "024");
    assert_eq!(sub.sub_id, "01");
    assert_eq!(sub.name, "auth");
    assert_eq!(sub.change_count, 1);
}

#[test]
fn regression_change_repository_populates_sub_module_id() {
    use crate::change_repository::FsChangeRepository;

    let tmp = TempDir::new().unwrap();
    let ito_path = setup_test_ito(&tmp);

    // Set up module 024 with sub-module 01_auth.
    create_module(&ito_path, "024", "ito-backend");
    create_sub_module(&ito_path, "024", "ito-backend", "01", "auth");

    // Direct change on the parent module.
    create_change(&ito_path, "024-07_health-check");
    // Sub-module change.
    create_change(&ito_path, "024.01-01_add-jwt");

    let change_repo = FsChangeRepository::new(&ito_path);
    let summaries = change_repo.list().unwrap();

    let direct = summaries
        .iter()
        .find(|s| s.id == "024-07_health-check")
        .unwrap();
    assert_eq!(direct.module_id.as_deref(), Some("024"));
    assert_eq!(
        direct.sub_module_id, None,
        "direct change should have no sub_module_id"
    );

    let sub_change = summaries
        .iter()
        .find(|s| s.id == "024.01-01_add-jwt")
        .unwrap();
    assert_eq!(sub_change.module_id.as_deref(), Some("024"));
    assert_eq!(
        sub_change.sub_module_id.as_deref(),
        Some("024.01"),
        "sub-module change should have sub_module_id set"
    );
}
