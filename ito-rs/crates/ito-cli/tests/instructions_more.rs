#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn agent_instruction_proposal_without_change_prints_new_proposal_guide() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("Create a New Proposal"));
    assert!(out.stdout.contains("ito create change"));
    assert!(out.stdout.contains("### Available Modules"));
    let lower = out.stdout.to_lowercase();
    assert!(lower.contains("| 000 |"));
}

#[test]
fn agent_instruction_proposal_without_change_supports_json_output() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "proposal", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["artifactId"], "new-proposal");
    assert!(
        v["instruction"]
            .as_str()
            .unwrap_or_default()
            .contains("Create a New Proposal")
    );
}

#[test]
fn agent_instruction_text_output_renders_artifact_envelope() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("<artifact id=\"proposal\""));
    assert!(out.stdout.contains("Write to:"));
    assert!(out.stdout.contains("<testing_policy>"));
    assert!(out.stdout.contains("- tdd.workflow: red-green-refactor"));
    assert!(out.stdout.contains("RED -> GREEN -> REFACTOR"));
    assert!(out.stdout.contains("- coverage.target_percent: 80"));
}

#[test]
fn agent_instruction_proposal_honors_testing_policy_override() {
    let base = fixtures::make_repo_with_spec_change_fixture();
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    fixtures::reset_repo(repo.path(), base.path());
    fixtures::write(
        repo.path().join(".ito/config.json"),
        "{\"defaults\":{\"testing\":{\"coverage\":{\"target_percent\":93}}}}\n",
    );

    let out = run_rust_candidate(
        rust_path,
        &[
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test-change",
        ],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0);
    assert!(out.stdout.contains("- coverage.target_percent: 93"));
}
