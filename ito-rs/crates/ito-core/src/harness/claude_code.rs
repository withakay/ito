use super::streaming_cli::CliHarness;
use super::types::{HarnessName, HarnessRunConfig};

/// Runs the `claude` CLI in non-interactive print mode (`claude -p`).
///
/// Selected via `ito ralph --harness claude`; requires the Claude Code CLI on PATH.
///
/// # Examples
///
/// ```
/// use ito_core::harness::{ClaudeCodeHarness, Harness, HarnessName};
///
/// let h = ClaudeCodeHarness;
/// assert_eq!(h.name(), HarnessName::Claude);
/// assert!(h.streams_output());
/// ```
#[derive(Debug, Default)]
pub struct ClaudeCodeHarness;

impl CliHarness for ClaudeCodeHarness {
    /// Identify the harness as Claude.
    ///
    /// Returns `HarnessName::Claude`.
    ///
    /// # Examples
    ///
    /// ```
    /// let h = ClaudeCodeHarness;
    /// assert_eq!(h.harness_name(), HarnessName::Claude);
    /// ```
    fn harness_name(&self) -> HarnessName {
        HarnessName::Claude
    }

    /// Binary name used to invoke the Claude CLI.
    ///
    /// # Returns
    ///
    /// `&str` containing the CLI binary name `claude`.
    ///
    /// # Examples
    ///
    /// ```
    /// let harness = ClaudeCodeHarness;
    /// assert_eq!(harness.binary(), "claude");
    /// ```
    fn binary(&self) -> &str {
        "claude"
    }

    /// Builds the command-line arguments for invoking the Claude CLI from a run configuration.
    ///
    /// The returned vector may include:
    /// - `--model <model>` when `config.model` is `Some`,
    /// - `--dangerously-skip-permissions` when `config.allow_all` is `true`,
    /// - and always `-p <prompt>` as the final prompt argument.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let harness = ClaudeCodeHarness;
    /// let cfg = HarnessRunConfig {
    ///     prompt: "do stuff".into(),
    ///     model: Some("sonnet".into()),
    ///     cwd: std::env::temp_dir(),
    ///     env: std::collections::BTreeMap::new(),
    ///     interactive: false,
    ///     allow_all: true,
    ///     inactivity_timeout: None,
    /// };
    /// let args = harness.build_args(&cfg);
    /// assert_eq!(
    ///     args,
    ///     vec![
    ///         "--model".to_string(),
    ///         "sonnet".to_string(),
    ///         "--dangerously-skip-permissions".to_string(),
    ///         "-p".to_string(),
    ///         "do stuff".to_string(),
    ///     ]
    /// );
    /// ```
    fn build_args(&self, config: &HarnessRunConfig) -> Vec<String> {
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

    /// Creates a HarnessRunConfig with the prompt "do stuff", an optional model, and `allow_all` derived from `allow`.
    ///
    /// `allow` maps to `allow_all`: `Allow::All` -> `true`, `Allow::None` -> `false`. If `model` is `Some`, the returned config's `model` is set to that value.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = config(Allow::All, Some("sonnet"));
    /// assert_eq!(cfg.prompt, "do stuff");
    /// assert_eq!(cfg.model.as_deref(), Some("sonnet"));
    /// assert!(cfg.allow_all);
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
    fn harness_name_is_claude() {
        let harness = ClaudeCodeHarness;
        assert_eq!(harness.harness_name(), HarnessName::Claude);
    }

    #[test]
    fn binary_is_claude() {
        let harness = ClaudeCodeHarness;
        assert_eq!(harness.binary(), "claude");
    }

    #[test]
    fn build_args_with_allow_all() {
        let harness = ClaudeCodeHarness;
        let cfg = config(Allow::All, Some("sonnet"));
        let args = harness.build_args(&cfg);
        assert_eq!(
            args,
            vec![
                "--model",
                "sonnet",
                "--dangerously-skip-permissions",
                "-p",
                "do stuff"
            ]
        );
    }

    #[test]
    fn build_args_without_allow_all() {
        let harness = ClaudeCodeHarness;
        let cfg = config(Allow::None, Some("sonnet"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "sonnet", "-p", "do stuff"]);
    }

    #[test]
    fn build_args_without_model() {
        let harness = ClaudeCodeHarness;
        let cfg = config(Allow::None, None);
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["-p", "do stuff"]);
    }
}