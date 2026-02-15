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
    fn harness_name(&self) -> HarnessName {
        HarnessName::GithubCopilot
    }

    fn binary(&self) -> &str {
        "copilot"
    }

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

    fn config(allow_all: bool, model: Option<&str>) -> HarnessRunConfig {
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
        let cfg = config(true, Some("gpt-4"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "gpt-4", "--yolo", "-p", "do stuff"]);
    }

    #[test]
    fn build_args_without_allow_all() {
        let harness = GitHubCopilotHarness;
        let cfg = config(false, Some("gpt-4"));
        let args = harness.build_args(&cfg);
        assert_eq!(args, vec!["--model", "gpt-4", "-p", "do stuff"]);
    }
}
