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
    /// Return the `HarnessName` variant representing this harness.
    ///
    /// Returns the `HarnessName::Opencode` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = OpencodeHarness::default();
    /// assert_eq!(h.harness_name(), HarnessName::Opencode);
    /// ```
    fn harness_name(&self) -> HarnessName {
        HarnessName::Opencode
    }

    /// Returns the name of the CLI binary used by this harness.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = OpencodeHarness::default();
    /// assert_eq!(h.binary(), "opencode");
    /// ```
    fn binary(&self) -> &str {
        "opencode"
    }

    /// Build the command-line arguments for invoking `opencode run` from a run configuration.
    ///
    /// The resulting vector always starts with `"run"`. If `config.model` is `Some`, `"-m"`
    /// and the model name are inserted before the prompt. The prompt from `config.prompt` is
    /// appended as the final argument.
    ///
    /// # Parameters
    ///
    /// - `config`: run configuration whose `model` (optional model name) and `prompt` (prompt string)
    ///   control the generated arguments.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the `opencode` subcommand and its arguments (e.g. `["run", "-m", "gpt-4", "do stuff"]`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use std::env;
    /// # use crate::{OpencodeHarness, HarnessRunConfig};
    /// let harness = OpencodeHarness::default();
    /// let cfg = HarnessRunConfig {
    ///     prompt: "do stuff".to_string(),
    ///     model: Some("gpt-4".to_string()),
    ///     cwd: env::temp_dir(),
    ///     env: BTreeMap::new(),
    ///     interactive: false,
    ///     allow_all: false,
    ///     inactivity_timeout: None,
    /// };
    /// let args = harness.build_args(&cfg);
    /// assert_eq!(args, vec!["run", "-m", "gpt-4", "do stuff"]);
    /// ```
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

    /// Create a test HarnessRunConfig with a fixed prompt ("do stuff") and a temporary working directory.
    ///
    /// The `allow` parameter controls the `allow_all` flag in the returned config. The `model` parameter,
    /// if present, is stored as the config's optional model string. Other fields are set to sensible
    /// defaults for tests (empty env, non-interactive, no inactivity timeout).
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = config(Allow::None, Some("gpt-4"));
    /// assert_eq!(cfg.prompt, "do stuff");
    /// assert_eq!(cfg.model.as_deref(), Some("gpt-4"));
    /// assert!(!cfg.allow_all);
    /// assert!(!cfg.interactive);
    /// assert!(cfg.inactivity_timeout.is_none());
    /// ```
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