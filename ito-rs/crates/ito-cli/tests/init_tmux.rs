#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[cfg(unix)]
use ito_test_support::pty::run_pty_interactive;

#[test]
fn init_writes_tmux_enabled_true_by_default() {
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
        json.get("tools")
            .and_then(|v| v.get("tmux"))
            .and_then(|v| v.get("enabled"))
            .and_then(|v| v.as_bool())
            == Some(true),
        "expected tools.tmux.enabled to default true\nGot:\n{config}"
    );
}

#[test]
fn init_with_no_tmux_writes_tmux_enabled_false() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
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
    assert_eq!(out.code, 0, "init failed: {}", out.stderr);

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&config).unwrap();
    assert!(
        json.get("tools")
            .and_then(|v| v.get("tmux"))
            .and_then(|v| v.get("enabled"))
            .and_then(|v| v.as_bool())
            == Some(false),
        "expected tools.tmux.enabled to be false with --no-tmux\nGot:\n{config}"
    );
}

#[test]
#[cfg(unix)]
#[ignore = "PTY interactive test hangs in CI; run locally with --include-ignored"]
fn init_interactive_can_disable_tmux_preference() {
    let base = fixtures::make_empty_repo();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_pty_interactive(
        rust_path,
        &["init", repo.path().to_string_lossy().as_ref()],
        repo.path(),
        home.path(),
        "\n\n\nn\n\n",
    );
    assert_eq!(out.code, 0, "stdout={}", out.stdout);
    assert!(out.stdout.contains("Do you use tmux?"), "{}", out.stdout);

    let config = std::fs::read_to_string(repo.path().join(".ito/config.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&config).unwrap();
    assert!(
        json.get("tools")
            .and_then(|v| v.get("tmux"))
            .and_then(|v| v.get("enabled"))
            .and_then(|v| v.as_bool())
            == Some(false),
        "expected tools.tmux.enabled to be false\nGot:\n{config}"
    );
}
