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
/// assert_eq!(h.name(), HarnessName::CLAUDE);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct ClaudeCodeHarness;

impl CliHarness for ClaudeCodeHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::CLAUDE
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
