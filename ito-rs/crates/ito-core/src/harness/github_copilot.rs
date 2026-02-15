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
    /// Identifies this harness as GitHub Copilot.
    ///
    /// # Returns
    ///
    /// `HarnessName::GithubCopilot`
    ///
    /// # Examples
    ///
    /// ```
    /// let harness = GitHubCopilotHarness::default();
    /// assert_eq!(harness.harness_name(), HarnessName::GithubCopilot);
    /// ```
    fn harness_name(&self) -> HarnessName {
        HarnessName::GithubCopilot
    }

    /// Returns the CLI binary name used by the GitHub Copilot harness.
    ///
    /// # Returns
    ///
    /// `"copilot"` — the binary name to invoke.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = GitHubCopilotHarness::default();
    /// assert_eq!(h.binary(), "copilot");
    /// ```
    fn binary(&self) -> &str {
        "copilot"
    }

    /// Builds the command-line arguments for invoking the `copilot` CLI from a run configuration.
    ///
    /// The resulting argument vector includes, in order:
    /// - `--model <model>` when `config.model` is set,
    /// - `--yolo` when `config.allow_all` is true,
    /// - `-p` to enable non-interactive print mode,
    /// - the prompt string from `config.prompt`.
    ///
    /// # Parameters
    ///
    /// - `config`: run configuration whose `model`, `allow_all`, and `prompt` fields determine the arguments.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the assembled command-line arguments suitable for passing to the `copilot` binary.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    enum Allow {
        All,
        None,
    }

    /// Construct a HarnessRunConfig for tests with a fixed prompt and configurable model and permission.
    ///
    /// - `allow` controls whether `allow_all` is set to `true` (Allow::All) or `false` (Allow::None).
    /// - `model` sets the optional model string used by the config.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = config(Allow::All, Some("gpt-4"));
    /// assert_eq!(cfg.prompt, "do stuff");
    /// assert_eq!(cfg.model.as_deref(), Some("gpt-4"));
    /// assert!(cfg.allow_all);
    /// assert!(!cfg.interactive);
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
    fn harness_name_is_github_copilot() {
        let harness = GitHubCopilotHarness;
        assert_eq!(harness.harness_name(), HarnessName::GithubCopilot);
    }

    #[test]
    fn binary_is_copilot() {
        let harness = GitHubCopilotHarness;
        assert_eq!(harness.binary(), "copilot");
    }

    #[test]
    fn build_args_with_allow_all() {
        let harness = GitHubCopilotHarness;
        let cfg = config(Allow::All, Some("gpt-4"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "gpt-4", "--yolo", "-p", "do stuff"]);
    }

    #[test]
    fn build_args_without_allow_all() {
        let harness = GitHubCopilotHarness;
        let cfg = config(Allow::None, Some("gpt-4"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "gpt-4", "-p", "do stuff"]);
    }
}