#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn new_change_covers_happy_and_error_paths() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["new", "change"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Missing required argument"));
    assert!(out.stderr.contains("<name>"));

    let out = run_rust_candidate(
        rust_path,
        &["new", "change", "BadName"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);

    // Unknown module should error.
    let out = run_rust_candidate(
        rust_path,
        &["new", "change", "add-thing", "--module", "001_demo"],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);

    // Happy path.
    let out = run_rust_candidate(
        rust_path,
        &[
            "new",
            "change",
            "add-thing",
            "--schema",
            "spec-driven",
            "--description",
            "desc",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(out.stderr.contains("✔ Created change"));
    assert!(out.stderr.contains("Schema: spec-driven"));
}
