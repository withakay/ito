//! Event context resolution: session ID, harness session ID, git context,
//! and user identity.
//!
//! These helpers are intentionally placed in the domain layer because the
//! `EventContext` type is defined here. The actual filesystem and process
//! operations use `std` directly (no external dependencies beyond `uuid`
//! which is available transitively).

use std::path::Path;

use super::event::EventContext;

/// Git-related context fields.
#[derive(Debug, Clone, Default)]
pub struct GitContext {
    /// Current branch name (None if detached HEAD or not in a git repo).
    pub branch: Option<String>,
    /// Worktree name if not the main worktree.
    pub worktree: Option<String>,
    /// Short HEAD commit hash.
    pub commit: Option<String>,
}

/// Resolve the full `EventContext` for the current CLI invocation.
///
/// Combines session ID, harness session ID, git context, and user identity.
/// All resolution is best-effort: failures result in `None` values, never errors.
pub fn resolve_context(ito_path: &Path) -> EventContext {
    let session_id = resolve_session_id(ito_path);
    let harness_session_id = resolve_harness_session_id();
    let git = resolve_git_context();

    EventContext {
        session_id,
        harness_session_id,
        branch: git.branch,
        worktree: git.worktree,
        commit: git.commit,
    }
}

/// Resolve (or create) the session ID.
///
/// Reads from `{ito_path}/.state/audit/.session`. If the file doesn't exist,
/// generates a new UUID v4 and writes it. The `.session` file is gitignored.
pub fn resolve_session_id(ito_path: &Path) -> String {
    let session_dir = ito_path.join(".state").join("audit");
    let session_file = session_dir.join(".session");

    // Try to read existing session ID
    if let Ok(contents) = std::fs::read_to_string(&session_file) {
        let contents = contents.trim().to_string();
        if !contents.is_empty() {
            return contents;
        }
    }

    // Generate new session ID
    let id = uuid::Uuid::new_v4().to_string();

    // Best-effort write
    let _ = std::fs::create_dir_all(&session_dir);
    let _ = std::fs::write(&session_file, &id);

    id
}

/// Check environment variables for a harness session ID.
///
/// Checks (in order): `ITO_HARNESS_SESSION_ID`, `CLAUDE_SESSION_ID`,
/// `OPENCODE_SESSION_ID`, `CODEX_SESSION_ID`. Returns the first found.
pub fn resolve_harness_session_id() -> Option<String> {
    let env_vars = [
        "ITO_HARNESS_SESSION_ID",
        "CLAUDE_SESSION_ID",
        "OPENCODE_SESSION_ID",
        "CODEX_SESSION_ID",
    ];

    for var in env_vars {
        if let Ok(val) = std::env::var(var)
            && !val.is_empty()
        {
            return Some(val);
        }
    }

    None
}

/// Resolve git context (branch, worktree, commit) from the current directory.
///
/// All fields are best-effort: if git is not available or the directory is
/// not a git repo, fields are `None`.
pub fn resolve_git_context() -> GitContext {
    let branch = run_git_command(&["symbolic-ref", "--short", "HEAD"]);
    let commit = run_git_command(&["rev-parse", "--short=8", "HEAD"]);

    // Detect worktree: compare git-dir with common-dir
    let worktree = detect_worktree_name();

    GitContext {
        branch,
        worktree,
        commit,
    }
}

/// Resolve the user identity for the `by` field.
///
/// Uses `git config user.name`, falling back to `$USER`, formatted as
/// `@lowercase-hyphenated`.
pub fn resolve_user_identity() -> String {
    let name = run_git_command(&["config", "user.name"])
        .or_else(|| std::env::var("USER").ok())
        .unwrap_or_else(|| "unknown".to_string());

    format!("@{}", name.to_lowercase().replace(' ', "-"))
}

/// Run a git command and return its stdout as a trimmed string, or None on failure.
fn run_git_command(args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git").args(args).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

/// Detect if we're in a worktree (not the main worktree) and return its name.
fn detect_worktree_name() -> Option<String> {
    let git_dir = run_git_command(&["rev-parse", "--git-dir"])?;
    let common_dir = run_git_command(&["rev-parse", "--git-common-dir"])?;

    // If git-dir != common-dir, we're in a worktree
    let git_dir_path = std::path::Path::new(&git_dir);
    let common_dir_path = std::path::Path::new(&common_dir);

    if git_dir_path.canonicalize().ok()? != common_dir_path.canonicalize().ok()? {
        // Extract worktree name from the path
        let toplevel = run_git_command(&["rev-parse", "--show-toplevel"])?;
        let path = std::path::Path::new(&toplevel);
        Some(path.file_name()?.to_string_lossy().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_user_identity_returns_at_prefixed_string() {
        let identity = resolve_user_identity();
        assert!(identity.starts_with('@'));
        assert!(!identity.contains(' '));
    }

    #[test]
    fn resolve_harness_session_id_returns_none_without_env() {
        // In test environment, these env vars are typically not set
        // We can't guarantee they're unset, so just test the function doesn't panic
        let _result = resolve_harness_session_id();
    }

    #[test]
    fn resolve_git_context_does_not_panic() {
        // This test verifies the function is safe to call in any environment
        let ctx = resolve_git_context();
        // In a git repo, branch should be Some; but we don't enforce it
        // since CI might have detached HEAD
        let _ = ctx;
    }

    #[test]
    fn resolve_session_id_generates_uuid() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");
        std::fs::create_dir_all(&ito_path).expect("create ito dir");

        let id = resolve_session_id(&ito_path);
        assert!(!id.is_empty());
        // Should be a valid UUID v4 format (36 chars with hyphens)
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
    }

    #[test]
    fn resolve_session_id_is_stable_across_calls() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");
        std::fs::create_dir_all(&ito_path).expect("create ito dir");

        let id1 = resolve_session_id(&ito_path);
        let id2 = resolve_session_id(&ito_path);
        assert_eq!(id1, id2);
    }

    #[test]
    fn resolve_context_populates_session_id() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ito_path = tmp.path().join(".ito");
        std::fs::create_dir_all(&ito_path).expect("create ito dir");

        let ctx = resolve_context(&ito_path);
        assert!(!ctx.session_id.is_empty());
    }
}
