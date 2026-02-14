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
/// assert_eq!(h.name(), HarnessName::CODEX);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct CodexHarness;

impl CliHarness for CodexHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::CODEX
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
