use predicates::str::contains;

#[test]
fn worktrees_instruction_does_not_require_change() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("worktrees")
        .assert()
        .success()
        .stdout(contains("Resolved Configuration"))
        .stdout(contains("Config Files Consulted"))
        .stdout(contains("config.local.json"));
}

#[test]
fn worktrees_instruction_json_output() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("worktrees")
        .arg("--json")
        .assert()
        .success()
        .stdout(contains(r#""artifactId": "worktrees""#))
        .stdout(contains(r#""instruction":"#));
}

#[test]
fn workflow_is_an_alias_for_worktrees() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("workflow")
        .assert()
        .success()
        .stdout(contains("Resolved Configuration"));
}
