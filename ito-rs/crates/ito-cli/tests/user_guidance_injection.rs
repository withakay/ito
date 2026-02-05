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
