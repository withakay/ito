#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

fn make_repo_with_archives() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    fixtures::write(td.path().join("README.md"), "# temp\n");
    fixtures::write(
        td.path().join(".ito/changes/000-01_active/proposal.md"),
        "## Why\nActive fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path()
            .join(".ito/changes/archive/2026-04-01-000-02_archived/proposal.md"),
        "## Why\nArchived fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    fixtures::write(
        td.path()
            .join(".ito/changes/archive/2026-04-02-000-03_done/tasks.md"),
        "## 1. Implementation\n- [x] 1.1 Done\n",
    );
    td
}

#[test]
fn list_archive_lists_archived_changes_only() {
    let base = make_repo_with_archives();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["list-archive"], repo.path(), home.path());

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Archived changes:"), "{}", out.stdout);
    assert!(out.stdout.contains("000-02_archived"), "{}", out.stdout);
    assert!(out.stdout.contains("000-03_done"), "{}", out.stdout);
    assert!(!out.stdout.contains("000-01_active"), "{}", out.stdout);
}

#[test]
fn list_archive_json_lists_archived_changes_only() {
    let base = make_repo_with_archives();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["list-archive", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let payload: serde_json::Value = serde_json::from_str(&out.stdout).expect(&out.stdout);
    let archived = payload
        .get("archived")
        .and_then(|value| value.as_array())
        .expect(&out.stdout);
    let mut names: Vec<&str> = Vec::with_capacity(archived.len());
    for value in archived {
        if let Some(name) = value.get("name").and_then(|name| name.as_str()) {
            names.push(name);
        }
    }

    assert_eq!(names, vec!["000-02_archived", "000-03_done"]);
}

#[test]
fn list_archive_reports_empty_archives() {
    let base = tempfile::tempdir().expect("repo");
    fixtures::write(base.path().join("README.md"), "# temp\n");
    // Create an initialized .ito project with an empty archive directory so
    // this test exercises the empty-archive case rather than an uninitialized
    // project. Keep an active change present so the project is recognised.
    fixtures::write(
        base.path().join(".ito/changes/000-01_active/proposal.md"),
        "## Why\nActive fixture\n\n## What Changes\n- None\n\n## Impact\n- None\n",
    );
    fixtures::write(base.path().join(".ito/changes/archive/.gitkeep"), "");

    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["list-archive"], repo.path(), home.path());

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout.contains("No archived changes found."),
        "{}",
        out.stdout
    );
}
