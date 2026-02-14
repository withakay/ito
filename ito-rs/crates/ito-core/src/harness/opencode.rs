use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Harness implementation that executes the `opencode` CLI (`opencode run`).
///
/// Selected via `ito ralph --harness opencode`; requires the OpenCode CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{Harness, HarnessName, OpencodeHarness};
///
/// let h = OpencodeHarness;
/// assert_eq!(h.name(), HarnessName::OPENCODE);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct OpencodeHarness;

impl CliHarness for OpencodeHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::OPENCODE
    }

    fn binary(&self) -> &str {
        "opencode"
    }

    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
        let mut args = vec!["run".to_string()];
        if let Some(model) = config.model.as_deref() {
            args.push("-m".to_string());
            args.push(model.to_string());
        }
        args.push(config.prompt.clone());
        args
    }
}
