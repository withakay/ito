use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Runs the `codex` CLI in non-interactive exec mode (`codex exec`).
///
/// Selected via `ito ralph --harness codex`; requires the Codex CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{CodexHarness, Harness, HarnessName};
///
/// let h = CodexHarness;
/// assert_eq!(h.name(), HarnessName::Codex);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct CodexHarness;

impl CliHarness for CodexHarness {
    /// Identifies this harness as the Codex harness.
    ///
    /// # Returns
    ///
    /// `HarnessName::Codex` representing the Codex harness.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = CodexHarness::default();
    /// assert_eq!(h.harness_name(), HarnessName::Codex);
    /// ```
    fn harness_name(&self) -> HarnessName {
        HarnessName::Codex
    }

    /// Executable name for the Codex CLI.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = CodexHarness::default();
    /// assert_eq!(h.binary(), "codex");
    /// ```
    fn binary(&self) -> &str {
        "codex"
    }

    /// Build the command-line arguments for running the Codex CLI in non-interactive exec mode.
    ///
    /// The returned Vec contains the subcommand "exec", optional `--model <name>` when
    /// `config.model` is set, the `--yolo` flag when `config.allow_all` is true, and the
    /// prompt as the final argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use std::path::PathBuf;
    ///
    /// let harness = CodexHarness::default();
    /// let config = HarnessRunConfig {
    ///     prompt: "do stuff".into(),
    ///     model: Some("o3".into()),
    ///     cwd: PathBuf::from("."),
    ///     env: BTreeMap::new(),
    ///     interactive: false,
    ///     inactivity_timeout: None,
    ///     allow_all: true,
    /// };
    /// let args = harness.build_args(&config);
    /// assert_eq!(args, vec!["exec", "--model", "o3", "--yolo", "do stuff"]);
    /// ```
    ///
    /// # Returns
    ///
    /// A Vec<String> of command-line arguments to pass to the `codex` binary.
    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
        let mut args = vec!["exec".to_string()];
        if let Some(model) = config.model.as_deref() {
            args.push("--model".to_string());
            args.push(model.to_string());
        }
        if config.allow_all {
            args.push("--yolo".to_string());
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
    fn harness_name_is_codex() {
        let harness = CodexHarness;
        assert_eq!(harness.harness_name(), HarnessName::Codex);
    }

    #[test]
    fn binary_is_codex() {
        let harness = CodexHarness;
        assert_eq!(harness.binary(), "codex");
    }

    #[test]
    fn build_args_with_allow_all() {
        let harness = CodexHarness;
        let cfg = config(Allow::All, Some("o3"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["exec", "--model", "o3", "--yolo", "do stuff"]);
    }

    #[test]
    fn build_args_without_allow_all() {
        let harness = CodexHarness;
        let cfg = config(Allow::None, Some("o3"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["exec", "--model", "o3", "do stuff"]);
    }
}