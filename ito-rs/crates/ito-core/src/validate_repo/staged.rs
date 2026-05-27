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
#[path = "staged_tests.rs"]
mod staged_tests;
