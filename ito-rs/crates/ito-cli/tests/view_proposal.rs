mod support;

use assert_cmd::Command;
use support::write;

#[test]
fn view_proposal_help_shows_viewer_flag() {
    let mut command = Command::cargo_bin("ito").unwrap();
    command.args(["view", "proposal", "--help"]);

    command
        .assert()
        .success()
        .stdout(predicates::str::contains("--json"))
        .stdout(predicates::str::contains("--viewer <VIEWER>"))
        .stdout(predicates::str::contains("Change id (directory name)"));
}

#[test]
fn view_proposal_json_outputs_bundle() {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/changes/001-29_demo/proposal.md"),
        "## Why\nDemo\n",
    );
    write(
        repo.path().join(".ito/changes/001-29_demo/tasks.md"),
        "## Tasks\n- [ ] Verify\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.args(["view", "proposal", "001-29_demo", "--json"]);

    let assert = command.assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let output: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");

    assert_eq!(output["change_id"], "001-29_demo");
    assert!(
        output["content"]
            .as_str()
            .expect("content string")
            .contains("# proposal.md")
    );
}

#[test]
fn view_proposal_unknown_change_fails() {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    std::fs::create_dir_all(repo.path().join(".ito/changes")).unwrap();

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.args(["view", "proposal", "001-99_missing", "--viewer", "bat"]);

    command.assert().failure().stderr(predicates::str::contains(
        "Change '001-99_missing' not found",
    ));
}

#[test]
fn view_proposal_disabled_tmux_is_rejected() {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/config.json"),
        r#"{"tools":{"tmux":{"enabled":false}}}"#,
    );
    write(
        repo.path().join(".ito/changes/001-29_demo/proposal.md"),
        "## Why\nDemo\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.args(["view", "proposal", "001-29_demo", "--viewer", "tmux-nvim"]);

    command.assert().failure().stderr(predicates::str::contains(
        "tmux is disabled in config (tools.tmux.enabled = false)",
    ));
}

#[test]
fn view_proposal_unknown_viewer_is_rejected() {
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/changes/001-29_demo/proposal.md"),
        "## Why\nDemo\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.args(["view", "proposal", "001-29_demo", "--viewer", "missing"]);

    command
        .assert()
        .failure()
        .stderr(predicates::str::contains("Unknown viewer 'missing'"));
}
