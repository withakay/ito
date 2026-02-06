use ito_core::change_repository::FsChangeRepository;
use ito_core::module_repository::FsModuleRepository;
use ito_core::validate::{validate_change, validate_module, validate_spec_markdown};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn validate_spec_markdown_reports_missing_purpose_and_requirements() {
    let md = r#"
## Purpose

## Requirements
"#;

    let r = validate_spec_markdown(md, false);
    assert!(!r.valid);
    assert!(r.summary.errors >= 1);
}

#[test]
fn validate_spec_markdown_strict_treats_warnings_as_invalid() {
    let md = r#"
## Purpose

Too short.

## Requirements

### Requirement: R
The system SHALL do it.

#### Scenario: S
ok
"#;

    let non_strict = validate_spec_markdown(md, false);
    assert!(non_strict.valid);
    assert!(non_strict.summary.warnings >= 1);

    let strict = validate_spec_markdown(md, true);
    assert!(!strict.valid);
    assert!(strict.summary.warnings >= 1);
}

#[test]
fn validate_change_requires_at_least_one_delta() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(ito.join("changes").join("001-01_demo")).unwrap();
    let change_repo = FsChangeRepository::new(&ito);

    let r = validate_change(&change_repo, "001-01_demo", false).unwrap();
    assert!(!r.valid);
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("at least one delta"))
    );
}

#[test]
fn validate_change_requires_shall_or_must_in_requirement_text() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";
    let change_repo = FsChangeRepository::new(&ito);

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("auth")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: R
This requirement has no keywords.

#### Scenario: S
ok
"#,
    );

    let r = validate_change(&change_repo, change_id, false).unwrap();
    assert!(!r.valid);
    assert!(
        r.issues
            .iter()
            .any(|i| i.message.contains("SHALL") || i.message.contains("MUST"))
    );
}

#[test]
fn validate_module_reports_missing_scope_and_short_purpose() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_repo = FsModuleRepository::new(&ito);

    write(
        &ito.join("modules").join("006_demo").join("module.md"),
        r#"
## Purpose

Too short.

## Scope
"#,
    );

    let (_name, r) = validate_module(&module_repo, &ito, "006_demo", false).unwrap();
    assert!(!r.valid);
    assert!(r.summary.errors >= 1);
}
