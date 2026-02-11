#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn config_set_get_supports_coordination_branch_keys() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.enabled",
            "false",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.name",
            "ito/internal/custom-changes",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "changes.coordination_branch.enabled"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(out.stdout.trim(), "false");

    let out = run_rust_candidate(
        rust_path,
        &["config", "get", "changes.coordination_branch.name"],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert_eq!(out.stdout.trim(), "ito/internal/custom-changes");
}

#[test]
fn config_set_rejects_invalid_coordination_branch_name() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    std::fs::create_dir_all(home.path().join(".config/ito")).expect("create config dir");

    let out = run_rust_candidate(
        rust_path,
        &[
            "config",
            "set",
            "changes.coordination_branch.name",
            "bad..branch",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("Invalid value 'bad..branch'"));
    assert!(out.stderr.contains("changes.coordination_branch.name"));
}
