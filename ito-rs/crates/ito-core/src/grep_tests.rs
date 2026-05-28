use super::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_dir(tmp: &TempDir) -> PathBuf {
    let ito = tmp.path().join(".ito");
    fs::create_dir_all(ito.join("changes/001-01_test-change/specs/auth")).unwrap();
    fs::write(
        ito.join("changes/001-01_test-change/proposal.md"),
        "# Proposal\n\nThis adds auth support.\n",
    )
    .unwrap();
    fs::write(
        ito.join("changes/001-01_test-change/tasks.md"),
        "# Tasks\n- [ ] Add login endpoint\n- [ ] Add tests\n",
    )
    .unwrap();
    fs::write(
            ito.join("changes/001-01_test-change/specs/auth/spec.md"),
            "## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL provide login.\n\n#### Scenario: Success\n- **WHEN** valid creds\n- **THEN** token returned\n",
        )
        .unwrap();
    ito
}

#[test]
fn collect_change_artifact_files_finds_all_md_files() {
    let tmp = TempDir::new().unwrap();
    let ito = setup_test_dir(&tmp);
    let change_dir = ito.join("changes/001-01_test-change");

    let files = collect_change_artifact_files(&change_dir);
    assert_eq!(files.len(), 3); // proposal, tasks, specs/auth/spec.md (no design.md)

    let mut names: Vec<String> = Vec::new();
    for p in &files {
        names.push(
            p.strip_prefix(&change_dir)
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );
    }
    assert!(names.contains(&"proposal.md".to_string()));
    assert!(names.contains(&"tasks.md".to_string()));
    assert!(names.contains(&"specs/auth/spec.md".to_string()));
}

#[test]
fn search_files_finds_matching_lines() {
    let tmp = TempDir::new().unwrap();
    let ito = setup_test_dir(&tmp);
    let change_dir = ito.join("changes/001-01_test-change");

    let files = collect_change_artifact_files(&change_dir);
    let output = search_files(&files, "Requirement:", 0).unwrap();

    assert_eq!(output.matches.len(), 1);
    assert!(output.matches[0].line.contains("Requirement: Login"));
    assert!(!output.truncated);
}

#[test]
fn search_files_respects_limit() {
    let tmp = TempDir::new().unwrap();
    let ito = setup_test_dir(&tmp);
    let change_dir = ito.join("changes/001-01_test-change");

    let files = collect_change_artifact_files(&change_dir);
    // Search for something that matches many lines
    let output = search_files(&files, ".", 2).unwrap();

    assert_eq!(output.matches.len(), 2);
    assert!(output.truncated);
}

#[test]
fn search_files_returns_empty_for_no_matches() {
    let tmp = TempDir::new().unwrap();
    let ito = setup_test_dir(&tmp);
    let change_dir = ito.join("changes/001-01_test-change");

    let files = collect_change_artifact_files(&change_dir);
    let output = search_files(&files, "ZZZZZZZ_NOMATCH", 0).unwrap();

    assert!(output.matches.is_empty());
    assert!(!output.truncated);
}

#[test]
fn search_files_rejects_invalid_regex() {
    let files = vec![PathBuf::from("/nonexistent")];
    let result = search_files(&files, "[invalid", 0);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid grep pattern"));
}

#[test]
fn search_files_includes_correct_line_numbers() {
    let tmp = TempDir::new().unwrap();
    let ito = setup_test_dir(&tmp);

    let files = vec![ito.join("changes/001-01_test-change/tasks.md")];
    let output = search_files(&files, r"Add", 0).unwrap();

    assert_eq!(output.matches.len(), 2);
    // "Add login endpoint" should be line 2, "Add tests" should be line 3
    assert_eq!(output.matches[0].line_number, 2);
    assert_eq!(output.matches[1].line_number, 3);
}
