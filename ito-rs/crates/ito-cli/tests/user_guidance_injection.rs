use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
use std::fs;

#[test]
fn agent_instruction_includes_user_guidance_when_present() {
    let tmp = tempfile::tempdir().expect("tempdir should succeed");
    let root = tmp.path();

    fs::create_dir_all(root.join(".ito/changes/000-01_test"))
        .expect("create change dir should succeed");

    let guidance = "<!-- ITO:START -->\nheader\n<!-- ITO:END -->\n\nPrefer TDD.\n";
    fs::create_dir_all(root.join(".ito")).expect("create .ito should succeed");
    fs::write(root.join(".ito/user-guidance.md"), guidance)
        .expect("write guidance file should succeed");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.current_dir(root)
        .args([
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test",
        ])
        .assert()
        .success()
        .stdout(contains("<user_guidance>").and(contains("Prefer TDD.")));
}

#[test]
fn agent_instruction_includes_scoped_user_prompt_for_artifact() {
    let tmp = tempfile::tempdir().expect("tempdir should succeed");
    let root = tmp.path();

    fs::create_dir_all(root.join(".ito/changes/000-01_test"))
        .expect("create change dir should succeed");
    fs::create_dir_all(root.join(".ito/user-prompts")).expect("create user-prompts should succeed");
    fs::write(
        root.join(".ito/user-prompts/proposal.md"),
        "Proposal-only guidance.",
    )
    .expect("write scoped guidance should succeed");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.current_dir(root)
        .args([
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test",
        ])
        .assert()
        .success()
        .stdout(contains("Proposal-only guidance."));
}

#[test]
fn agent_instruction_prefers_user_prompts_shared_guidance_file() {
    let tmp = tempfile::tempdir().expect("tempdir should succeed");
    let root = tmp.path();

    fs::create_dir_all(root.join(".ito/changes/000-01_test"))
        .expect("create change dir should succeed");
    fs::create_dir_all(root.join(".ito/user-prompts")).expect("create user-prompts should succeed");
    fs::write(
        root.join(".ito/user-guidance.md"),
        "Legacy shared guidance.",
    )
    .expect("write legacy guidance should succeed");
    fs::write(
        root.join(".ito/user-prompts/guidance.md"),
        "New shared guidance.",
    )
    .expect("write shared guidance should succeed");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.current_dir(root)
        .args([
            "agent",
            "instruction",
            "proposal",
            "--change",
            "000-01_test",
        ])
        .assert()
        .success()
        .stdout(contains("New shared guidance."))
        .stdout(predicates::str::contains("Legacy shared guidance.").not());
}
