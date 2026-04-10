//! Git remote URL resolution for org/repo namespace discovery.
//!
//! Provides helpers to determine the `(org, repo)` pair for the current
//! project by consulting config first and falling back to the `origin` remote
//! URL when config is incomplete.

use std::path::Path;

use ito_config::types::BackendApiConfig;

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

/// Resolve the `(org, repo)` pair for the current project.
///
/// Resolution order:
///
/// 1. `backend.project.org` / `backend.project.repo` from the supplied config
///    (both must be non-empty for this source to be used).
/// 2. Parse the `origin` remote URL via `git remote get-url origin` run in
///    `repo_root`, delegating URL parsing to
///    [`ito_common::git_url::parse_remote_url_org_repo`].
///
/// Returns `None` when neither source yields a complete pair (e.g., no config
/// values and no `origin` remote, or the remote URL is in an unrecognised
/// format).
pub fn resolve_org_repo_from_config_or_remote(
    repo_root: &Path,
    config: &BackendApiConfig,
) -> Option<(String, String)> {
    let runner = SystemProcessRunner;
    resolve_org_repo_from_config_or_remote_with_runner(&runner, repo_root, config)
}

/// Testable inner implementation that accepts an injected [`ProcessRunner`].
pub(crate) fn resolve_org_repo_from_config_or_remote_with_runner(
    runner: &dyn ProcessRunner,
    repo_root: &Path,
    config: &BackendApiConfig,
) -> Option<(String, String)> {
    // 1. Config takes priority when both fields are present and non-empty.
    let config_org = config
        .project
        .org
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from);
    let config_repo = config
        .project
        .repo
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from);

    if let (Some(org), Some(repo)) = (config_org, config_repo) {
        return Some((org, repo));
    }

    // 2. Fall back to parsing the `origin` remote URL.
    let url = get_origin_remote_url(runner, repo_root)?;
    ito_common::git_url::parse_remote_url_org_repo(&url)
}

/// Parse `<org>/<repo>` from a git remote URL.
///
/// This is a thin re-export of
/// [`ito_common::git_url::parse_remote_url_org_repo`] for callers that already
/// depend on `ito-core` and do not want to add a direct `ito-common` dependency.
///
/// See the `ito_common::git_url` module for the full list of supported URL
/// formats and edge-case behaviour.
pub fn parse_remote_url_org_repo(url: &str) -> Option<(String, String)> {
    ito_common::git_url::parse_remote_url_org_repo(url)
}

/// Attempt to resolve `(org, repo)` from the `origin` remote URL only.
///
/// Runs `git remote get-url origin` in `repo_root` and parses the result.
/// Returns `Ok(Some((org, repo)))` on success, `Ok(None)` when no origin is
/// configured or the URL format is not recognised, and `Err` only on process
/// execution failure.
pub fn resolve_org_repo_from_remote(repo_root: &Path) -> CoreResult<Option<(String, String)>> {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(repo_root);
    let output = runner
        .run(&request)
        .map_err(|e| CoreError::process(format!("git remote get-url origin failed: {e}")))?;
    if !output.success {
        return Ok(None);
    }
    let url = output.stdout.trim().to_string();
    Ok(ito_common::git_url::parse_remote_url_org_repo(&url))
}

/// Run `git remote get-url origin` in `repo_root` and return the trimmed URL.
///
/// Returns `None` when the command fails or produces no output (e.g., no
/// `origin` remote is configured).
fn get_origin_remote_url(runner: &dyn ProcessRunner, repo_root: &Path) -> Option<String> {
    let request = ProcessRequest::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(repo_root);
    let output = runner.run(&request).ok()?;
    if !output.success {
        return None;
    }
    let url = output.stdout.trim().to_string();
    if url.is_empty() {
        return None;
    }
    Some(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessExecutionError, ProcessOutput};
    use ito_config::types::BackendProjectConfig;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    struct StubRunner {
        outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    }

    impl StubRunner {
        fn with_outputs(outputs: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
            Self {
                outputs: RefCell::new(outputs.into()),
            }
        }
    }

    impl ProcessRunner for StubRunner {
        fn run(&self, _request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            self.outputs
                .borrow_mut()
                .pop_front()
                .expect("expected process output")
        }

        fn run_with_timeout(
            &self,
            _request: &ProcessRequest,
            _timeout: std::time::Duration,
        ) -> Result<ProcessOutput, ProcessExecutionError> {
            unreachable!("not used")
        }
    }

    fn ok_output(stdout: &str) -> ProcessOutput {
        ProcessOutput {
            exit_code: 0,
            success: true,
            stdout: stdout.to_string(),
            stderr: String::new(),
            timed_out: false,
        }
    }

    fn err_output(stderr: &str) -> ProcessOutput {
        ProcessOutput {
            exit_code: 1,
            success: false,
            stdout: String::new(),
            stderr: stderr.to_string(),
            timed_out: false,
        }
    }

    fn config_with_project(org: &str, repo: &str) -> BackendApiConfig {
        BackendApiConfig {
            project: BackendProjectConfig {
                org: Some(org.to_string()),
                repo: Some(repo.to_string()),
            },
            ..BackendApiConfig::default()
        }
    }

    // ── Config-first resolution ───────────────────────────────────────────────

    #[test]
    fn returns_config_values_when_both_set() {
        let config = config_with_project("acme", "widget");
        // Runner must not be called when config provides both values.
        let runner = StubRunner::with_outputs(vec![]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    #[test]
    fn falls_back_to_remote_when_config_org_missing() {
        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: None,
                repo: Some("widget".to_string()),
            },
            ..BackendApiConfig::default()
        };
        let runner =
            StubRunner::with_outputs(vec![Ok(ok_output("git@github.com:acme/widget.git\n"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    #[test]
    fn falls_back_to_remote_when_config_repo_missing() {
        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: Some("acme".to_string()),
                repo: None,
            },
            ..BackendApiConfig::default()
        };
        let runner =
            StubRunner::with_outputs(vec![Ok(ok_output("https://github.com/acme/widget.git\n"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    #[test]
    fn falls_back_to_remote_when_config_empty() {
        let config = BackendApiConfig::default();
        let runner =
            StubRunner::with_outputs(vec![Ok(ok_output("git@github.com:withakay/ito.git\n"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, Some(("withakay".to_string(), "ito".to_string())));
    }

    #[test]
    fn ignores_empty_config_strings_and_falls_back_to_remote() {
        let config = BackendApiConfig {
            project: BackendProjectConfig {
                org: Some("".to_string()),
                repo: Some("".to_string()),
            },
            ..BackendApiConfig::default()
        };
        let runner =
            StubRunner::with_outputs(vec![Ok(ok_output("https://github.com/acme/widget.git\n"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, Some(("acme".to_string(), "widget".to_string())));
    }

    // ── Remote-command failure paths ──────────────────────────────────────────

    #[test]
    fn returns_none_when_remote_command_fails() {
        let config = BackendApiConfig::default();
        let runner =
            StubRunner::with_outputs(vec![Ok(err_output("fatal: No such remote 'origin'"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn returns_none_when_remote_url_unrecognised() {
        let config = BackendApiConfig::default();
        // A URL with only one path component — cannot extract org/repo.
        let runner = StubRunner::with_outputs(vec![Ok(ok_output("https://github.com/onlyone\n"))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn returns_none_when_remote_output_is_empty() {
        let config = BackendApiConfig::default();
        let runner = StubRunner::with_outputs(vec![Ok(ok_output(""))]);
        let result = resolve_org_repo_from_config_or_remote_with_runner(
            &runner,
            std::env::temp_dir().as_path(),
            &config,
        );
        assert_eq!(result, None);
    }

    // ── parse_remote_url_org_repo re-export ───────────────────────────────────

    #[test]
    fn reexport_delegates_to_common_parser() {
        assert_eq!(
            parse_remote_url_org_repo("git@github.com:withakay/ito.git"),
            Some(("withakay".to_string(), "ito".to_string()))
        );
        assert_eq!(parse_remote_url_org_repo(""), None);
    }
}
