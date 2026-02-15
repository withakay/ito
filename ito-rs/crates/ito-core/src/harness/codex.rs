use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Runs the `codex` CLI in non-interactive exec mode (`codex exec`).
///
/// Selected via `ito ralph --harness codex`; requires the Codex CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{CodexHarness, Harness, HarnessName};
///
/// let h = CodexHarness;
/// assert_eq!(h.name(), HarnessName::Codex);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct CodexHarness;

impl CliHarness for CodexHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::Codex
    }

    fn binary(&self) -> &str {
        "codex"
    }

    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
        let mut args = vec!["exec".to_string()];
        if let Some(model) = config.model.as_deref() {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if config.allow_all {
            args.push("--yolo".to_string());
        }
        args.push(config.prompt.clone());
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn config(allow_all: bool, model: Option<&str>) -> HarnessRunConfig {
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
        let cfg = config(true, Some("o3"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["exec", "--model", "o3", "--yolo", "do stuff"]);
    }

    #[test]
    fn build_args_without_allow_all() {
        let harness = CodexHarness;
        let cfg = config(false, Some("o3"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["exec", "--model", "o3", "do stuff"]);
    }
}
