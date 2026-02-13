use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `claude` CLI in non-interactive print mode (`claude -p`).
/// Selected via `ito ralph --harness claude`; requires the Claude Code CLI on PATH.
#[derive(Debug, Default)]
pub struct ClaudeCodeHarness;

impl Harness for ClaudeCodeHarness {
    fn name(&self) -> HarnessName {
        HarnessName::CLAUDE
    }

    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult> {
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

        run_streaming_cli("claude", &args, config)
    }

    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    fn streams_output(&self) -> bool {
        true
    }
}
