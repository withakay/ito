//! Process and hashing helpers for coordination worktree lifecycle operations.

use std::path::Path;

use sha2::{Digest, Sha256};

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessOutput, ProcessRequest, ProcessRunner};

/// Run `git -C <worktree_path> add -A` to stage all changes.
pub(super) fn stage_all(runner: &dyn ProcessRunner, worktree_path: &Path) -> CoreResult<()> {
    let worktree_str = worktree_path.to_string_lossy();
    let request = ProcessRequest::new("git").args(["-C", worktree_str.as_ref(), "add", "-A"]);
    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot stage changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;
    if output.success {
        return Ok(());
    }
    Err(CoreError::process(format!(
        "Cannot stage changes in coordination worktree '{worktree}'.\n\
         Git reported: {detail}\n\
         Fix: ensure '{worktree}' is a valid git worktree and the files are readable.",
        worktree = worktree_path.display(),
        detail = render_output(&output),
    )))
}

/// Return whether staged changes are ready to commit.
pub(super) fn has_staged_changes(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
) -> CoreResult<bool> {
    let worktree_str = worktree_path.to_string_lossy();
    let request = ProcessRequest::new("git").args([
        "-C",
        worktree_str.as_ref(),
        "diff",
        "--cached",
        "--quiet",
    ]);
    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot check for staged changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;
    match output.exit_code {
        0 => Ok(false),
        1 => Ok(true),
        code => Err(CoreError::process(format!(
            "Cannot check for staged changes in coordination worktree '{worktree}'.\n\
             Git exited with unexpected code {code}: {detail}\n\
             Fix: ensure '{worktree}' is a valid git worktree.",
            worktree = worktree_path.display(),
            detail = render_output(&output),
        ))),
    }
}

/// Commit staged coordination changes with the supplied message.
pub(super) fn commit_staged(
    runner: &dyn ProcessRunner,
    worktree_path: &Path,
    message: &str,
) -> CoreResult<()> {
    let worktree_str = worktree_path.to_string_lossy();
    let request =
        ProcessRequest::new("git").args(["-C", worktree_str.as_ref(), "commit", "-m", message]);
    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot commit staged changes in coordination worktree '{worktree}'.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and '{worktree}' is a git worktree.",
            worktree = worktree_path.display(),
        ))
    })?;
    if output.success {
        return Ok(());
    }
    Err(CoreError::process(format!(
        "Cannot commit staged changes in coordination worktree '{worktree}'.\n\
         Git reported: {detail}\n\
         Fix: ensure git user.name and user.email are configured \
         (`git config --global user.email \"you@example.com\"`).",
        worktree = worktree_path.display(),
        detail = render_output(&output),
    )))
}

/// Stable FNV-1a hash used for a deterministic local project identifier.
pub(super) fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Prefer stderr, then stdout, when presenting a failed process result.
pub(super) fn render_output(output: &ProcessOutput) -> String {
    let stderr = output.stderr.trim();
    let stdout = output.stdout.trim();
    if !stderr.is_empty() {
        return stderr.to_string();
    }
    if !stdout.is_empty() {
        return stdout.to_string();
    }
    "no command output".to_string()
}

/// Return the repository-format-specific empty tree object ID.
pub(super) fn empty_tree_hash(
    runner: &dyn ProcessRunner,
    project_root: &Path,
) -> CoreResult<String> {
    let object_format = repository_object_format(runner, project_root)?;
    let hash = match object_format {
        GitObjectFormat::Sha1 => "4b825dc642cb6eb9a060e54bf8d69288fbee4904".to_string(),
        GitObjectFormat::Sha256 => hex::encode(Sha256::digest(b"tree 0\0")),
    };
    Ok(hash)
}

fn repository_object_format(
    runner: &dyn ProcessRunner,
    project_root: &Path,
) -> CoreResult<GitObjectFormat> {
    let output = runner.run(
        &ProcessRequest::new("git")
            .args(["rev-parse", "--show-object-format"])
            .current_dir(project_root),
    );
    let Ok(output) = output else {
        return Ok(GitObjectFormat::Sha1);
    };
    if !output.success {
        return Ok(GitObjectFormat::Sha1);
    }
    Ok(match output.stdout.trim() {
        "sha256" => GitObjectFormat::Sha256,
        _ => GitObjectFormat::Sha1,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GitObjectFormat {
    Sha1,
    Sha256,
}
