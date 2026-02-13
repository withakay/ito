use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `codex` CLI in non-interactive exec mode (`codex exec`).
/// Selected via `ito ralph --harness codex`; requires the Codex CLI on PATH.
#[derive(Debug, Default)]
pub struct CodexHarness;

impl Harness for CodexHarness {
    fn name(&self) -> HarnessName {
        HarnessName::CODEX
    }

    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult> {
        let mut args = vec!["exec".to_string()];
        if let Some(model) = config.model.as_deref() {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if config.allow_all {
            args.push("--yolo".to_string());
        }
        args.push(config.prompt.clone());

        run_streaming_cli("codex", &args, config)
    }

    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    fn streams_output(&self) -> bool {
        true
    }
}
