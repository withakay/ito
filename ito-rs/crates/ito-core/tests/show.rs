use ito_core::change_repository::FsChangeRepository;
use ito_core::show::{
    DeltaSpecFile, load_delta_spec_file, parse_change_show_json, parse_spec_show_json,
    read_change_delta_spec_files,
};
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn parse_spec_show_json_extracts_overview_requirements_and_scenarios() {
    let md = r#"
## Purpose

This spec exists to prove parsing works for tests. It is long enough to pass warnings.

## Requirements

### Requirement: The system SHALL do something
The system SHALL do something.

#### Scenario: Happy path
Given A
When B
Then C

### Requirement: The system MUST do another thing
The system MUST do another thing.

#### Scenario: Another path
Given X
Then Y
"#;

    let json = parse_spec_show_json("spec-id", md);
    assert_eq!(json.id, "spec-id");
    assert!(json.overview.contains("This spec exists"));
    assert_eq!(json.requirement_count, 2);
    assert_eq!(json.requirements.len(), 2);
    assert_eq!(json.requirements[0].scenarios.len(), 1);
    assert!(
        json.requirements[0].scenarios[0]
            .raw_text
            .contains("Given A")
    );
}

#[test]
fn read_change_delta_spec_files_lists_specs_sorted() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let change_id = "001-01_demo";

    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("b")
            .join("spec.md"),
        "# b\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("a")
            .join("spec.md"),
        "# a\n",
    );

    let repo = FsChangeRepository::new(&ito);
    let files = read_change_delta_spec_files(&repo, change_id).unwrap();
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].spec, "a");
    assert_eq!(files[1].spec, "b");
}

#[test]
fn load_delta_spec_file_uses_parent_dir_name_as_spec() {
    let td = tempfile::tempdir().unwrap();
    let path = td.path().join("auth").join("spec.md");
    write(&path, "# auth\n");

    let f = load_delta_spec_file(&path).unwrap();
    assert_eq!(f.spec, "auth");
    assert!(f.markdown.contains("# auth"));
}

#[test]
fn parse_change_show_json_emits_deltas_with_operations() {
    let files = vec![DeltaSpecFile {
        spec: "auth".to_string(),
        markdown: r#"
## ADDED Requirements

### Requirement: Added thing
The system SHALL add a thing.

#### Scenario: S
Given A
Then B
"#
        .to_string(),
    }];

    let json = parse_change_show_json("001-01_demo", &files);
    assert_eq!(json.delta_count, 1);
    assert_eq!(json.deltas.len(), 1);
    assert_eq!(json.deltas[0].spec, "auth");
    assert_eq!(json.deltas[0].operation, "ADDED");
    assert!(json.deltas[0].description.contains("Add requirement"));
}
