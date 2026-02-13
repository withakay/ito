use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `claude` CLI in non-interactive print mode (`claude -p`).
/// Selected via `ito ralph --harness claude`; requires the Claude Code CLI on PATH.
#[derive(Debug, Default)]
pub struct ClaudeCodeHarness;

impl Harness for ClaudeCodeHarness {
    /// Identify the harness as the Claude harness.
    ///
    /// # Returns
    ///
    /// `HarnessName::CLAUDE`
    ///
    /// # Examples
    ///
    /// ```
    /// let h = ClaudeCodeHarness::default();
    /// assert_eq!(h.name(), HarnessName::CLAUDE);
    /// ```
    fn name(&self) -> HarnessName {
        HarnessName::CLAUDE
    }

    /// Run the Claude CLI using the provided configuration and stream its output.
    ///
    /// # Returns
    ///
    /// A `HarnessRunResult` describing the outcome of the run.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ito_core::harness::{ClaudeCodeHarness, HarnessRunConfig};
    ///
    /// let mut harness = ClaudeCodeHarness::default();
    /// let config = HarnessRunConfig {
    ///     prompt: "Translate to French: Hello".to_string(),
    ///     ..Default::default()
    /// };
    /// let result = harness.run(&config).unwrap();
    /// ```
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

    /// Stops the harness. This implementation is a no-op because `run` is synchronous.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut h = ClaudeCodeHarness::default();
    /// // Calling stop has no effect for this harness.
    /// h.stop();
    /// ```
    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    /// Indicates whether this harness produces streaming output.
    
    ///
    
    /// Returns `true` if the harness produces streaming output, `false` otherwise.
    
    /// For this harness, the method always returns `true`.
    
    ///
    
    /// # Examples
    
    ///
    
    /// ```
    
    /// let h = ClaudeCodeHarness::default();
    
    /// assert!(h.streams_output());
    
    /// ```
    fn streams_output(&self) -> bool {
        true
    }
}