//! Snapshot of files currently staged in the git index.
//!
//! [`StagedFiles::from_git`] runs `git diff --cached --name-only -z` via a
//! [`crate::process::ProcessRunner`] and parses the null-byte-delimited
//! output. The null delimiter (rather than newline) is required because git
//! paths may legitimately contain newlines.
//!
//! Tests inject a mock [`ProcessRunner`] to avoid spawning real git
//! processes; integration tests exercise the real binary via `tempfile`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessRequest, ProcessRunner};

/// An ordered, deduplicated snapshot of paths staged in the git index.
///
/// Paths are stored relative to the project root (matching git's own output
/// from `git diff --cached --name-only`). Iteration is lexicographic.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StagedFiles {
    paths: BTreeSet<PathBuf>,
}

impl StagedFiles {
    /// Construct an empty snapshot.
    ///
    /// Used by full-repo validation flows where the caller does not care
    /// about the staging area (rules that consult the staging area treat
    /// "no staged paths" as "skip").
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Construct a snapshot from a fixed list of paths.
    ///
    /// Primarily a test helper; production code uses [`Self::from_git`].
    #[must_use]
    pub fn from_paths<I, P>(paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }

    /// Construct a snapshot by running `git diff --cached --name-only -z`
    /// in `project_root`.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::process`] when:
    ///
    /// - the `git` binary cannot be spawned (typically: not on `PATH`);
    /// - the command exits non-zero (typically: not a git repository or
    ///   index corrupt).
    ///
    /// All error messages follow the `What / Why / Fix` convention from
    /// `ito-rs/AGENTS.md`.
    pub fn from_git(runner: &dyn ProcessRunner, project_root: &Path) -> CoreResult<Self> {
        let request = ProcessRequest::new("git")
            .args(["diff", "--cached", "--name-only", "-z"])
            .current_dir(project_root);

        let output = runner.run(&request).map_err(|err| {
            CoreError::process(format!(
                "Cannot read staged files from the git index.\n\
                 Git command failed to run: {err}\n\
                 Fix: ensure git is installed and `{root}` is a git repository.",
                root = project_root.display(),
            ))
        })?;

        if !output.success {
            return Err(CoreError::process(format!(
                "`git diff --cached --name-only -z` exited with code {code} in `{root}`.\n\
                 stderr: {stderr}\n\
                 Fix: confirm `{root}` is a git repository (`git status` should succeed) \
                 and that the index is not corrupt.",
                code = output.exit_code,
                root = project_root.display(),
                stderr = output.stderr.trim(),
            )));
        }

        Ok(Self::from_z_separated(&output.stdout))
    }

    /// Parse the null-byte-delimited output of `git diff --cached --name-only -z`.
    ///
    /// Empty entries (which occur as the trailing NUL after the last path,
    /// or in pathological double-NUL inputs) are filtered out so the
    /// resulting set never contains a `PathBuf::from("")`.
    pub(crate) fn from_z_separated(stdout: &str) -> Self {
        let paths = stdout
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect();
        Self { paths }
    }

    /// True if no paths are staged.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    /// Number of staged paths.
    #[must_use]
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// True if the given path is in the staging area.
    #[must_use]
    pub fn contains(&self, path: &Path) -> bool {
        self.paths.contains(path)
    }

    /// Iterate over staged paths in lexicographic order.
    pub fn iter(&self) -> impl Iterator<Item = &Path> {
        self.paths.iter().map(PathBuf::as_path)
    }
}

#[cfg(test)]
mod tests {
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
        let staged = StagedFiles::from_git(&runner, Path::new("/tmp/repo"))
            .expect("valid output should parse");

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
}
