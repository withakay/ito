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
fn harness_name_is_codex() {
    let harness = CodexHarness;
    assert_eq!(harness.harness_name(), HarnessName::Codex);
}

#[test]
fn binary_is_codex() {
    let harness = CodexHarness;
    assert_eq!(harness.binary(), "codex");
}

#[test]
fn build_args_with_allow_all() {
    let harness = CodexHarness;
    let cfg = config(Allow::All, Some("o3"));
    let args = harness.build_args(&cfg);
    assert_eq!(args, vec!["exec", "--model", "o3", "--yolo", "do stuff"]);
}

#[test]
fn build_args_without_allow_all() {
    let harness = CodexHarness;
    let cfg = config(Allow::None, Some("o3"));
    let args = harness.build_args(&cfg);
    assert_eq!(args, vec!["exec", "--model", "o3", "do stuff"]);
}
