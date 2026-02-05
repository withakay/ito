use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, contents).unwrap();
}

fn make_base_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    write(td.path().join("README.md"), "# temp\n");

    // Minimal module.
    write(
        td.path().join(".ito/modules/000_ungrouped/module.md"),
        "# Ungrouped\n\n## Purpose\nModule for archive tests. This purpose is long enough.\n\n## Scope\n- *\n\n## Changes\n- [ ] 000-01_test-change\n",
    );

    // Spec exists in main so archive will update it.
    write(
        td.path().join(".ito/specs/alpha/spec.md"),
        "# Alpha\n\n## Purpose\nThis purpose text is intentionally long enough to avoid strict-mode warnings.\n\n## Requirements\n\n### Requirement: Alpha Behavior\nThe system SHALL do the alpha thing.\n\n#### Scenario: Alpha works\n- **WHEN** the user triggers alpha\n- **THEN** the system performs alpha\n",
    );

    // Change with a completed task and a delta spec.
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/proposal.md"),
        "## Why\nTest fixture\n\n## What Changes\n- Adds a small delta\n\n## Impact\n- None\n",
    );
    write(
        td.path().join(".ito/changes/000-01_test-change/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Do a thing\n",
    );
    write(
        td.path()
            .join(".ito/changes/000-01_test-change/specs/alpha/spec.md"),
        "## ADDED Requirements\n\n### Requirement: Alpha Delta\nThe system SHALL include alpha delta behavior in strict validation.\n\n#### Scenario: Delta ok\n- **WHEN** running validation\n- **THEN** it passes\n",
    );

    td
}

fn reset_repo(dst: &Path, src: &Path) {
    ito_test_support::reset_dir(dst, src).unwrap();
}

#[test]
fn archive_with_specs_and_validation_smoke() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["archive", "000-01_test-change", "-y"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let archive_root = repo.path().join(".ito/changes/archive");
    let mut found = false;
    for e in archive_root.read_dir().expect("archive dir") {
        let e = e.unwrap();
        if e.file_name()
            .to_string_lossy()
            .contains("000-01_test-change")
        {
            found = true;
        }
    }
    assert!(found);

    // Module should be marked complete.
    let module_md =
        std::fs::read_to_string(repo.path().join(".ito/modules/000_ungrouped/module.md"))
            .expect("module.md");
    assert!(module_md.contains("- [x] 000-01_test-change"));
}
