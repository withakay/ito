use super::streaming_cli::run_streaming_cli;
use super::types::{Harness, HarnessName, HarnessRunConfig, HarnessRunResult};
use miette::Result;

/// Runs the `codex` CLI in non-interactive exec mode (`codex exec`).
/// Selected via `ito ralph --harness codex`; requires the Codex CLI on PATH.
#[derive(Debug, Default)]
pub struct CodexHarness;

impl Harness for CodexHarness {
    /// Identify this harness implementation as the Codex harness.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{CodexHarness, Harness, HarnessName};
    ///
    /// let h = CodexHarness::default();
    /// assert_eq!(h.name(), HarnessName::CODEX);
    /// ```
    fn name(&self) -> HarnessName {
        HarnessName::CODEX
    }

    /// Execute the `codex` CLI with arguments derived from `config` and stream its output.
    ///
    /// The constructed command begins with `"exec"`, includes `--model <model>` if `config.model` is set,
    /// appends `--yolo` when `config.allow_all` is true, and appends `config.prompt` as the final argument.
    ///
    /// # Returns
    ///
    /// The resulting `HarnessRunResult` on success.
    ///
    /// # Examples
    ///
    ///
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

    /// Performs no action; provided to satisfy the `Harness` trait.
    ///
    /// This method is intentionally empty because `run` executes synchronously and there is nothing to stop.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{CodexHarness, Harness};
    ///
    /// let mut h = CodexHarness::default();
    /// h.stop();
    /// ```
    fn stop(&mut self) {
        // No-op: `run` is synchronous.
    }

    /// Indicates whether this harness emits output incrementally (streaming) during a run.
    ///
    /// # Returns
    ///
    /// `true` if the harness streams output as it runs, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use ito_core::harness::{CodexHarness, Harness};
    ///
    /// let h = CodexHarness::default();
    /// assert!(h.streams_output());
    /// ```
    fn streams_output(&self) -> bool {
        true
    }
}
