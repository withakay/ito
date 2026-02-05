use ito_test_support::run_rust_candidate;

#[test]
fn init_writes_gitignore_session_json_and_is_idempotent() {
    let repo = tempfile::tempdir().expect("repo");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    let args = ["init", "--tools", "none", "."];
    let rs = run_rust_candidate(rust_path, &args, repo.path(), home.path()).normalized(home.path());
    assert_eq!(rs.code, 0);

    let gitignore = std::fs::read_to_string(repo.path().join(".gitignore")).expect(".gitignore");
    assert!(gitignore.lines().any(|l| l.trim() == ".ito/session.json"));

    // Second init should not create duplicates.
    let args = ["init", "--force", "--tools", "none", "."];
    let rs = run_rust_candidate(rust_path, &args, repo.path(), home.path()).normalized(home.path());
    assert_eq!(rs.code, 0);

    let gitignore = std::fs::read_to_string(repo.path().join(".gitignore")).expect(".gitignore");
    let count = gitignore
        .lines()
        .map(|l| l.trim())
        .filter(|l| *l == ".ito/session.json")
        .count();
    assert_eq!(count, 1);
}
