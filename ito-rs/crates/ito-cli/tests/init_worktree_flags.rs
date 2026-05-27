#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn init_worktree_flags_write_project_local_worktree_config() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let repo_path = repo.path().to_string_lossy();
    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo_path.as_ref(),
            "--tools",
            "none",
            "--worktrees",
            "--worktree-strategy",
            "bare_control_siblings",
            "--worktree-integration-mode",
            "merge_parent",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "stderr={}", out.stderr);

    let config = std::fs::read_to_string(repo.path().join(".ito/config.local.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&config).unwrap();
    assert_eq!(json["worktrees"]["enabled"], true);
    assert_eq!(json["worktrees"]["strategy"], "bare_control_siblings");
    assert_eq!(
        json["worktrees"]["apply"]["integration_mode"],
        "merge_parent"
    );
}
