use super::*;

/// Verifies that SystemProcessRunner captures both standard output and standard error and reports the exit status and timeout flag correctly.
///
/// # Examples
///
/// ```
/// let runner = SystemProcessRunner;
/// let request = ProcessRequest::new("sh").args(["-c", "echo out; echo err >&2"]);
/// let output = runner.run(&request).unwrap();
/// assert!(output.success);
/// assert_eq!(output.exit_code, 0);
/// assert!(output.stdout.contains("out"));
/// assert!(output.stderr.contains("err"));
/// assert!(!output.timed_out);
/// ```
#[test]
fn captures_stdout_and_stderr() {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("sh").args(["-c", "echo out; echo err >&2"]);
    let output = runner.run(&request).unwrap();
    assert!(output.success);
    assert_eq!(output.exit_code, 0);
    assert!(output.stdout.contains("out"));
    assert!(output.stderr.contains("err"));
    assert!(!output.timed_out);
}

#[test]
fn captures_non_zero_exit() {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("sh").args(["-c", "echo boom >&2; exit 7"]);
    let output = runner.run(&request).unwrap();
    assert!(!output.success);
    assert_eq!(output.exit_code, 7);
    assert!(output.stderr.contains("boom"));
}

#[test]
fn missing_executable_is_spawn_failure() {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("__ito_missing_executable__");
    let result = runner.run(&request);
    match result {
        Err(ProcessExecutionError::Spawn { .. }) => {}
        other => panic!("expected spawn error, got {other:?}"),
    }
}

#[test]
fn rejects_empty_program() {
    let request = ProcessRequest::new("");
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("program is empty"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn rejects_nul_in_program() {
    let request = ProcessRequest::new("sh\0bad");
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("program contains NUL byte"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn rejects_relative_program_with_components() {
    let request = ProcessRequest::new("bin/sh");
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("executable name or absolute path"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn rejects_current_dir_with_parent_component() {
    let request = ProcessRequest::new("sh").current_dir("../tmp");
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("current_dir must not include"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn rejects_nul_in_argument() {
    let request = ProcessRequest::new("sh").arg("a\0b");
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("argument contains NUL byte"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn rejects_excessive_argument_bytes() {
    let oversized = "a".repeat(MAX_TOTAL_ARG_BYTES);
    let request = ProcessRequest::new("sh").arg(oversized);
    let result = validate_request(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("arguments exceed maximum total size"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}

#[test]
fn run_returns_invalid_request_before_spawn() {
    let runner = SystemProcessRunner;
    let request = ProcessRequest::new("bin/sh");
    let result = runner.run(&request);
    match result {
        Err(ProcessExecutionError::InvalidRequest { detail }) => {
            assert!(detail.contains("executable name or absolute path"));
        }
        other => panic!("expected invalid request, got {other:?}"),
    }
}
