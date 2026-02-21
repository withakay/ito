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

/// Verifies that `ito agent instruction bootstrap --tool <name>` fails when given an invalid tool and reports the invalid value plus the allowed tools.
///
/// The command is expected to exit with a failure status and write an error to stderr containing the invalid tool name and the list of valid tools.
///
/// # Examples
///
/// ```no_run
/// use assert_cmd::prelude::*;
/// let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// cmd.arg("agent")
///     .arg("instruction")
///     .arg("bootstrap")
///     .arg("--tool")
///     .arg("invalid")
///     .assert()
///     .failure()
///     .stderr(predicates::str::contains("Invalid tool 'invalid'"))
///     .stderr(predicates::str::contains("Valid tools: opencode, claude, codex, github-copilot"));
/// ```
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

/// Verifies that `ito agent instruction bootstrap --tool claude` succeeds and emits Claude-specific bootstrap instructions.
///
/// Asserts the command exits successfully and stdout includes the generic bootstrap headings, references to tool notes and dedicated tools, the `.claude/skills/` path, and the expected instruction targets: proposal, apply, review, and archive.
///
/// # Examples
///
/// ```
/// let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// cmd.arg("agent")
///    .arg("instruction")
///    .arg("bootstrap")
///    .arg("--tool")
///    .arg("claude")
///    .assert()
///    .success();
/// ```
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

/// Verifies that `ito agent instruction bootstrap --tool github-copilot` succeeds and emits GitHub Copilot-specific bootstrap instructions.
///
/// The test asserts the command exits successfully and that stdout contains the generic bootstrap headings and GitHub Copilot-specific markers, including a reference to `copilot-instructions.md` and the expected instruction subcommands (`proposal`, `apply`, `review`, `archive`).
///
/// # Examples
///
/// ```
/// let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
/// cmd.arg("agent")
///     .arg("instruction")
///     .arg("bootstrap")
///     .arg("--tool")
///     .arg("github-copilot")
///     .assert()
///     .success()
///     .stdout(predicates::str::contains("Ito Bootstrap Instructions"))
///     .stdout(predicates::str::contains("Tool Notes"))
///     .stdout(predicates::str::contains("GitHub Copilot"))
///     .stdout(predicates::str::contains("copilot-instructions.md"))
///     .stdout(predicates::str::contains("ito agent instruction proposal"))
///     .stdout(predicates::str::contains("ito agent instruction apply"))
///     .stdout(predicates::str::contains("ito agent instruction review"))
///     .stdout(predicates::str::contains("ito agent instruction archive"));
/// ```
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
