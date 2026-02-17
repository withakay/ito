#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn serve_errors_when_not_initialized() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(rust_path, &["serve"], repo.path(), home.path());
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("No .ito directory"));
    assert!(out.stderr.contains("ito init"));
}
