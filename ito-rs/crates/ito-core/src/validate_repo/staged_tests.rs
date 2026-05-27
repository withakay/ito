use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
use std::cell::RefCell;
use std::time::Duration;

/// Test runner that returns canned output for the first call and records
/// each request it sees, so tests can assert on the issued git command.
struct FakeRunner {
    output: Result<ProcessOutput, String>,
    seen: RefCell<Vec<ProcessRequest>>,
}

impl FakeRunner {
    fn ok(stdout: &str) -> Self {
        Self {
            output: Ok(ProcessOutput {
                exit_code: 0,
                success: true,
                stdout: stdout.to_string(),
                stderr: String::new(),
                timed_out: false,
            }),
            seen: RefCell::new(Vec::new()),
        }
    }

    fn failed(exit_code: i32, stderr: &str) -> Self {
        Self {
            output: Ok(ProcessOutput {
                exit_code,
                success: false,
                stdout: String::new(),
                stderr: stderr.to_string(),
                timed_out: false,
            }),
            seen: RefCell::new(Vec::new()),
        }
    }

    fn missing_git() -> Self {
        Self {
            output: Err("program 'git' not found on PATH".to_string()),
            seen: RefCell::new(Vec::new()),
        }
    }
}

impl ProcessRunner for FakeRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.seen.borrow_mut().push(request.clone());
        match &self.output {
            Ok(o) => Ok(o.clone()),
            Err(msg) => Err(ProcessExecutionError::InvalidRequest {
                detail: msg.clone(),
            }),
        }
    }

    fn run_with_timeout(
        &self,
        request: &ProcessRequest,
        _timeout: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        self.run(request)
    }
}

#[test]
fn from_git_returns_empty_snapshot_for_empty_index() {
    let runner = FakeRunner::ok("");
    let staged = StagedFiles::from_git(&runner, Path::new("/tmp/repo"))
        .expect("empty index should not error");
    assert!(staged.is_empty());

    // The runner should have been asked to run the right command.
    let seen = runner.seen.borrow();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].program, "git");
    assert_eq!(seen[0].args, vec!["diff", "--cached", "--name-only", "-z"]);
    assert_eq!(seen[0].current_dir.as_deref(), Some(Path::new("/tmp/repo")));
}

#[test]
fn from_git_parses_z_delimited_paths_and_handles_newlines_in_filenames() {
    // Path with embedded newline: `weird\nname.txt`. The `-z` flag is
    // exactly what makes this safe to parse — newlines remain inside
    // the path and the separator is `\0`.
    let runner = FakeRunner::ok("a.rs\0b/c.rs\0weird\nname.txt\0");
    let staged =
        StagedFiles::from_git(&runner, Path::new("/tmp/repo")).expect("valid output should parse");

    assert_eq!(staged.len(), 3);
    let paths: Vec<&Path> = staged.iter().collect();
    // Sorted lexicographically.
    assert_eq!(
        paths,
        vec![
            Path::new("a.rs"),
            Path::new("b/c.rs"),
            Path::new("weird\nname.txt"),
        ]
    );
}

#[test]
fn from_git_propagates_spawn_error_with_what_why_fix_message() {
    let runner = FakeRunner::missing_git();
    let err = StagedFiles::from_git(&runner, Path::new("/tmp/repo"))
        .expect_err("missing git should surface");

    let msg = format!("{err}");
    assert!(
        msg.contains("Cannot read staged files"),
        "expected What/Why/Fix style error, got: {msg}",
    );
    assert!(msg.contains("Fix:"), "missing Fix: line in: {msg}");
}

#[test]
fn from_git_returns_error_when_git_exits_non_zero() {
    let runner = FakeRunner::failed(128, "fatal: not a git repository");
    let err = StagedFiles::from_git(&runner, Path::new("/tmp/not-a-repo"))
        .expect_err("non-zero git exit should surface");

    let msg = format!("{err}");
    assert!(
        msg.contains("exited with code 128"),
        "expected exit code in error, got: {msg}",
    );
    assert!(
        msg.contains("not a git repository"),
        "expected stderr to be threaded through, got: {msg}",
    );
}

#[test]
fn empty_snapshot_reports_zero_length() {
    let staged = StagedFiles::empty();
    assert!(staged.is_empty());
    assert_eq!(staged.len(), 0);
    assert!(staged.iter().next().is_none());
}

#[test]
fn from_paths_deduplicates_and_orders_lexicographically() {
    let staged = StagedFiles::from_paths(vec![
        PathBuf::from("src/b.rs"),
        PathBuf::from("src/a.rs"),
        PathBuf::from("src/a.rs"),
    ]);

    assert_eq!(staged.len(), 2);

    let paths: Vec<&Path> = staged.iter().collect();
    assert_eq!(
        paths,
        vec![Path::new("src/a.rs"), Path::new("src/b.rs")],
        "iter() must yield paths in lexicographic order with duplicates removed",
    );
}

#[test]
fn contains_matches_a_staged_path() {
    let staged = StagedFiles::from_paths(vec![PathBuf::from(".gitignore")]);
    assert!(staged.contains(Path::new(".gitignore")));
    assert!(!staged.contains(Path::new("README.md")));
}

#[test]
fn from_z_separated_handles_trailing_nul() {
    let staged = StagedFiles::from_z_separated("a.rs\0b.rs\0");
    let paths: Vec<&Path> = staged.iter().collect();
    assert_eq!(paths, vec![Path::new("a.rs"), Path::new("b.rs")]);
}

#[test]
fn from_z_separated_handles_consecutive_nuls() {
    // Pathological but possible if upstream tooling double-encodes.
    let staged = StagedFiles::from_z_separated("a.rs\0\0b.rs");
    let paths: Vec<&Path> = staged.iter().collect();
    assert_eq!(
        paths,
        vec![Path::new("a.rs"), Path::new("b.rs")],
        "consecutive NULs must not produce empty path entries",
    );
}

#[test]
fn from_z_separated_handles_only_nuls() {
    let staged = StagedFiles::from_z_separated("\0\0\0");
    assert!(staged.is_empty());
}

#[test]
fn from_z_separated_handles_empty_input() {
    let staged = StagedFiles::from_z_separated("");
    assert!(staged.is_empty());
}
