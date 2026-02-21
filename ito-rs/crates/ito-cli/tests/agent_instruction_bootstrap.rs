use predicates::str::contains;

#[test]
fn bootstrap_requires_tool_flag() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .assert()
        .failure()
        .stderr(contains("Missing required option --tool"));
}

#[test]
fn bootstrap_rejects_invalid_tool() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(contains("Invalid tool 'invalid'"))
        .stderr(contains(
            "Valid tools: opencode, claude, codex, github-copilot",
        ));
}

#[test]
fn bootstrap_opencode_success() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("opencode")
        .assert()
        .success()
        .stdout(contains("Ito Bootstrap Instructions"))
        .stdout(contains("Tool Notes"))
        .stdout(contains("dedicated tools"))
        .stdout(contains(".opencode/skills/"))
        .stdout(contains("ito agent instruction proposal"))
        .stdout(contains("ito agent instruction apply"))
        .stdout(contains("ito agent instruction review"))
        .stdout(contains("ito agent instruction archive"));
}

#[test]
fn bootstrap_claude_success() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("claude")
        .assert()
        .success()
        .stdout(contains("Ito Bootstrap Instructions"))
        .stdout(contains("Tool Notes"))
        .stdout(contains("dedicated tools"))
        .stdout(contains(".claude/skills/"))
        .stdout(contains("ito agent instruction proposal"))
        .stdout(contains("ito agent instruction apply"))
        .stdout(contains("ito agent instruction review"))
        .stdout(contains("ito agent instruction archive"));
}

#[test]
fn bootstrap_codex_success() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("codex")
        .assert()
        .success()
        .stdout(contains("Ito Bootstrap Instructions"))
        .stdout(contains("Tool Notes"))
        .stdout(contains("Shell-first"))
        .stdout(contains("ito agent instruction proposal"))
        .stdout(contains("ito agent instruction apply"))
        .stdout(contains("ito agent instruction review"))
        .stdout(contains("ito agent instruction archive"));
}

#[test]
fn bootstrap_github_copilot_success() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("github-copilot")
        .assert()
        .success()
        .stdout(contains("Ito Bootstrap Instructions"))
        .stdout(contains("Tool Notes"))
        .stdout(contains("GitHub Copilot"))
        .stdout(contains("copilot-instructions.md"))
        .stdout(contains("ito agent instruction proposal"))
        .stdout(contains("ito agent instruction apply"))
        .stdout(contains("ito agent instruction review"))
        .stdout(contains("ito agent instruction archive"));
}

#[test]
fn bootstrap_json_output() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("opencode")
        .arg("--json")
        .assert()
        .success()
        .stdout(contains(r#""artifactId": "bootstrap""#))
        .stdout(contains(r#""instruction":"#));
}

#[test]
fn bootstrap_output_is_short() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    let output = cmd
        .arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("opencode")
        .output()
        .expect("command should execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();

    assert!(
        line_count < 100,
        "Bootstrap output should be short (< 100 lines), got {} lines",
        line_count
    );
}

#[test]
fn bootstrap_contains_artifact_pointers() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("bootstrap")
        .arg("--tool")
        .arg("opencode")
        .assert()
        .success()
        .stdout(contains("proposal"))
        .stdout(contains("specs"))
        .stdout(contains("tasks"))
        .stdout(contains("apply"))
        .stdout(contains("review"))
        .stdout(contains("archive"));
}
