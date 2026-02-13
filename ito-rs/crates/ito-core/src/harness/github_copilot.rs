use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `copilot` CLI in non-interactive print mode (`copilot -p`).
/// Selected via `ito ralph --harness copilot`; requires the Copilot CLI on PATH.
#[derive(Debug, Default)]
pub struct GitHubCopilotHarness;

impl Harness for GitHubCopilotHarness {
    /// Identify this harness as the GitHub Copilot harness.
    ///
    /// # Returns
    ///
    /// `HarnessName::GITHUB_COPILOT`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{GitHubCopilotHarness, Harness, HarnessName};
    ///
    /// let h = GitHubCopilotHarness::default();
    /// assert_eq!(h.name(), HarnessName::GITHUB_COPILOT);
    /// ```
    fn name(&self) -> HarnessName {
        HarnessName::GITHUB_COPILOT
    }

    /// Execute the GitHub Copilot CLI using values from the provided run configuration.
    ///
    /// The function builds CLI arguments from `config` (adds `--model <model>` when `config.model` is set,
    /// adds `--yolo` when `config.allow_all` is true, and passes the prompt with `-p <prompt>`),
    /// then delegates execution to the streaming CLI and returns its `HarnessRunResult`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut harness = GitHubCopilotHarness::default();
    /// let config = HarnessRunConfig {
    ///     model: Some("gpt-copilot-1".to_string()),
    ///     allow_all: true,
    ///     prompt: "Fix the failing tests".to_string(),
    ///     ..Default::default()
    /// };
    /// let result = harness.run(&config)?;
    /// ```
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

    /// Stops the harness; a no-op for GitHubCopilotHarness because `run` is synchronous.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{GitHubCopilotHarness, Harness};
    ///
    /// let mut h = GitHubCopilotHarness::default();
    /// h.stop();
    /// ```
    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    /// Indicates whether this harness produces streaming output.
    ///
    /// # Returns
    ///
    /// `true` if the harness produces streaming output, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{GitHubCopilotHarness, Harness};
    ///
    /// let h = GitHubCopilotHarness::default();
    /// assert!(h.streams_output());
    /// ```
    fn streams_output(&self) -> bool {
        true
    }
}
