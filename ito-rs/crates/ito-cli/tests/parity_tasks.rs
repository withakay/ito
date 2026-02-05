use std::path::Path;

use ito_test_support::{collect_file_bytes, reset_dir, run_rust_candidate};

fn make_base_repo() -> tempfile::TempDir {
    let td = tempfile::tempdir().expect("repo");
    std::fs::write(td.path().join("README.md"), "# temp\n").unwrap();
    // Minimal scaffolded change directory.
    let change_dir = td.path().join(".ito").join("changes").join("test-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    td
}

fn reset_repo(dst: &Path, src: &Path) {
    reset_dir(dst, src).unwrap();
}

#[test]
fn parity_tasks_init_writes_same_file() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let args = ["tasks", "init", "test-change"];

    reset_repo(repo.path(), base.path());
    let rs = run_rust_candidate(rust_path, &args, repo.path(), home.path()).normalized(home.path());
    let rs_fs = collect_file_bytes(repo.path());

    assert_eq!(rs.code, 0);
    assert!(rs_fs.contains_key(".ito/changes/test-change/tasks.md"));
    let md = std::fs::read_to_string(repo.path().join(".ito/changes/test-change/tasks.md"))
        .expect("tasks.md");
    assert!(md.contains("## Wave 1"));
    assert!(md.contains("- **Depends On**:"));
    assert!(md.contains("- **Updated At**:"));
}

#[test]
fn parity_tasks_status_next_start_complete_match_oracle() {
    let base = make_base_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    // Init first.
    let init_args = ["tasks", "init", "test-change"];
    reset_repo(repo.path(), base.path());
    let rs_init =
        run_rust_candidate(rust_path, &init_args, repo.path(), home.path()).normalized(home.path());
    assert_eq!(rs_init.code, 0);

    // Status output.
    let status_args = ["tasks", "status", "test-change"];
    let rs = run_rust_candidate(rust_path, &status_args, repo.path(), home.path())
        .normalized(home.path());

    assert_eq!(rs.code, 0);
    assert!(rs.stdout.contains("Progress:"));
    assert!(rs.stdout.contains("Ready"));
    assert!(rs.stdout.contains("Blocked"));

    // Next output.
    let next_args = ["tasks", "next", "test-change"];
    let rs =
        run_rust_candidate(rust_path, &next_args, repo.path(), home.path()).normalized(home.path());

    assert_eq!(rs.code, 0);
    assert!(rs.stdout.contains("Next Task"));

    // Start 1.1.
    let start_args = ["tasks", "start", "test-change", "1.1"];
    let rs = run_rust_candidate(rust_path, &start_args, repo.path(), home.path())
        .normalized(home.path());
    let rs_fs = collect_file_bytes(repo.path());

    assert_eq!(rs.code, 0);
    assert!(rs_fs.contains_key(".ito/changes/test-change/tasks.md"));

    // Complete 1.1.
    let complete_args = ["tasks", "complete", "test-change", "1.1"];
    let rs = run_rust_candidate(rust_path, &complete_args, repo.path(), home.path())
        .normalized(home.path());
    let rs_fs = collect_file_bytes(repo.path());

    assert_eq!(rs.code, 0);
    assert!(rs_fs.contains_key(".ito/changes/test-change/tasks.md"));
}
