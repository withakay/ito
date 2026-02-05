#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn tasks_add_shelve_unshelve_show_cover_more_paths() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/test-change")).unwrap();

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &[
            "tasks",
            "add",
            "test-change",
            "Write more tests",
            "--wave",
            "2",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "shelve", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "unshelve", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "show", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Tasks for:"));
    assert!(out.stdout.contains("## Wave 1"));
}

#[test]
fn tasks_complete_supports_checkbox_compat_mode() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let change_dir = repo.path().join(".ito/changes/test-change");
    std::fs::create_dir_all(&change_dir).unwrap();
    fixtures::write(
        change_dir.join("tasks.md"),
        "## Tasks\n- [ ] 1.1 Do the thing\n- [ ] 1.2 Do another thing\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "complete", "test-change", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let md = std::fs::read_to_string(change_dir.join("tasks.md")).expect("tasks.md");
    assert!(md.contains("- [x] 1.1"));
}

#[test]
fn tasks_error_paths_cover_more_branches() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito/changes/test-change")).unwrap();

    // status when missing tasks.md
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "status", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("No tasks.md found"));

    // init twice
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "init", "test-change"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("already exists"));

    // unshelve when not shelved
    let out = run_rust_candidate(
        rust_path,
        &["tasks", "unshelve", "test-change", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("not shelved"));

    // checkbox-only: add not supported; start uses 1-based index
    let change_dir = repo.path().join(".ito/changes/compat");
    std::fs::create_dir_all(&change_dir).unwrap();
    fixtures::write(
        change_dir.join("tasks.md"),
        "## Tasks\n- [ ] 1.1 Do the thing\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "add", "compat", "Nope"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("checkbox-only"));

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "compat", "1.1"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("not found"));
}

#[test]
fn tasks_start_supports_checkbox_compat_mode_and_enforces_single_in_progress() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let change_dir = repo.path().join(".ito/changes/compat");
    std::fs::create_dir_all(&change_dir).unwrap();
    fixtures::write(
        change_dir.join("tasks.md"),
        "## Tasks\n- [ ] first\n- [ ] second\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "compat", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    let md = std::fs::read_to_string(change_dir.join("tasks.md")).expect("tasks.md");
    assert!(md.contains("- [~] first"));

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "compat", "2"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("in-progress"));
}

#[test]
fn tasks_next_supports_checkbox_compat_mode_and_shows_current_or_next() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let change_dir = repo.path().join(".ito/changes/compat");
    std::fs::create_dir_all(&change_dir).unwrap();
    fixtures::write(
        change_dir.join("tasks.md"),
        "## Tasks\n- [ ] first\n- [ ] second\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "next", "compat"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Next Task (compat)"));
    assert!(out.stdout.contains("Task 1: first"));
    assert!(
        out.stdout
            .contains("Run \"ito tasks start compat 1\" to begin")
    );

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "start", "compat", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "next", "compat"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Current Task (compat)"));
    assert!(out.stdout.contains("Task 1: first"));

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "complete", "compat", "1"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);

    let out = run_rust_candidate(
        rust_path,
        &["tasks", "next", "compat"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Next Task (compat)"));
    assert!(out.stdout.contains("Task 2: second"));
}
