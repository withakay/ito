//! Process execution boundary for core-side command invocation.

use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

const MAX_TOTAL_ARG_BYTES: usize = 256 * 1024;
static OUTPUT_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Process invocation request.
#[derive(Debug, Clone, Default)]
pub struct ProcessRequest {
    /// Executable name or absolute path.
    pub program: String,
    /// Positional arguments.
    pub args: Vec<String>,
    /// Optional working directory.
    pub current_dir: Option<PathBuf>,
}

impl ProcessRequest {
    /// Create a new request for the given program.
    ///
    /// The program can be an executable name (resolved via PATH) or an absolute path.
    /// Use the builder methods to configure arguments and working directory.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            current_dir: None,
        }
    }

    /// Add a single argument to the process invocation.
    ///
    /// This is a builder method that returns `self` for chaining.
    /// Arguments are passed to the process in the order they are added.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple arguments to the process invocation.
    ///
    /// This is a builder method that returns `self` for chaining.
    /// Arguments are appended in iteration order after any previously added arguments.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    /// Set the working directory for the process.
    ///
    /// If not set, the process inherits the current working directory.
    /// This is a builder method that returns `self` for chaining.
    pub fn current_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.current_dir = Some(dir.into());
        self
    }
}

/// Structured process execution output.
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    /// Exit status code, or -1 if unavailable.
    pub exit_code: i32,
    /// Whether the process exited successfully.
    pub success: bool,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// True if execution was forcibly terminated due to timeout.
    pub timed_out: bool,
}

/// Process execution failure modes.
#[derive(Debug, thiserror::Error)]
pub enum ProcessExecutionError {
    /// Spawn failed before a child process was created.
    #[error("failed to spawn '{program}': {source}")]
    Spawn {
        /// Program being executed.
        program: String,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// Waiting for process completion failed.
    #[error("failed waiting for '{program}': {source}")]
    Wait {
        /// Program being executed.
        program: String,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// Creating a temporary output file failed.
    #[error("failed to create temp output file '{path}': {source}")]
    CreateTemp {
        /// Temp path used for output capture.
        path: PathBuf,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// Reading a temporary output file failed.
    #[error("failed to read temp output file '{path}': {source}")]
    ReadTemp {
        /// Temp path used for output capture.
        path: PathBuf,
        /// Underlying I/O error.
        source: io::Error,
    },
    /// Invalid process request contents.
    #[error("invalid process request: {detail}")]
    InvalidRequest {
        /// Reason the request is invalid.
        detail: String,
    },
}

/// Abstraction for process execution.
pub trait ProcessRunner {
    /// Execute a process and wait for completion, capturing all output.
    ///
    /// This method blocks until the process exits or fails to start.
    /// Both stdout and stderr are captured and returned in the result.
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError>;

    /// Execute a process with a timeout, capturing all output.
    ///
    /// If the process doesn't complete within the timeout, it will be killed
    /// and the result will have `timed_out` set to true. Output captured
    /// before the timeout is still returned.
    fn run_with_timeout(
        &self,
        request: &ProcessRequest,
        timeout: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError>;
}

/// Default runner backed by `std::process::Command`.
#[derive(Debug, Default)]
pub struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        validate_request(request)?;
        let mut command = build_command(request);
        let output = command
            .output()
            .map_err(|source| ProcessExecutionError::Spawn {
                program: request.program.clone(),
                source,
            })?;
        Ok(ProcessOutput {
            exit_code: output.status.code().unwrap_or(-1),
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            timed_out: false,
        })
    }

    /// Executes the given process request and returns its captured output, enforcing the supplied timeout.
    ///
    /// On timeout the child process is killed; any output written before termination is returned and `timed_out` is set to `true`.
    ///
    /// # Returns
    ///
    /// A `ProcessOutput` containing the process exit code (or -1 if unavailable), a `success` flag (false if timed out or exit indicates failure), captured `stdout` and `stderr` as `String`s, and `timed_out` indicating whether the process was terminated due to the timeout.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use ito_core::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};
    ///
    /// let runner = SystemProcessRunner::default();
    /// let req = ProcessRequest::new("sh")
    ///     .arg("-c")
    ///     .arg("echo hello; sleep 0.01; echo world");
    /// let out = runner.run_with_timeout(&req, Duration::from_secs(1)).unwrap();
    /// assert!(out.stdout.contains("hello"));
    /// assert!(out.stdout.contains("world"));
    /// assert!(!out.timed_out);
    /// ```
    fn run_with_timeout(
        &self,
        request: &ProcessRequest,
        timeout: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        validate_request(request)?;
        let now_ms = chrono::Utc::now().timestamp_millis();
        let pid = std::process::id();
        let stdout_path = temp_output_path("stdout", pid, now_ms);
        let stderr_path = temp_output_path("stderr", pid, now_ms);

        let stdout_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&stdout_path)
            .map_err(|source| ProcessExecutionError::CreateTemp {
                path: stdout_path.clone(),
                source,
            })?;
        let stderr_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&stderr_path)
            .map_err(|source| ProcessExecutionError::CreateTemp {
                path: stderr_path.clone(),
                source,
            })?;

        let mut command = build_command(request);
        let mut child = command
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .map_err(|source| ProcessExecutionError::Spawn {
                program: request.program.clone(),
                source,
            })?;

        let started = Instant::now();
        let mut timed_out = false;
        let mut exit_code = -1;
        let mut success = false;

        loop {
            if let Some(status) =
                child
                    .try_wait()
                    .map_err(|source| ProcessExecutionError::Wait {
                        program: request.program.clone(),
                        source,
                    })?
            {
                exit_code = status.code().unwrap_or(-1);
                success = status.success();
                break;
            }

            if started.elapsed() >= timeout {
                timed_out = true;
                let _ = child.kill();
                let _ = child.wait();
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        let stdout =
            fs::read_to_string(&stdout_path).map_err(|source| ProcessExecutionError::ReadTemp {
                path: stdout_path.clone(),
                source,
            })?;
        let stderr =
            fs::read_to_string(&stderr_path).map_err(|source| ProcessExecutionError::ReadTemp {
                path: stderr_path.clone(),
                source,
            })?;
        let _ = fs::remove_file(&stdout_path);
        let _ = fs::remove_file(&stderr_path);

        Ok(ProcessOutput {
            exit_code,
            success: !timed_out && success,
            stdout,
            stderr,
            timed_out,
        })
    }
}

fn build_command(request: &ProcessRequest) -> Command {
    let mut command = Command::new(&request.program);
    command.args(&request.args);
    if let Some(dir) = &request.current_dir {
        command.current_dir(dir);
    }
    command
}

fn validate_request(request: &ProcessRequest) -> Result<(), ProcessExecutionError> {
    validate_program(&request.program)?;
    validate_args(&request.program, &request.args)?;
    validate_current_dir(&request.current_dir)?;
    Ok(())
}

fn validate_program(program: &str) -> Result<(), ProcessExecutionError> {
    if program.is_empty() {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "program is empty".to_string(),
        });
    }

    if program.contains('\0') {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "program contains NUL byte".to_string(),
        });
    }

    let program_path = Path::new(program);
    if program_path.is_absolute() {
        if contains_dot_components(program_path) {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "program path contains '.' or '..'".to_string(),
            });
        }
        return Ok(());
    }

    let mut components = program_path.components();
    let Some(component) = components.next() else {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "program path is empty".to_string(),
        });
    };

    match component {
        Component::Normal(_) => {}
        Component::CurDir => {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "program path must not be '.'".to_string(),
            });
        }
        Component::ParentDir => {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "program path must not include '..'".to_string(),
            });
        }
        Component::RootDir => {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "program path must be absolute when rooted".to_string(),
            });
        }
        Component::Prefix(_) => {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "program path prefix is not an executable name".to_string(),
            });
        }
    }

    if components.next().is_some() {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "program must be an executable name or absolute path".to_string(),
        });
    }

    Ok(())
}

fn validate_args(program: &str, args: &[String]) -> Result<(), ProcessExecutionError> {
    let mut total_bytes = program.len();
    if total_bytes > MAX_TOTAL_ARG_BYTES {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "program name exceeds maximum size".to_string(),
        });
    }

    for arg in args {
        if arg.contains('\0') {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "argument contains NUL byte".to_string(),
            });
        }

        total_bytes = total_bytes.saturating_add(arg.len());
        if total_bytes > MAX_TOTAL_ARG_BYTES {
            return Err(ProcessExecutionError::InvalidRequest {
                detail: "arguments exceed maximum total size".to_string(),
            });
        }
    }

    Ok(())
}

fn validate_current_dir(dir: &Option<PathBuf>) -> Result<(), ProcessExecutionError> {
    let Some(dir) = dir else {
        return Ok(());
    };

    if os_str_has_nul(dir.as_os_str()) {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "current_dir contains NUL byte".to_string(),
        });
    }

    if contains_dot_components(dir) {
        return Err(ProcessExecutionError::InvalidRequest {
            detail: "current_dir must not include '.' or '..'".to_string(),
        });
    }

    Ok(())
}

fn contains_dot_components(path: &Path) -> bool {
    for component in path.components() {
        match component {
            Component::CurDir | Component::ParentDir => return true,
            Component::Normal(_) | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    false
}

#[cfg(unix)]
fn os_str_has_nul(value: &std::ffi::OsStr) -> bool {
    value.as_bytes().contains(&0)
}

#[cfg(windows)]
fn os_str_has_nul(value: &std::ffi::OsStr) -> bool {
    value.encode_wide().any(|unit| unit == 0)
}

fn temp_output_path(stream: &str, pid: u32, now_ms: i64) -> PathBuf {
    let counter = OUTPUT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!("ito-process-{stream}-{pid}-{now_ms}-{counter}.log"));
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_stdout_and_stderr() {
        let runner = SystemProcessRunner;
        let request = ProcessRequest::new("sh").args(["-lc", "echo out; echo err >&2"]);
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
        let request = ProcessRequest::new("sh").args(["-lc", "echo boom >&2; exit 7"]);
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
}
