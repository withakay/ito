use predicates::str::contains;

#[test]
fn cleanup_instruction_output_contains_manifest_and_legacy_sections() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("cleanup")
        .assert()
        .success()
        .stdout(contains("# Ito Cleanup"))
        .stdout(contains("## Expected Ito-Managed Files"))
        .stdout(contains("## Known Legacy Paths"))
        .stdout(contains("ito-writing-skills"));
}

#[test]
fn cleanup_instruction_json_output_has_correct_artifact_id() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("agent")
        .arg("instruction")
        .arg("cleanup")
        .arg("--json")
        .assert()
        .success()
        .stdout(contains(r#""artifactId": "cleanup""#))
        .stdout(contains("Known Legacy Paths"));
}
