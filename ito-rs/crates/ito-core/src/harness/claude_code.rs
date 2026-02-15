use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Runs the `claude` CLI in non-interactive print mode (`claude -p`).
///
/// Selected via `ito ralph --harness claude`; requires the Claude Code CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{ClaudeCodeHarness, Harness, HarnessName};
///
/// let h = ClaudeCodeHarness;
/// assert_eq!(h.name(), HarnessName::Claude);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct ClaudeCodeHarness;

impl CliHarness for ClaudeCodeHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::Claude
    }

    fn binary(&self) -> &str {
        "claude"
    }

    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(model) = config.model.as_deref() {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if config.allow_all {
            args.push("--dangerously-skip-permissions".to_string());
        }
        args.push("-p".to_string());
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
        let cfg = config(true, Some("sonnet"));
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
        let cfg = config(false, Some("sonnet"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "sonnet", "-p", "do stuff"]);
    }

    #[test]
    fn build_args_without_model() {
        let harness = ClaudeCodeHarness;
        let cfg = config(false, None);
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["-p", "do stuff"]);
    }
}
