//! Tests for sub-module creation (`create_sub_module`).

use super::*;
use std::path::Path;
use tempfile::TempDir;

fn setup_ito(tmp: &TempDir) -> std::path::PathBuf {
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("modules")).unwrap();
    std::fs::create_dir_all(ito_path.join("changes")).unwrap();
    ito_path
}

fn make_module(ito_path: &Path, id: &str, name: &str) {
    let dir = ito_path.join("modules").join(format!("{id}_{name}"));
    std::fs::create_dir_all(&dir).unwrap();
    let md = format!(
        "# {name}\n\n## Purpose\nTest module purpose text that is long enough.\n\n## Scope\n- *\n\n## Changes\n<!-- none -->\n"
    );
    std::fs::write(dir.join("module.md"), md).unwrap();
}

// ── Task 4.1 + 4.2: create_sub_module ─────────────────────────────────

#[test]
fn create_sub_module_creates_directory_and_module_md() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    let result = create_sub_module(&ito_path, "auth", "024", None).unwrap();

    assert_eq!(result.sub_module_id, "024.01");
    assert_eq!(result.sub_module_name, "auth");
    assert_eq!(result.parent_module_id, "024");
    assert!(result.sub_module_dir.exists());
    assert!(result.sub_module_dir.join("module.md").exists());
}

#[test]
fn create_sub_module_with_description_writes_purpose() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    let result = create_sub_module(&ito_path, "sync", "024", Some("Sync subsystem")).unwrap();

    let md = std::fs::read_to_string(result.sub_module_dir.join("module.md")).unwrap();
    assert!(
        md.contains("Sync subsystem"),
        "description should appear in module.md"
    );
}

#[test]
fn create_sub_module_allocates_sequential_numbers() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    let r1 = create_sub_module(&ito_path, "auth", "024", None).unwrap();
    let r2 = create_sub_module(&ito_path, "sync", "024", None).unwrap();

    assert_eq!(r1.sub_module_id, "024.01");
    assert_eq!(r2.sub_module_id, "024.02");
}

#[test]
fn create_sub_module_errors_on_unknown_parent_module() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);

    let err = create_sub_module(&ito_path, "auth", "999", None).unwrap_err();
    assert!(
        matches!(err, CreateError::ModuleNotFound(_)),
        "expected ModuleNotFound, got {err:?}"
    );
}

#[test]
fn create_sub_module_errors_on_duplicate_name() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    create_sub_module(&ito_path, "auth", "024", None).unwrap();
    let err = create_sub_module(&ito_path, "auth", "024", None).unwrap_err();
    assert!(
        matches!(err, CreateError::DuplicateSubModuleName(_, _)),
        "expected DuplicateSubModuleName, got {err:?}"
    );
}

#[test]
fn create_sub_module_rejects_invalid_name() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    let err = create_sub_module(&ito_path, "Auth_Module", "024", None).unwrap_err();
    assert!(
        matches!(err, CreateError::InvalidChangeName(_)),
        "expected InvalidChangeName, got {err:?}"
    );
}

#[test]
fn create_sub_module_accepts_full_module_folder_name() {
    let tmp = TempDir::new().unwrap();
    let ito_path = setup_ito(&tmp);
    make_module(&ito_path, "024", "ito-backend");

    // Should resolve "024_ito-backend" to parent id "024".
    let result = create_sub_module(&ito_path, "auth", "024_ito-backend", None).unwrap();
    assert_eq!(result.parent_module_id, "024");
    assert_eq!(result.sub_module_id, "024.01");
}
