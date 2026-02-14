use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Runs the `copilot` CLI in non-interactive print mode (`copilot -p`).
///
/// Selected via `ito ralph --harness copilot`; requires the Copilot CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{GitHubCopilotHarness, Harness, HarnessName};
///
/// let h = GitHubCopilotHarness;
/// assert_eq!(h.name(), HarnessName::GithubCopilot);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct GitHubCopilotHarness;

impl CliHarness for GitHubCopilotHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::GithubCopilot
    }

    fn binary(&self) -> &str {
        "copilot"
    }

    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
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
        args
    }
}
