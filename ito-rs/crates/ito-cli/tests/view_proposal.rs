mod support;

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
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

// --- HTML viewer integration tests ---

#[test]
fn view_proposal_html_viewer_is_recognized() {
    // `--viewer html` should NOT produce "Unknown viewer" — it is a registered backend.
    // It may still fail (pandoc not installed, or not available), but the key assertion
    // is that it is not rejected as an unknown viewer name.
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/changes/001-30_demo/proposal.md"),
        "## Why\nHTML demo\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.args(["view", "proposal", "001-30_demo", "--viewer", "html"]);

    // The command may fail (pandoc not installed) but must never say "Unknown viewer".
    let output = command.output().expect("command ran");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Unknown viewer"),
        "html should be a registered viewer, got stderr: {stderr}"
    );
}

#[test]
fn view_proposal_html_viewer_errors_when_pandoc_missing() {
    // Run with a minimal PATH that excludes pandoc to guarantee the not-found path.
    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/changes/001-30_demo/proposal.md"),
        "## Why\nHTML demo\n",
    );
    write(
        repo.path().join(".ito/changes/001-30_demo/tasks.md"),
        "## Tasks\n- [ ] Verify\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    // Set PATH to an empty directory so pandoc cannot be found.
    let empty_dir = tempfile::tempdir().expect("empty dir");
    command.env("PATH", empty_dir.path());
    command.args(["view", "proposal", "001-30_demo", "--viewer", "html"]);

    command
        .assert()
        .failure()
        .stderr(contains("pandoc").or(contains("unavailable")));
}

/// Create a stub shell script in `dir` that exits with `code`.
#[cfg(unix)]
fn write_stub_script(dir: &std::path::Path, name: &str, code: i32) {
    let path = dir.join(name);
    write(&path, &format!("#!/bin/sh\nexit {code}\n"));
    // Make executable
    std::process::Command::new("chmod")
        .args(["+x", path.to_str().unwrap()])
        .output()
        .expect("chmod failed");
}

#[cfg(unix)]
#[test]
fn view_proposal_html_viewer_succeeds_with_stub_pandoc() {
    // Create a fake PATH with stub pandoc and opener that exit 0.
    // This avoids opening a real browser and doesn't need real pandoc.
    let stub_dir = tempfile::tempdir().expect("stub dir");
    write_stub_script(stub_dir.path(), "pandoc", 0);
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    write_stub_script(stub_dir.path(), opener, 0);

    let repo = tempfile::tempdir().expect("repo");
    write(repo.path().join("README.md"), "# temp\n");
    write(
        repo.path().join(".ito/changes/001-30_demo/proposal.md"),
        "## Why\nHTML demo\n\n## What Changes\n- Test\n",
    );
    write(
        repo.path().join(".ito/changes/001-30_demo/tasks.md"),
        "## Tasks\n- [ ] Verify\n",
    );

    let mut command = Command::cargo_bin("ito").unwrap();
    command.current_dir(repo.path());
    command.env("PATH", stub_dir.path());
    command.args(["view", "proposal", "001-30_demo", "--viewer", "html"]);

    // Both stubs exit 0 so the command should succeed end-to-end.
    command.assert().success();
}
