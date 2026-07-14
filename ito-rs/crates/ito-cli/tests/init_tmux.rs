#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn init_does_not_write_removed_tmux_config() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    let args = fixtures::init_minimal_args(repo.path());
    let argv = fixtures::args_to_strs(&args);
    let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&config).unwrap();
    assert!(
        json.get("tools").is_none(),
        "init must not write the removed tmux-only tools namespace\nGot:\n{config}"
    );
}

#[test]
fn init_help_and_parser_do_not_expose_no_tmux_flag() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::reset_repo(repo.path(), base.path());

    let help = run_rust_candidate(rust_path, &["init", "--help"], repo.path(), home.path());
    assert_eq!(help.code, 0, "help failed: {}", help.stderr);
    assert!(!help.stdout.contains("--no-tmux"), "{}", help.stdout);

    let rejected = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--no-tmux",
        ],
        repo.path(),
        home.path(),
    );
    assert_ne!(rejected.code, 0, "removed flag should be rejected");
    assert!(rejected.stderr.contains("--no-tmux"), "{}", rejected.stderr);
}

#[test]
fn init_reports_legacy_tmux_config_without_rewriting_it() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");
    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        r#"{"tools":{"tmux":{"enabled":false}}}"#,
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "init",
            repo.path().to_string_lossy().as_ref(),
            "--tools",
            "none",
            "--update",
        ],
        repo.path(),
        home.path(),
    );
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);
    assert!(
        out.stderr.contains("tools.tmux.enabled")
            && out.stderr.contains("removed")
            && out.stderr.contains("no effect"),
        "expected an actionable removed-key warning, got: {}",
        out.stderr
    );

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    assert!(
        config.contains("\"tmux\""),
        "loading config must not silently rewrite the user's file"
    );
}
