#[path = "support/mod.rs"]
mod fixtures;

use ito_test_support::run_rust_candidate;

#[test]
fn agent_instruction_context_supports_json_output() {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    std::fs::write(repo.path().join("README.md"), "temp\n").expect("write");
    fixtures::git_init_with_initial_commit(repo.path());
    fixtures::write(repo.path().join(".ito/config.json"), "{}\n");

    let target = "023-07_harness-context-inference";
    let ok = std::process::Command::new("git")
        .args(["checkout", "-b", target])
        .current_dir(repo.path())
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .status()
        .expect("git")
        .success();
    assert!(ok, "git checkout should succeed");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "context", "--json"],
        repo.path(),
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["target"]["kind"], "change");
    assert_eq!(v["target"]["id"], target);
    assert!(
        v["nudge"]
            .as_str()
            .unwrap_or_default()
            .contains("ito tasks next"),
        "nudge={}",
        v["nudge"].as_str().unwrap_or_default()
    );
}

#[test]
fn agent_instruction_context_prefers_path_inference_in_text_output() {
    let repo = tempfile::tempdir().expect("work");
    let home = tempfile::tempdir().expect("home");
    let rust_path = assert_cmd::cargo::cargo_bin!("ito");

    std::fs::write(repo.path().join("README.md"), "temp\n").expect("write");
    fixtures::git_init_with_initial_commit(repo.path());
    fixtures::write(repo.path().join(".ito/config.json"), "{}\n");

    let cwd = repo.path().join("ito-worktrees").join("002-01_path-change");
    std::fs::create_dir_all(&cwd).expect("mkdir");

    let out = run_rust_candidate(
        rust_path,
        &["agent", "instruction", "context"],
        &cwd,
        home.path(),
    );

    assert_eq!(out.code, 0, "stderr={}", out.stderr);
    assert!(
        out.stdout
            .contains("[Ito Target] change 002-01_path-change")
    );
    assert!(out.stdout.contains("ito tasks next 002-01_path-change"));
}
