use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `copilot` CLI in non-interactive print mode (`copilot -p`).
/// Selected via `ito ralph --harness copilot`; requires the Copilot CLI on PATH.
#[derive(Debug, Default)]
pub struct GitHubCopilotHarness;

impl Harness for GitHubCopilotHarness {
    fn name(&self) -> HarnessName {
        HarnessName::GITHUB_COPILOT
    }

    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult> {
        let mut args = Vec::new();
        if let Some(model) = config.model.as_deref() {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if config.allow_all {
            args.push("--yolo".to_string());
        }
        args.push("-p".to_string());
        args.push(config.prompt.clone());

        run_streaming_cli("copilot", &args, config)
    }

    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    fn streams_output(&self) -> bool {
        true
    }
}
