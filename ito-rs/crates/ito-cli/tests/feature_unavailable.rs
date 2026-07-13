use assert_cmd::Command;
use serde_json::Value;

fn ito() -> Command {
    Command::cargo_bin("ito").expect("ito binary")
}

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

#[cfg(not(feature = "backend"))]
#[test]
fn backend_task_compatibility_command_returns_structured_error() {
    let output = ito()
        .args(["tasks", "claim", "001-01_example", "--json"])
        .output()
        .expect("run ito");
    assert!(!output.status.success());
    assert_feature_json(&output.stdout, "backend", "ito tasks claim");
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
