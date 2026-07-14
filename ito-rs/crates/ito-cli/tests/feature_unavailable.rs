use assert_cmd::Command;
#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
use ito_test_support::collect_file_bytes;
#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
use serde_json::Value;
#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
use std::path::Path;

fn ito() -> Command {
    Command::cargo_bin("ito").expect("ito binary")
}

#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
fn isolated_ito(project: &Path, home: &Path) -> Command {
    let mut command = ito();
    command
        .current_dir(project)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join(".config"))
        .env("XDG_DATA_HOME", home);
    command
}

#[cfg(any(not(feature = "backend"), not(feature = "coordination-branch")))]
fn assert_feature_json(output: &[u8], feature: &str, requested_by: &str) {
    let value: Value = serde_json::from_slice(output).expect("valid JSON error");
    assert_eq!(value["error"]["kind"], "feature_unavailable");
    assert_eq!(value["error"]["feature"], feature);
    assert_eq!(value["error"]["requested_by"], requested_by);
    assert!(
        value["error"]["recovery"]
            .as_str()
            .is_some_and(|text| !text.is_empty())
    );
}

#[cfg(not(feature = "backend"))]
#[test]
fn backend_compatibility_command_returns_structured_error() {
    let output = ito()
        .args(["backend", "status", "--json"])
        .output()
        .expect("run ito");
    assert!(!output.status.success());
    assert_feature_json(&output.stdout, "backend", "ito backend");
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn coordination_compatibility_commands_return_structured_errors() {
    let output = ito().args(["sync", "--json"]).output().expect("run ito");
    assert!(!output.status.success());
    assert_feature_json(&output.stdout, "coordination-branch", "ito sync");
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn coordination_requests_fail_before_logging_or_project_mutation() {
    let project = tempfile::tempdir().expect("temp project");
    let home = tempfile::tempdir().expect("temp home");
    let ito_dir = project.path().join(".ito");
    std::fs::create_dir(&ito_dir).expect("create .ito");
    std::fs::write(
        ito_dir.join("config.json"),
        r#"{ "changes": { "coordination_branch": { "enabled": false, "storage": "embedded" } } }"#,
    )
    .expect("write config");
    let project_before = collect_file_bytes(project.path());
    let home_before = collect_file_bytes(home.path());

    for (args, requested_by) in [
        (&["sync", "--json"][..], "ito sync"),
        (
            &[
                "agent",
                "instruction",
                "apply",
                "--change",
                "000-01_missing",
                "--sync",
                "--json",
            ][..],
            "ito agent instruction apply --sync",
        ),
    ] {
        let output = isolated_ito(project.path(), home.path())
            .args(args)
            .output()
            .expect("run ito");

        assert!(!output.status.success(), "args={args:?}");
        assert_feature_json(&output.stdout, "coordination-branch", requested_by);
        assert_eq!(collect_file_bytes(project.path()), project_before);
        assert_eq!(collect_file_bytes(home.path()), home_before);
    }
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn init_coordination_request_fails_before_initialization() {
    let project = tempfile::tempdir().expect("temp project");
    let home = tempfile::tempdir().expect("temp home");
    let project_before = collect_file_bytes(project.path());
    let home_before = collect_file_bytes(home.path());

    let output = isolated_ito(project.path(), home.path())
        .args([
            "init",
            ".",
            "--tools",
            "none",
            "--setup-coordination-branch",
        ])
        .output()
        .expect("run ito");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("feature 'coordination-branch' is unavailable"));
    assert!(stderr.contains("ito init --setup-coordination-branch"));
    assert_eq!(collect_file_bytes(project.path()), project_before);
    assert_eq!(collect_file_bytes(home.path()), home_before);
}

#[cfg(not(feature = "backend"))]
#[test]
fn backend_task_compatibility_command_returns_structured_error() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["tasks", "claim", "001-01_example", "--json"],
            "ito tasks claim",
        ),
        (
            &["tasks", "release", "001-01_example", "--json"],
            "ito tasks release",
        ),
        (&["tasks", "allocate", "--json"], "ito tasks allocate"),
        (
            &["tasks", "sync", "pull", "001-01_example", "--json"],
            "ito tasks sync",
        ),
    ];

    for (args, requested_by) in cases {
        let output = ito().args(*args).output().expect("run ito");
        assert!(!output.status.success(), "args={args:?}");
        assert_feature_json(&output.stdout, "backend", requested_by);
    }
}

#[cfg(not(feature = "backend"))]
#[test]
fn backend_task_request_fails_before_logging_or_project_mutation() {
    let project = tempfile::tempdir().expect("temp project");
    let home = tempfile::tempdir().expect("temp home");
    let ito_dir = project.path().join(".ito");
    std::fs::create_dir(&ito_dir).expect("create .ito");
    std::fs::write(ito_dir.join("config.json"), "{}\n").expect("write config");
    let project_before = collect_file_bytes(project.path());
    let home_before = collect_file_bytes(home.path());

    let output = isolated_ito(project.path(), home.path())
        .args(["tasks", "claim", "000-01_missing", "--json"])
        .output()
        .expect("run ito");

    assert!(!output.status.success());
    assert_feature_json(&output.stdout, "backend", "ito tasks claim");
    assert_eq!(collect_file_bytes(project.path()), project_before);
    assert_eq!(collect_file_bytes(home.path()), home_before);
}

#[cfg(not(feature = "backend"))]
#[test]
fn backend_config_is_rejected_before_fallback_mutation() {
    let project = tempfile::tempdir().expect("temp project");
    let ito_dir = project.path().join(".ito");
    std::fs::create_dir(&ito_dir).expect("create .ito");
    std::fs::write(
        ito_dir.join("config.json"),
        r#"{
          "backend": { "enabled": true },
          "changes": {
            "coordination_branch": { "enabled": false, "storage": "embedded" }
          }
        }"#,
    )
    .expect("write config");

    let output = ito()
        .current_dir(project.path())
        .args(["list", "--json"])
        .output()
        .expect("run ito");

    assert!(!output.status.success());
    assert_feature_json(&output.stdout, "backend", "backend.enabled");
    assert_eq!(
        std::fs::read_dir(&ito_dir)
            .expect("read .ito")
            .map(|entry| entry.expect("entry").file_name())
            .collect::<Vec<_>>(),
        vec![std::ffi::OsString::from("config.json")]
    );
}

#[cfg(not(feature = "coordination-branch"))]
#[test]
fn either_legacy_coordination_signal_is_rejected_before_mutation() {
    for coordination in [
        r#"{ "enabled": true, "storage": "embedded" }"#,
        r#"{ "enabled": false, "storage": "worktree" }"#,
    ] {
        let project = tempfile::tempdir().expect("temp project");
        let ito_dir = project.path().join(".ito");
        std::fs::create_dir(&ito_dir).expect("create .ito");
        std::fs::write(
            ito_dir.join("config.json"),
            format!(
                r#"{{
                  "backend": {{ "enabled": false }},
                  "changes": {{ "coordination_branch": {coordination} }}
                }}"#
            ),
        )
        .expect("write config");

        let output = ito()
            .current_dir(project.path())
            .args(["list", "--json"])
            .output()
            .expect("run ito");
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).expect("valid JSON error");
        assert_eq!(value["error"]["feature"], "coordination-branch");
        assert_eq!(std::fs::read_dir(&ito_dir).expect("read .ito").count(), 1);
    }
}

#[test]
fn invalid_experimental_config_remains_a_configuration_error() {
    let project = tempfile::tempdir().expect("temp project");
    let ito_dir = project.path().join(".ito");
    std::fs::create_dir(&ito_dir).expect("create .ito");
    std::fs::write(
        ito_dir.join("config.json"),
        r#"{
          "backend": { "enabled": false },
          "changes": {
            "coordination_branch": { "enabled": false, "storage": "unknown" }
          }
        }"#,
    )
    .expect("write config");

    let output = ito()
        .current_dir(project.path())
        .args(["list", "--json"])
        .output()
        .expect("run ito");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown variant `unknown`"),
        "stderr={stderr}"
    );
    assert!(!stderr.contains("feature unavailable"), "stderr={stderr}");
    assert_eq!(std::fs::read_dir(&ito_dir).expect("read .ito").count(), 1);
}
