use predicates::str::contains;

#[test]
fn help_prints_usage() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Usage:"));
}

#[test]
fn help_shows_navigation_footer() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("ito help --all"));
}

#[test]
fn agent_instruction_help_shows_instruction_details() {
    // This tests that subcommand help routing works correctly
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.args(["agent", "instruction", "-h"])
        .assert()
        .success()
        // Should show instruction help (with Artifacts section), not agent help
        .stdout(contains("Artifacts:"))
        .stdout(contains("bootstrap"))
        .stdout(contains("apply"))
        .stdout(contains("manifesto"))
        .stdout(contains("--variant"))
        .stdout(contains("--profile"))
        .stdout(contains("--operation"));
}

fn help_output(args: &[&str]) -> String {
    let output = assert_cmd::cargo::cargo_bin_cmd!("ito")
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(output).expect("help output should be UTF-8")
}

#[cfg(not(feature = "backend"))]
#[test]
fn standard_help_hides_backend_task_operations() {
    let output = help_output(&["tasks", "--help"]);
    for operation in ["claim", "release", "allocate", "sync"] {
        assert!(
            !output
                .lines()
                .any(|line| line.trim_start().starts_with(operation)),
            "standard tasks help exposed {operation}:\n{output}"
        );
    }
}

#[cfg(feature = "backend")]
#[test]
fn backend_help_shows_backend_task_operations() {
    let output = help_output(&["tasks", "--help"]);
    for operation in ["claim", "release", "allocate", "sync"] {
        assert!(
            output
                .lines()
                .any(|line| line.trim_start().starts_with(operation)),
            "backend tasks help omitted {operation}:\n{output}"
        );
    }
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn standard_help_hides_coordination_init_and_instruction_entries() {
    let init = help_output(&["init", "--help"]);
    assert!(!init.contains("--setup-coordination-branch"), "{init}");
    assert!(!init.contains("--no-coordination-worktree"), "{init}");

    let instruction = help_output(&["agent", "instruction", "--help"]);
    assert!(!instruction.contains("migrate-to-coordination-worktree"));
    assert!(!instruction.contains("--sync"));
    assert!(instruction.contains("migrate-to-main"));
}

#[cfg(feature = "coordination-branch")]
#[test]
fn coordination_help_shows_coordination_init_and_instruction_entries() {
    let init = help_output(&["init", "--help"]);
    assert!(init.contains("--setup-coordination-branch"), "{init}");
    assert!(init.contains("--no-coordination-worktree"), "{init}");

    let instruction = help_output(&["agent", "instruction", "--help"]);
    assert!(instruction.contains("migrate-to-coordination-worktree"));
    assert!(instruction.contains("--sync"));
}

#[cfg(not(feature = "backend"))]
#[test]
fn standard_agent_instruction_help_hides_backend_guide_entry() {
    let output = help_output(&["agent", "instruction", "--help"]);
    assert!(!output.lines().any(|line| {
        line.trim_start()
            .starts_with("backend                            Backend")
    }));
    assert!(!output.contains("ito agent instruction backend"));
}

#[cfg(feature = "backend")]
#[test]
fn backend_agent_instruction_help_shows_backend_guide_entry() {
    let output = help_output(&["agent", "instruction", "--help"]);
    assert!(output.contains("backend                            Backend"));
    assert!(output.contains("ito agent instruction backend"));
}

#[test]
fn dash_h_help_matches_dash_dash_help() {
    let out_short = assert_cmd::cargo::cargo_bin_cmd!("ito")
        .args(["show", "-h"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let out_long = assert_cmd::cargo::cargo_bin_cmd!("ito")
        .args(["show", "--help"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let short = String::from_utf8_lossy(&out_short).to_string();
    let long = String::from_utf8_lossy(&out_long).to_string();
    assert_eq!(short, long);
}

#[test]
fn help_all_shows_complete_reference() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.args(["help", "--all"])
        .assert()
        .success()
        .stdout(contains("ITO CLI REFERENCE"))
        .stdout(contains("ito init"))
        .stdout(contains("ito list"))
        .stdout(contains("ito agent instruction"));
}

#[test]
fn help_all_global_flag_works() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    cmd.arg("--help-all")
        .assert()
        .success()
        .stdout(contains("ITO CLI REFERENCE"));
}

#[test]
fn help_all_json_outputs_valid_json() {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("ito");
    let output = cmd
        .args(["help", "--all", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("help --all --json should output valid JSON");

    assert!(json.get("version").is_some());
    assert!(json.get("commands").is_some());
    let commands = json.get("commands").unwrap().as_array().unwrap();
    assert!(!commands.is_empty());
}
