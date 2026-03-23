use ito_core::change_repository::FsChangeRepository;
use ito_core::errors::CoreError;
use ito_core::module_repository::FsModuleRepository;
use ito_core::show::{
    DeltaSpecFile, bundle_main_specs_markdown, bundle_main_specs_show_json, load_delta_spec_file,
    parse_change_show_json, parse_spec_show_json, read_change_delta_spec_files,
    read_module_markdown,
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

#[test]
fn read_module_markdown_returns_contents_for_existing_module() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    let module_content = "# My Module\n\n## Purpose\nDoes things.\n";
    write(
        &ito.join("modules").join("006_demo").join("module.md"),
        module_content,
    );

    let module_repo = FsModuleRepository::new(&ito);
    let result = read_module_markdown(&module_repo, "006").expect("should read module.md");
    assert_eq!(result, module_content);
}

#[test]
fn read_module_markdown_returns_empty_for_missing_module_md() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    // Create the module directory but not the module.md file
    std::fs::create_dir_all(ito.join("modules").join("007_empty")).unwrap();

    let module_repo = FsModuleRepository::new(&ito);
    let result =
        read_module_markdown(&module_repo, "007").expect("should succeed with empty string");
    assert!(
        result.is_empty(),
        "should return empty string for missing module.md, got: {result}"
    );
}

#[test]
fn read_module_markdown_returns_error_for_nonexistent_module() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    // Don't create any modules directory

    let module_repo = FsModuleRepository::new(&ito);
    let result = read_module_markdown(&module_repo, "999");
    assert!(result.is_err(), "should fail for nonexistent module");
}

#[test]
fn bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    write(
        &ito.join("specs").join("b").join("spec.md"),
        "# B\n\n## Purpose\nB purpose long enough.\n\n## Requirements\n\n### Requirement: B\nThe system SHALL do b.\n\n#### Scenario: B\n- **WHEN** b\n- **THEN** b\n",
    );
    write(
        &ito.join("specs").join("a").join("spec.md"),
        "# A\n\n## Purpose\nA purpose long enough.\n\n## Requirements\n\n### Requirement: A\nThe system SHALL do a.\n\n#### Scenario: A\n- **WHEN** a\n- **THEN** a\n",
    );

    // Add a delta spec elsewhere to ensure it is ignored by bundling.
    write(
        &ito.join("changes")
            .join("000-01_demo")
            .join("specs")
            .join("zzz")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Delta\nThe system SHALL not appear.\n\n#### Scenario: Delta\n- **WHEN** bundled\n- **THEN** excluded\n",
    );

    let json = bundle_main_specs_show_json(&ito).unwrap();
    assert_eq!(json.spec_count, 2);
    assert_eq!(json.specs.len(), 2);
    assert_eq!(json.specs[0].id, "a");
    assert_eq!(json.specs[1].id, "b");

    let a_path = ito_common::paths::spec_markdown_path(&ito, "a");
    let b_path = ito_common::paths::spec_markdown_path(&ito, "b");
    assert_eq!(json.specs[0].path, a_path.to_string_lossy().to_string());
    assert_eq!(json.specs[1].path, b_path.to_string_lossy().to_string());
    assert!(json.specs[0].markdown.contains("# A"));
    assert!(json.specs[1].markdown.contains("# B"));
}

#[test]
fn bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");

    write(
        &ito.join("specs").join("alpha").join("spec.md"),
        "# Alpha\n\n## Purpose\nAlpha purpose long enough.\n\n## Requirements\n\n### Requirement: Alpha\nThe system SHALL alpha.\n\n#### Scenario: Alpha\n- **WHEN** alpha\n- **THEN** alpha\n",
    );
    write(
        &ito.join("specs").join("beta").join("spec.md"),
        "# Beta\n\n## Purpose\nBeta purpose long enough.\n\n## Requirements\n\n### Requirement: Beta\nThe system SHALL beta.\n\n#### Scenario: Beta\n- **WHEN** beta\n- **THEN** beta\n",
    );
    write(
        &ito.join("changes")
            .join("000-01_demo")
            .join("specs")
            .join("delta")
            .join("spec.md"),
        "DELTA MARKDOWN MUST NOT APPEAR\n",
    );

    let md = bundle_main_specs_markdown(&ito).unwrap();
    let alpha_path = ito_common::paths::spec_markdown_path(&ito, "alpha");
    let beta_path = ito_common::paths::spec_markdown_path(&ito, "beta");

    assert!(md.contains(&format!(
        "<!-- spec-id: alpha; source: {} -->",
        alpha_path.to_string_lossy()
    )));
    assert!(md.contains(&format!(
        "<!-- spec-id: beta; source: {} -->",
        beta_path.to_string_lossy()
    )));
    assert!(md.contains("# Alpha"));
    assert!(md.contains("# Beta"));
    assert!(!md.contains("DELTA MARKDOWN MUST NOT APPEAR"));

    let alpha_idx = md.find("<!-- spec-id: alpha").unwrap();
    let beta_idx = md.find("<!-- spec-id: beta").unwrap();
    assert!(alpha_idx < beta_idx);
}
#[test]
fn bundle_main_specs_show_json_returns_not_found_when_no_specs_exist() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(&ito).unwrap();

    let err = bundle_main_specs_show_json(&ito).expect_err("should fail when no specs exist");
    let CoreError::NotFound(msg) = err else {
        panic!("expected not-found error, got: {err:?}");
    };
    assert!(
        msg.contains("No specs found"),
        "expected not-found message to mention no specs, got: {msg}"
    );
}

/// Verifies that a requirement block's `Requirement ID` bullet is parsed into `requirement_id` and removed from the requirement's stored text.
///
/// This test ensures `parse_spec_show_json` extracts the `- **Requirement ID**: ...` line into `requirement_id` for a requirement and that the collapsed `text` for the requirement does not include the `Requirement ID` line.
///
/// # Examples
///
/// ```
/// let md = r#"
/// ## Requirements
///
/// ### Requirement: The system SHALL authenticate users
/// The system SHALL authenticate users via OAuth2.
/// - **Requirement ID**: REQ-AUTH-001
///
/// #### Scenario: Happy path
/// Given a valid token
/// Then access is granted
/// "#;
///
/// let json = parse_spec_show_json("auth", md);
/// assert_eq!(json.requirements.len(), 1);
/// assert_eq!(json.requirements[0].requirement_id.as_deref(), Some("REQ-AUTH-001"));
/// assert!(!json.requirements[0].text.contains("Requirement ID"));
/// ```
#[test]
fn parse_requirement_block_extracts_requirement_id() {
    let md = r#"
## Requirements

### Requirement: The system SHALL authenticate users
The system SHALL authenticate users via OAuth2.
- **Requirement ID**: REQ-AUTH-001

#### Scenario: Happy path
Given a valid token
Then access is granted
"#;

    let json = parse_spec_show_json("auth", md);
    assert_eq!(json.requirements.len(), 1);
    assert_eq!(
        json.requirements[0].requirement_id.as_deref(),
        Some("REQ-AUTH-001")
    );
    // The ID line must NOT appear in the collapsed text.
    assert!(
        !json.requirements[0].text.contains("Requirement ID"),
        "requirement_id line should be excluded from text, got: {}",
        json.requirements[0].text
    );
}

#[test]
fn parse_requirement_block_requirement_id_absent_gives_none() {
    let md = r#"
## Requirements

### Requirement: The system SHALL do something
The system SHALL do something.

#### Scenario: S
Given A
Then B
"#;

    let json = parse_spec_show_json("spec", md);
    assert_eq!(json.requirements.len(), 1);
    assert!(
        json.requirements[0].requirement_id.is_none(),
        "expected None for requirement_id when not declared"
    );
}

/// Verifies parsing of multiple requirement blocks and extraction of declared requirement IDs.
///
/// Ensures three requirements are parsed from the markdown: the first two include `Requirement ID` lines
/// yielding `REQ-001` and `REQ-002`, and the third has no `Requirement ID`.
///
/// # Examples
///
/// ```
/// let md = r#"
/// ## Requirements
///
/// ### Requirement: First requirement
/// First requirement text.
/// - **Requirement ID**: REQ-001
///
/// ### Requirement: Second requirement
/// Second requirement text.
/// - **Requirement ID**: REQ-002
///
/// ### Requirement: Third requirement without ID
/// Third requirement text.
/// "#;
///
/// let json = parse_spec_show_json("spec", md);
/// assert_eq!(json.requirements.len(), 3);
/// assert_eq!(json.requirements[0].requirement_id.as_deref(), Some("REQ-001"));
/// assert_eq!(json.requirements[1].requirement_id.as_deref(), Some("REQ-002"));
/// assert!(json.requirements[2].requirement_id.is_none());
/// ```
#[test]
fn parse_requirement_block_multiple_requirements_with_ids() {
    let md = r#"
## Requirements

### Requirement: First requirement
First requirement text.
- **Requirement ID**: REQ-001

### Requirement: Second requirement
Second requirement text.
- **Requirement ID**: REQ-002

### Requirement: Third requirement without ID
Third requirement text.
"#;

    let json = parse_spec_show_json("spec", md);
    assert_eq!(json.requirements.len(), 3);
    assert_eq!(
        json.requirements[0].requirement_id.as_deref(),
        Some("REQ-001")
    );
    assert_eq!(
        json.requirements[1].requirement_id.as_deref(),
        Some("REQ-002")
    );
    assert!(json.requirements[2].requirement_id.is_none());
}

#[test]
fn parse_delta_spec_requirement_id_is_extracted() {
    let files = vec![DeltaSpecFile {
        spec: "auth".to_string(),
        markdown: r#"
## ADDED Requirements

### Requirement: The system SHALL support SSO
The system SHALL support SSO.
- **Requirement ID**: REQ-SSO-001

#### Scenario: SSO login
Given an SSO provider
Then the user is authenticated
"#
        .to_string(),
    }];

    let json = parse_change_show_json("001-27_demo", &files);
    assert_eq!(json.deltas.len(), 1);
    assert_eq!(
        json.deltas[0].requirement.requirement_id.as_deref(),
        Some("REQ-SSO-001")
    );
    // ID line must not appear in the requirement text.
    assert!(
        !json.deltas[0].requirement.text.contains("Requirement ID"),
        "requirement_id line should be excluded from text"
    );
}

/// Ensures bundling main specs fails with an IO error when a spec directory is missing `spec.md`.
///
/// # Examples
///
/// ```
/// let td = tempfile::tempdir().unwrap();
/// let ito = td.path().join(".ito");
/// std::fs::create_dir_all(ito.join("specs").join("orphan")).unwrap();
///
/// let err = bundle_main_specs_show_json(&ito).expect_err("should fail when spec.md missing");
/// match err {
///     CoreError::Io { context, .. } => assert!(context.contains("reading spec orphan")),
///     other => panic!("expected Io error, got: {other:?}"),
/// }
/// ```
#[test]
fn bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing() {
    let td = tempfile::tempdir().unwrap();
    let ito = td.path().join(".ito");
    std::fs::create_dir_all(ito.join("specs").join("orphan")).unwrap();

    let err = bundle_main_specs_show_json(&ito).expect_err("should fail when spec.md missing");
    let CoreError::Io { context, .. } = err else {
        panic!("expected io error, got: {err:?}");
    };
    assert!(
        context.contains("reading spec orphan"),
        "expected context to mention orphan spec, got: {context}"
    );
}
