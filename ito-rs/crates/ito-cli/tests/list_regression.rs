use std::path::Path;
use std::time::Duration;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent directories should exist");
    }
    std::fs::write(path, contents).expect("fixture file should write");
}

fn make_change(repo: &Path, id: &str, tasks: &str) {
    write(
        repo.join(".ito/changes").join(id).join("proposal.md"),
        "## Why\nfixture\n\n## What Changes\n- fixture\n\n## Impact\n- fixture\n",
    );
    write(repo.join(".ito/changes").join(id).join("tasks.md"), tasks);
    write(
        repo.join(".ito/changes")
            .join(id)
            .join("specs")
            .join("alpha")
            .join("spec.md"),
        "## ADDED Requirements\n\n### Requirement: Fixture\nFixture requirement.\n\n#### Scenario: Works\n- **WHEN** fixture runs\n- **THEN** it is ready\n",
    );
}

fn make_repo() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");

    make_change(
        repo.path(),
        "000-01_old-pending",
        "## 1. Implementation\n- [ ] 1.1 pending\n",
    );
    std::thread::sleep(Duration::from_millis(20));

    make_change(
        repo.path(),
        "000-02_mid-partial",
        "## 1. Implementation\n- [x] 1.1 done\n- [ ] 1.2 pending\n",
    );
    std::thread::sleep(Duration::from_millis(20));

    make_change(
        repo.path(),
        "000-03_new-complete",
        "## 1. Implementation\n- [x] 1.1 done\n",
    );

    repo
}

fn extract_names(stdout: &str) -> Vec<String> {
    let parsed: serde_json::Value = serde_json::from_str(stdout).expect("valid json output");
    parsed["changes"]
        .as_array()
        .expect("changes should be an array")
        .iter()
        .map(|item| {
            item["name"]
                .as_str()
                .expect("name should be string")
                .to_string()
        })
        .collect()
}

#[test]
fn list_default_text_and_json_shape_regression() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(rust_path, &["list"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Changes:"));
    assert!(out.stdout.contains("000-03_new-complete"));
    assert!(out.stdout.contains("000-02_mid-partial"));
    assert!(out.stdout.contains("000-01_old-pending"));

    let out = run_rust_candidate(rust_path, &["list", "--json"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&out.stdout).expect("list json");
    let first = &parsed["changes"][0];
    assert!(first.get("name").is_some());
    assert!(first.get("completedTasks").is_some());
    assert!(first.get("shelvedTasks").is_some());
    assert!(first.get("inProgressTasks").is_some());
    assert!(first.get("pendingTasks").is_some());
    assert!(first.get("totalTasks").is_some());
    assert!(first.get("lastModified").is_some());
    assert!(first.get("status").is_some());
    assert!(first.get("workStatus").is_some());
    assert!(first.get("completed").is_some());
}

#[test]
fn list_filters_regression() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["list", "--ready", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(extract_names(&out.stdout).len(), 2);

    let out = run_rust_candidate(
        rust_path,
        &["list", "--pending", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(extract_names(&out.stdout), vec!["000-01_old-pending"]);

    let out = run_rust_candidate(
        rust_path,
        &["list", "--partial", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(extract_names(&out.stdout), vec!["000-02_mid-partial"]);

    let out = run_rust_candidate(
        rust_path,
        &["list", "--completed", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(extract_names(&out.stdout), vec!["000-03_new-complete"]);
}

#[test]
fn list_sort_regression() {
    let repo = make_repo();
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let out = run_rust_candidate(
        rust_path,
        &["list", "--sort", "name", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(
        extract_names(&out.stdout),
        vec![
            "000-01_old-pending",
            "000-02_mid-partial",
            "000-03_new-complete"
        ]
    );

    let out = run_rust_candidate(
        rust_path,
        &["list", "--sort", "recent", "--json"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert_eq!(
        extract_names(&out.stdout),
        vec![
            "000-03_new-complete",
            "000-02_mid-partial",
            "000-01_old-pending"
        ]
    );
}
