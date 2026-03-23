use predicates::str::contains;

#[test]
fn repo_sweep_succeeds_without_change_flag() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("repo-sweep")
        .assert()
        .success();
}

#[test]
fn repo_sweep_output_contains_key_phrases() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("repo-sweep")
        .assert()
        .success()
        .stdout(contains("Sub-Module"))
        .stdout(contains("NNN.SS-NN_name"));
}

#[test]
fn repo_sweep_json_output_has_correct_artifact_id() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("repo-sweep")
        .arg("--json")
        .assert()
        .success()
        .stdout(contains(r#""artifactId": "repo-sweep""#))
        .stdout(contains(r#""instruction":"#));
}
