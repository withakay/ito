use super::*;
use std::collections::BTreeMap;

enum Allow {
    All,
    None,
}

fn config(allow: Allow, model: Option<&str>) -> HarnessRunConfig {
    let allow_all = match allow {
        Allow::All => true,
        Allow::None => false,
    };
    HarnessRunConfig {
        prompt: "do stuff".to_string(),
        model: model.map(String::from),
        cwd: std::env::temp_dir(),
        env: BTreeMap::new(),
        interactive: false,
        allow_all,
        inactivity_timeout: None,
    }
}

#[test]
fn harness_name_is_claude() {
    let harness = ClaudeCodeHarness;
    assert_eq!(harness.harness_name(), HarnessName::Claude);
}

#[test]
fn binary_is_claude() {
    let harness = ClaudeCodeHarness;
    assert_eq!(harness.binary(), "claude");
}

#[test]
fn build_args_with_allow_all() {
    let harness = ClaudeCodeHarness;
    let cfg = config(Allow::All, Some("sonnet"));
    let args = harness.build_args(&cfg);
    assert_eq!(
        args,
        vec![
            "--model",
            "sonnet",
            "--dangerously-skip-permissions",
            "-p",
            "do stuff"
        ]
    );
}

#[test]
fn build_args_without_allow_all() {
    let harness = ClaudeCodeHarness;
    let cfg = config(Allow::None, Some("sonnet"));
    let args = harness.build_args(&cfg);
    assert_eq!(args, vec!["--model", "sonnet", "-p", "do stuff"]);
}

#[test]
fn build_args_without_model() {
    let harness = ClaudeCodeHarness;
    let cfg = config(Allow::None, None);
    let args = harness.build_args(&cfg);
    assert_eq!(args, vec!["-p", "do stuff"]);
}
