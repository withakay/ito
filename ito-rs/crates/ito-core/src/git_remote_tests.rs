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
    let runner = StubRunner::with_outputs(vec![Ok(ok_output("git@github.com:acme/widget.git\n"))]);
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
    let runner = StubRunner::with_outputs(vec![Ok(ok_output("git@github.com:withakay/ito.git\n"))]);
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
    let runner = StubRunner::with_outputs(vec![Ok(err_output("fatal: No such remote 'origin'"))]);
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
