use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

#[derive(Debug, Default)]
/// Harness implementation that executes the `opencode` CLI.
pub struct OpencodeHarness;

impl Harness for OpencodeHarness {
    /// The harness identifier for this implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = OpencodeHarness::default();
    /// assert_eq!(h.name(), HarnessName::OPENCODE);
    /// ```
    fn name(&self) -> HarnessName {
        HarnessName::OPENCODE
    }

    /// Run the opencode CLI with the provided harness configuration and stream its output.
    ///
    /// The command invoked is `opencode run` with an optional `-m <model>` flag when `config.model` is set,
    /// followed by the prompt from `config.prompt`.
    ///
    /// # Returns
    ///
    /// A `HarnessRunResult` describing the outcome of the CLI run.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut harness = OpencodeHarness::default();
    /// let config = HarnessRunConfig { prompt: "Generate a Rust function".into(), model: None, ..Default::default() };
    /// let result = harness.run(&config).unwrap();
    /// // Inspect `result` for run details
    /// ```
    fn run(&mut self, config: &HarnessRunConfig) -> Result<HarnessRunResult> {
        let mut args = vec!["run".to_string()];
        if let Some(model) = config.model.as_deref() {
            args.push("-m".to_string());
            args.push(model.to_string());
        }
        args.push(config.prompt.clone());

        run_streaming_cli("opencode", &args, config)
    }

    /// Stops the harness. This implementation is a no-op because `run` completes synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::harness::opencode::OpencodeHarness;
    ///
    /// let mut h = OpencodeHarness::default();
    /// h.stop();
    /// ```
    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    /// Indicates whether the harness produces streaming output.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = OpencodeHarness::default();
    /// assert!(h.streams_output());
    /// ```
    fn streams_output(&self) -> bool {
        true
    }
}
