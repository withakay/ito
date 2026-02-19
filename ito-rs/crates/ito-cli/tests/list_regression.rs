use std::path::Path;

use ito_test_support::run_rust_candidate;

fn write(path: impl AsRef<Path>, contents: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent directories should exist");
    }
    std::fs::write(path, contents).expect("fixture file should write");
}

/// Creates a change entry under `repo/.ito/changes/{id}/` with fixture content.
///
/// The function writes three files for the change:
/// - `proposal.md` with a fixed proposal header,
/// - `tasks.md` with the provided `tasks` content,
/// - `specs/alpha/spec.md` with a fixed spec containing requirements and a scenario.
///
/// # Parameters
///
/// - `repo`: filesystem path to the repository root where `.ito/changes` will be created.
/// - `id`: identifier for the change; used as the directory name under `.ito/changes`.
/// - `tasks`: content to write into the change's `tasks.md`.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use tempfile::tempdir;
///
/// let tmp = tempdir().unwrap();
/// let repo = tmp.path();
/// make_change(repo, "000-01_example", "- [ ] example task\n");
/// assert!(repo.join(".ito/changes/000-01_example/proposal.md").exists());
/// assert!(repo.join(".ito/changes/000-01_example/tasks.md").exists());
/// assert!(repo.join(".ito/changes/000-01_example/specs/alpha/spec.md").exists());
/// ```
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

/// Set the modification time for `dir` and all files and subdirectories inside it recursively.
///
/// This will update the mtime of `dir` itself and every entry contained within it. The function
/// will panic if reading the directory or setting any file's modification time fails.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use filetime::FileTime;
///
/// let tmp = tempfile::tempdir().unwrap();
/// let dir = tmp.path().join("foo");
/// std::fs::create_dir_all(&dir).unwrap();
/// let t = FileTime::from_unix_time(1_600_000_000, 0);
/// set_mtime_recursive(Path::new(&dir), t);
/// ```
fn set_mtime_recursive(dir: &Path, time: filetime::FileTime) {
    filetime::set_file_mtime(dir, time).expect("set dir mtime");
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        filetime::set_file_mtime(&path, time).expect("set entry mtime");
        if path.is_dir() {
            set_mtime_recursive(&path, time);
        }
    }
}

/// Create a temporary repository fixture containing three change entries with deterministic mtimes.
///
/// The repository includes a README.md and three changes under `.ito/changes/`:
/// - `000-01_old-pending` (one pending task)
/// - `000-02_mid-partial` (one completed and one pending task)
/// - `000-03_new-complete` (one completed task)
///
/// Modification times for each change directory are set deterministically so tests that sort by recency behave consistently.
///
/// # Examples
///
/// ```
/// let repo = make_repo();
/// assert!(repo.path().join("README.md").exists());
/// assert!(repo.path().join(".ito/changes/000-01_old-pending").exists());
/// ```
fn make_repo() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");

    make_change(
        repo.path(),
        "000-01_old-pending",
        "## 1. Implementation\n- [ ] 1.1 pending\n",
    );

    make_change(
        repo.path(),
        "000-02_mid-partial",
        "## 1. Implementation\n- [x] 1.1 done\n- [ ] 1.2 pending\n",
    );

    make_change(
        repo.path(),
        "000-03_new-complete",
        "## 1. Implementation\n- [x] 1.1 done\n",
    );

    // Set explicit mtimes recursively so sort-by-recent is deterministic without sleeping.
    // last_modified_recursive() walks all files, so every entry must be set.
    let changes = repo.path().join(".ito/changes");
    let t1 = filetime::FileTime::from_unix_time(1_000_000, 0);
    let t2 = filetime::FileTime::from_unix_time(2_000_000, 0);
    let t3 = filetime::FileTime::from_unix_time(3_000_000, 0);
    set_mtime_recursive(&changes.join("000-01_old-pending"), t1);
    set_mtime_recursive(&changes.join("000-02_mid-partial"), t2);
    set_mtime_recursive(&changes.join("000-03_new-complete"), t3);

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
    let idx_01 = out
        .stdout
        .find("000-01_old-pending")
        .expect("default list output should contain 000-01_old-pending");
    let idx_02 = out
        .stdout
        .find("000-02_mid-partial")
        .expect("default list output should contain 000-02_mid-partial");
    let idx_03 = out
        .stdout
        .find("000-03_new-complete")
        .expect("default list output should contain 000-03_new-complete");
    assert!(
        idx_01 < idx_02 && idx_02 < idx_03,
        "default text list should be ascending by change ID"
    );

    let out = run_rust_candidate(rust_path, &["list", "--json"], repo.path(), home.path());
    assert_eq!(out.code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&out.stdout).expect("list json");
    assert_eq!(
        extract_names(&out.stdout),
        vec![
            "000-01_old-pending",
            "000-02_mid-partial",
            "000-03_new-complete"
        ]
    );
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
