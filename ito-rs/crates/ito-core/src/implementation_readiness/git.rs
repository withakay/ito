//! Git process adapter for readiness authority evaluation.

use std::path::{Path, PathBuf};

use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

/// Tracked upstream metadata needed for authority resolution and scoped refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrackedUpstream {
    /// Local remote-tracking ref used as proposal authority.
    pub tracking_ref: String,
    /// Configured remote name.
    pub remote: String,
    /// Branch ref advertised by the remote.
    pub remote_ref: String,
}

/// One entry read directly from an immutable Git tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitTreeEntry {
    /// Git file mode such as `100644` or `120000`.
    pub mode: String,
    /// Git object type such as `blob` or `commit`.
    pub object_type: String,
    /// Object OID named by the tree entry.
    pub oid: String,
    /// Repository-relative path stored in the tree.
    pub path: String,
}

impl GitTreeEntry {
    /// Whether the entry is a regular file blob rather than a symlink or gitlink.
    pub(crate) fn is_regular_blob(&self) -> bool {
        self.object_type == "blob" && matches!(self.mode.as_str(), "100644" | "100755")
    }
}

/// Immutable identity of one checkout used by execute readiness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckoutState {
    /// Canonical worktree top-level, or the supplied bare control path.
    pub root: PathBuf,
    /// Canonical Git common directory shared by linked worktrees.
    pub common_dir: PathBuf,
    /// Commit currently checked out at `HEAD`.
    pub head_oid: String,
    /// Local branch name, or `None` for detached/bare checkouts.
    pub branch: Option<String>,
    /// Whether this path is a bare control repository rather than a worktree.
    pub is_bare: bool,
}

/// Failure returned by the readiness Git boundary.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub(crate) struct ReadinessGitError {
    message: String,
}

impl ReadinessGitError {
    /// Create a Git-boundary failure with an actionable detail message.
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Git operations required by readiness evaluation.
pub(crate) trait ReadinessGit {
    /// Resolve the tracked upstream metadata for a local target branch ref.
    fn tracked_upstream(
        &self,
        repository_root: &Path,
        local_branch_ref: &str,
    ) -> Result<TrackedUpstream, ReadinessGitError>;

    /// Fetch exactly one configured upstream branch into its tracking ref.
    fn refresh_upstream(
        &self,
        repository_root: &Path,
        upstream: &TrackedUpstream,
    ) -> Result<(), ReadinessGitError>;

    /// Resolve a ref to a commit OID.
    fn resolve_commit(
        &self,
        repository_root: &Path,
        target_ref: &str,
    ) -> Result<String, ReadinessGitError>;

    /// List immutable tree entries below one literal repository-relative path.
    fn list_tree(
        &self,
        _repository_root: &Path,
        _authority_oid: &str,
        _path: &str,
    ) -> Result<Vec<GitTreeEntry>, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "authority tree listing is not implemented by this Git adapter",
        ))
    }

    /// Read one blob by object OID without consulting a checkout.
    fn read_blob(
        &self,
        _repository_root: &Path,
        _blob_oid: &str,
    ) -> Result<String, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "authority blob reading is not implemented by this Git adapter",
        ))
    }

    /// Find the newest first-parent target commit that introduced one literal marker path.
    fn find_introduction_commit(
        &self,
        _repository_root: &Path,
        _authority_oid: &str,
        _marker_path: &str,
    ) -> Result<String, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "proposal integration discovery is not implemented by this Git adapter",
        ))
    }

    /// Inspect checkout identity without reading proposal files from it.
    fn inspect_checkout(&self, _checkout: &Path) -> Result<CheckoutState, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "checkout identity inspection is not implemented by this Git adapter",
        ))
    }

    /// Test whether `ancestor_oid` is an ancestor of `descendant_oid`.
    fn is_ancestor(
        &self,
        _checkout: &Path,
        _ancestor_oid: &str,
        _descendant_oid: &str,
    ) -> Result<bool, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "checkout ancestry inspection is not implemented by this Git adapter",
        ))
    }
}

#[derive(Debug, Default)]
pub(super) struct SystemReadinessGit;

impl ReadinessGit for SystemReadinessGit {
    fn tracked_upstream(
        &self,
        repository_root: &Path,
        local_branch_ref: &str,
    ) -> Result<TrackedUpstream, ReadinessGitError> {
        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "for-each-ref",
                "--format=%(refname)%00%(upstream)%00%(upstream:remotename)%00%(upstream:remoteref)",
                local_branch_ref,
            ],
            "inspect target branch upstream",
        )?;
        let fields = output
            .lines()
            .map(|line| line.split('\0').collect::<Vec<_>>())
            .find(|fields| fields.first().copied() == Some(local_branch_ref))
            .unwrap_or_default();
        let tracking_ref = fields.get(1).copied().unwrap_or_default().trim();
        let remote = fields.get(2).copied().unwrap_or_default().trim();
        let remote_ref = fields.get(3).copied().unwrap_or_default().trim();
        if tracking_ref.is_empty() || remote.is_empty() || remote_ref.is_empty() {
            return Err(ReadinessGitError::new(format!(
                "target branch '{local_branch_ref}' has no complete tracked upstream configuration"
            )));
        }

        Ok(TrackedUpstream {
            tracking_ref: tracking_ref.to_string(),
            remote: remote.to_string(),
            remote_ref: remote_ref.to_string(),
        })
    }

    fn refresh_upstream(
        &self,
        repository_root: &Path,
        upstream: &TrackedUpstream,
    ) -> Result<(), ReadinessGitError> {
        let refspec = format!("+{}:{}", upstream.remote_ref, upstream.tracking_ref);
        run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "fetch",
                "--no-tags",
                "--no-write-fetch-head",
                upstream.remote.as_str(),
                refspec.as_str(),
            ],
            "refresh target branch upstream",
        )?;
        Ok(())
    }

    fn resolve_commit(
        &self,
        repository_root: &Path,
        target_ref: &str,
    ) -> Result<String, ReadinessGitError> {
        let commit_ref = format!("{target_ref}^{{commit}}");
        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "rev-parse",
                "--verify",
                "--end-of-options",
                commit_ref.as_str(),
            ],
            "resolve authority commit",
        )?;
        let oid = output.trim();
        if !matches!(oid.len(), 40 | 64) || !oid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ReadinessGitError::new(format!(
                "authority resolution returned an invalid commit OID: '{oid}'"
            )));
        }
        Ok(oid.to_ascii_lowercase())
    }

    fn list_tree(
        &self,
        repository_root: &Path,
        authority_oid: &str,
        path: &str,
    ) -> Result<Vec<GitTreeEntry>, ReadinessGitError> {
        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "--no-replace-objects",
                "--no-lazy-fetch",
                "--literal-pathspecs",
                "ls-tree",
                "-r",
                "-z",
                "--full-tree",
                authority_oid,
                "--",
                path,
            ],
            "list authority tree",
        )?;
        parse_tree_entries(&output)
    }

    fn read_blob(
        &self,
        repository_root: &Path,
        blob_oid: &str,
    ) -> Result<String, ReadinessGitError> {
        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "--no-replace-objects",
                "--no-lazy-fetch",
                "cat-file",
                "blob",
                blob_oid,
            ],
            "read authority blob",
        )?;
        if output.contains('\u{fffd}') {
            return Err(ReadinessGitError::new(format!(
                "authority blob '{blob_oid}' is not valid UTF-8"
            )));
        }
        Ok(output)
    }

    fn find_introduction_commit(
        &self,
        repository_root: &Path,
        authority_oid: &str,
        marker_path: &str,
    ) -> Result<String, ReadinessGitError> {
        let shallow = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "--no-replace-objects",
                "--no-lazy-fetch",
                "rev-parse",
                "--is-shallow-repository",
            ],
            "inspect repository history depth",
        )?;
        if shallow.trim() == "true" {
            return Err(ReadinessGitError::new(
                "cannot prove proposal integration from shallow Git history",
            ));
        }

        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "--no-replace-objects",
                "--no-lazy-fetch",
                "--literal-pathspecs",
                "log",
                "--first-parent",
                "--format=%H",
                "--diff-filter=A",
                "--no-renames",
                "--end-of-options",
                authority_oid,
                "--",
                marker_path,
            ],
            "discover proposal integration commit",
        )?;
        let candidate = output
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .ok_or_else(|| {
                ReadinessGitError::new(format!(
                    "target history does not contain an introduction commit for '{marker_path}'"
                ))
            })?;
        validate_oid(candidate)?;

        let candidate_entries = self.list_tree(repository_root, candidate, marker_path)?;
        if !candidate_entries
            .iter()
            .any(|entry| entry.path == marker_path && entry.is_regular_blob())
        {
            return Err(ReadinessGitError::new(format!(
                "candidate integration commit '{candidate}' does not contain regular marker '{marker_path}'"
            )));
        }

        let parents = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "--no-replace-objects",
                "--no-lazy-fetch",
                "rev-list",
                "--parents",
                "-n",
                "1",
                "--end-of-options",
                candidate,
            ],
            "inspect proposal integration parent",
        )?;
        let first_parent = parents.split_whitespace().nth(1);
        if let Some(first_parent) = first_parent {
            let parent_entries = self.list_tree(repository_root, first_parent, marker_path)?;
            if parent_entries.iter().any(|entry| entry.path == marker_path) {
                return Err(ReadinessGitError::new(format!(
                    "candidate integration commit '{candidate}' did not introduce marker '{marker_path}' on target first-parent history"
                )));
            }
        }

        Ok(candidate.to_ascii_lowercase())
    }

    fn inspect_checkout(&self, checkout: &Path) -> Result<CheckoutState, ReadinessGitError> {
        let is_bare = run_git(
            &SystemProcessRunner,
            checkout,
            ["rev-parse", "--is-bare-repository"],
            "inspect checkout repository kind",
        )?
        .trim()
            == "true";
        let common_dir = git_absolute_path(
            checkout,
            &run_git(
                &SystemProcessRunner,
                checkout,
                ["rev-parse", "--path-format=absolute", "--git-common-dir"],
                "resolve checkout common Git directory",
            )?,
        )?;
        let root = if is_bare {
            std::fs::canonicalize(checkout).map_err(|error| {
                ReadinessGitError::new(format!(
                    "cannot canonicalize bare control checkout '{}': {error}",
                    checkout.display()
                ))
            })?
        } else {
            git_absolute_path(
                checkout,
                &run_git(
                    &SystemProcessRunner,
                    checkout,
                    ["rev-parse", "--path-format=absolute", "--show-toplevel"],
                    "resolve checkout top-level",
                )?,
            )?
        };
        let head_oid = self.resolve_commit(checkout, "HEAD")?;
        let branch_output = run_git(
            &SystemProcessRunner,
            checkout,
            ["rev-parse", "--abbrev-ref=strict", "HEAD"],
            "resolve checkout branch",
        )?;
        let branch = branch_output.trim().to_string();
        let branch = (!is_bare && branch != "HEAD" && !branch.is_empty()).then_some(branch);

        Ok(CheckoutState {
            root,
            common_dir,
            head_oid,
            branch,
            is_bare,
        })
    }

    fn is_ancestor(
        &self,
        checkout: &Path,
        ancestor_oid: &str,
        descendant_oid: &str,
    ) -> Result<bool, ReadinessGitError> {
        validate_oid(ancestor_oid)?;
        validate_oid(descendant_oid)?;
        let output = SystemProcessRunner
            .run(
                &ProcessRequest::new("git")
                    .args([
                        "--no-replace-objects",
                        "--no-lazy-fetch",
                        "merge-base",
                        "--is-ancestor",
                        ancestor_oid,
                        descendant_oid,
                    ])
                    .current_dir(checkout),
            )
            .map_err(|error| {
                ReadinessGitError::new(format!("inspect checkout ancestry failed: {error}"))
            })?;
        match output.exit_code {
            0 => Ok(true),
            1 => Ok(false),
            _ => {
                let detail = if output.stderr.trim().is_empty() {
                    output.stdout.trim()
                } else {
                    output.stderr.trim()
                };
                Err(ReadinessGitError::new(format!(
                    "inspect checkout ancestry failed with exit code {}: {detail}",
                    output.exit_code
                )))
            }
        }
    }
}

fn git_absolute_path(cwd: &Path, output: &str) -> Result<PathBuf, ReadinessGitError> {
    let raw = output.trim();
    if raw.is_empty() {
        return Err(ReadinessGitError::new(
            "Git returned an empty absolute path",
        ));
    }
    let path = PathBuf::from(raw);
    let path = if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    };
    std::fs::canonicalize(&path).map_err(|error| {
        ReadinessGitError::new(format!(
            "cannot canonicalize Git path '{}': {error}",
            path.display()
        ))
    })
}

fn parse_tree_entries(output: &str) -> Result<Vec<GitTreeEntry>, ReadinessGitError> {
    let mut entries = Vec::new();
    for record in output.split('\0').filter(|record| !record.is_empty()) {
        let (metadata, path) = record.split_once('\t').ok_or_else(|| {
            ReadinessGitError::new("authority tree returned malformed entry metadata")
        })?;
        let mut fields = metadata.split_whitespace();
        let mode = fields.next().unwrap_or_default();
        let object_type = fields.next().unwrap_or_default();
        let oid = fields.next().unwrap_or_default();
        if mode.is_empty() || object_type.is_empty() || fields.next().is_some() {
            return Err(ReadinessGitError::new(
                "authority tree returned malformed entry fields",
            ));
        }
        validate_oid(oid)?;
        entries.push(GitTreeEntry {
            mode: mode.to_string(),
            object_type: object_type.to_string(),
            oid: oid.to_ascii_lowercase(),
            path: path.to_string(),
        });
    }
    Ok(entries)
}

fn validate_oid(oid: &str) -> Result<(), ReadinessGitError> {
    if matches!(oid.len(), 40 | 64) && oid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Ok(());
    }
    Err(ReadinessGitError::new(format!(
        "Git returned an invalid object OID: '{oid}'"
    )))
}

fn run_git<const N: usize>(
    runner: &dyn ProcessRunner,
    repository_root: &Path,
    args: [&str; N],
    operation: &str,
) -> Result<String, ReadinessGitError> {
    let output = runner
        .run(
            &ProcessRequest::new("git")
                .args(args)
                .current_dir(repository_root),
        )
        .map_err(|error| ReadinessGitError::new(format!("{operation} failed: {error}")))?;
    if !output.success {
        let detail = if output.stderr.trim().is_empty() {
            output.stdout.trim()
        } else {
            output.stderr.trim()
        };
        return Err(ReadinessGitError::new(format!(
            "{operation} failed with exit code {}: {detail}",
            output.exit_code
        )));
    }
    Ok(output.stdout)
}
