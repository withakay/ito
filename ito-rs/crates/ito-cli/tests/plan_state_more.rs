#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn plan_init_and_status_and_state_commands_cover_happy_and_error_paths() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(repo.path().join(".ito")).unwrap();

    // Errors when planning/state files are missing.
    let out = run_rust_candidate(rust_path, &["plan", "status"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("ROADMAP.md not found"));

    let out = run_rust_candidate(rust_path, &["state", "show"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("STATE.md not found"));

    // Init creates the structure.
    let out = run_rust_candidate(rust_path, &["plan", "init"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("Planning structure initialized"));
    assert!(out.stdout.contains(".ito/planning/STATE.md"));

    // Status now works.
    let out = run_rust_candidate(rust_path, &["plan", "status"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("Current Progress"));
    assert!(out.stdout.contains("Phases"));

    // State mutation writes to STATE.md.
    let out = run_rust_candidate(
        rust_path,
        &["state", "note", "hello"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Note recorded"));

    let out = run_rust_candidate(rust_path, &["state", "show"], repo.path(), home.path());
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stdout.contains("hello"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "decision", "ship"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Decision recorded"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "blocker", "waiting"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Blocker recorded"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "question", "why"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Question added"));

    let out = run_rust_candidate(
        rust_path,
        &["state", "focus", "now"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Focus updated"));
}
