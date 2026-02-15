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
/// assert_eq!(h.name(), HarnessName::Opencode);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct OpencodeHarness;

impl CliHarness for OpencodeHarness {
    fn harness_name(&self) -> HarnessName {
        HarnessName::Opencode
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    enum Allow {
        #[allow(dead_code)]
        All,
        None,
    }

    fn config(allow: Allow, model: Option<&str>) -> HarnessRunConfig {
        let allow_all = match allow {
            Allow::All => true,
            Allow::None => false,
        };
        HarnessRunConfig {
            prompt: "do stuff".to_string(),
            model: model.map(String::from),
            cwd: std::env::temp_dir(),
            env: BTreeMap::new(),
            interactive: false,
            allow_all,
            inactivity_timeout: None,
        }
    }

    #[test]
    fn harness_name_is_opencode() {
        let harness = OpencodeHarness;
        assert_eq!(harness.harness_name(), HarnessName::Opencode);
    }

    #[test]
    fn binary_is_opencode() {
        let harness = OpencodeHarness;
        assert_eq!(harness.binary(), "opencode");
    }

    #[test]
    fn build_args_with_model() {
        let harness = OpencodeHarness;
        let cfg = config(Allow::None, Some("gpt-4"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["run", "-m", "gpt-4", "do stuff"]);
    }

    #[test]
    fn build_args_without_model() {
        let harness = OpencodeHarness;
        let cfg = config(Allow::None, None);
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["run", "do stuff"]);
    }
}
