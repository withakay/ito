//! Change worktree initialization: file copy-over, setup command execution,
//! and include-pattern resolution.
//!
//! Resolves include patterns from two sources:
//!
//! 1. `worktrees.init.include` in the Ito config — a list of glob patterns.
//! 2. `.worktree-include` file at the repo root — one glob per line, `#`-prefixed
//!    comment lines and blank lines are ignored.
//!
//! The union of both sources is used. Patterns are expanded against the source
//! worktree root and matched files are copied into the destination worktree.
//!
//! After file copy, an optional setup command (or list of commands) from
//! `worktrees.init.setup` is executed inside the new worktree.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use ito_config::types::{WorktreeInitConfig, WorktreesConfig};

use crate::errors::{CoreError, CoreResult};
use crate::process::{ProcessRequest, ProcessRunner, SystemProcessRunner};

/// Name of the optional include-file at the repo root.
const WORKTREE_INCLUDE_FILE: &str = ".worktree-include";

/// Resolve all include patterns from config and the `.worktree-include` file,
/// then expand them against `source_root` and return the set of matching
/// relative paths (deduplicated, sorted).
///
/// Missing `.worktree-include` file is not an error — only config patterns are
/// used. Empty patterns are silently ignored.
pub fn resolve_include_files(
    config: &WorktreeInitConfig,
    source_root: &Path,
) -> CoreResult<Vec<PathBuf>> {
    let patterns = collect_patterns(config, source_root)?;
    expand_globs(&patterns, source_root)
}

/// Parse a `.worktree-include` file into a list of non-empty, non-comment lines.
///
/// Lines starting with `#` (after trimming) and blank lines are ignored.
pub fn parse_worktree_include_file(content: &str) -> Vec<String> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect()
}

/// Copy matched include files from `source_root` into `dest_root`, preserving
/// relative paths. Existing destination files are overwritten. Missing source
/// files are silently skipped.
///
/// Returns the list of files that were actually copied.
pub fn copy_include_files(
    config: &WorktreeInitConfig,
    source_root: &Path,
    dest_root: &Path,
) -> CoreResult<Vec<PathBuf>> {
    let relative_paths = resolve_include_files(config, source_root)?;
    let mut copied = Vec::new();

    for rel_path in &relative_paths {
        let src = source_root.join(rel_path);
        let dst = dest_root.join(rel_path);

        if !src.exists() {
            continue;
        }

        // Ensure parent directory exists in the destination.
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                CoreError::io(
                    format!(
                        "Cannot create directory '{}' in worktree '{}' for include file '{}'.\n\
                         Fix: ensure the worktree path is writable.",
                        parent.display(),
                        dest_root.display(),
                        rel_path.display(),
                    ),
                    err,
                )
            })?;
        }

        fs::copy(&src, &dst).map_err(|err| {
            CoreError::io(
                format!(
                    "Cannot copy '{}' to '{}' during worktree initialization.\n\
                     Fix: ensure the source file is readable and the destination is writable.",
                    src.display(),
                    dst.display(),
                ),
                err,
            )
        })?;

        copied.push(rel_path.clone());
    }

    Ok(copied)
}

/// Initialize a new change worktree: copy include files, then run setup commands.
///
/// This is the full initialization sequence called after a worktree is created:
///
/// 1. Copy include files from `source_root` into `dest_root` (idempotent).
/// 2. Run setup commands from `config.init.setup` in `dest_root` (if configured).
///
/// Coordination symlink wiring is handled separately by the caller because it
/// requires the `.ito/` path context which this function does not own.
///
/// # Errors
///
/// Returns [`CoreError`] if file copy fails or a setup command exits non-zero.
pub fn init_worktree(
    source_root: &Path,
    dest_root: &Path,
    config: &WorktreesConfig,
) -> CoreResult<()> {
    let runner = SystemProcessRunner;
    init_worktree_with_runner(&runner, source_root, dest_root, config)
}

/// Testable inner implementation of [`init_worktree`].
pub(crate) fn init_worktree_with_runner(
    runner: &dyn ProcessRunner,
    source_root: &Path,
    dest_root: &Path,
    config: &WorktreesConfig,
) -> CoreResult<()> {
    // Step 1: Copy include files.
    copy_include_files(&config.init, source_root, dest_root)?;

    // Step 2: Run setup commands (if any).
    run_setup_with_runner(runner, dest_root, config)?;

    Ok(())
}

/// Run the configured setup command(s) inside `worktree_root`.
///
/// Each command is executed as `sh -c <command>` with `worktree_root` as the
/// working directory. Commands run in order; if any exits non-zero, subsequent
/// commands are skipped and an error is returned.
///
/// Returns `Ok(())` when no setup is configured or all commands succeed.
pub fn run_worktree_setup(worktree_root: &Path, config: &WorktreesConfig) -> CoreResult<()> {
    let runner = SystemProcessRunner;
    run_setup_with_runner(&runner, worktree_root, config)
}

/// Testable inner implementation of [`run_worktree_setup`].
pub(crate) fn run_setup_with_runner(
    runner: &dyn ProcessRunner,
    worktree_root: &Path,
    config: &WorktreesConfig,
) -> CoreResult<()> {
    let Some(setup) = &config.init.setup else {
        return Ok(());
    };

    if setup.is_empty() {
        return Ok(());
    }

    let commands = setup.as_commands();

    for cmd in &commands {
        let request = ProcessRequest::new("sh")
            .arg("-c")
            .arg(*cmd)
            .current_dir(worktree_root);

        let output = runner.run(&request).map_err(|err| {
            CoreError::process(format!(
                "Cannot run worktree setup command '{}' in '{}'.\n\
                 Process failed to start: {err}\n\
                 Fix: ensure the command exists and the worktree path is accessible.",
                cmd,
                worktree_root.display(),
            ))
        })?;

        if !output.success {
            let detail = if !output.stderr.trim().is_empty() {
                output.stderr.trim().to_string()
            } else if !output.stdout.trim().is_empty() {
                output.stdout.trim().to_string()
            } else {
                format!("exit code {}", output.exit_code)
            };

            return Err(CoreError::process(format!(
                "Worktree setup command failed: '{}'\n\
                 Working directory: {}\n\
                 Output: {detail}\n\
                 Fix: verify the command works when run manually in that directory.",
                cmd,
                worktree_root.display(),
            )));
        }
    }

    Ok(())
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Collect patterns from both config and the `.worktree-include` file.
fn collect_patterns(config: &WorktreeInitConfig, source_root: &Path) -> CoreResult<Vec<String>> {
    let mut patterns: Vec<String> = config.include.clone();

    let include_path = source_root.join(WORKTREE_INCLUDE_FILE);
    if include_path.is_file() {
        let content = fs::read_to_string(&include_path).map_err(|err| {
            CoreError::io(
                format!(
                    "Cannot read '{}' in '{}'.\n\
                     Fix: ensure the file is readable or remove it.",
                    WORKTREE_INCLUDE_FILE,
                    source_root.display(),
                ),
                err,
            )
        })?;
        let file_patterns = parse_worktree_include_file(&content);
        patterns.extend(file_patterns);
    }

    Ok(patterns)
}

/// Expand glob patterns against `source_root` and return deduplicated relative paths.
///
/// All resolved paths are canonicalized and verified to remain under
/// `source_root`. Patterns that resolve outside the source root (e.g. via
/// `..` components) are silently skipped to prevent path-traversal attacks.
fn expand_globs(patterns: &[String], source_root: &Path) -> CoreResult<Vec<PathBuf>> {
    let mut result = BTreeSet::new();

    // Canonicalize source_root so that symlink-based escapes are caught.
    let canonical_root = source_root.canonicalize().map_err(|err| {
        CoreError::io(
            format!(
                "Cannot canonicalize source root '{}'.\n\
                 Fix: ensure the directory exists and is readable.",
                source_root.display(),
            ),
            err,
        )
    })?;

    for pattern in patterns {
        if pattern.is_empty() {
            continue;
        }

        let full_pattern = source_root.join(pattern);
        let full_pattern_str = full_pattern.to_string_lossy();

        let entries = glob::glob(&full_pattern_str).map_err(|err| {
            CoreError::validation(format!(
                "Invalid glob pattern '{}' in worktree include configuration.\n\
                 Pattern error: {err}\n\
                 Fix: use valid glob syntax (e.g. '*.env', '.envrc', 'config/*.toml').",
                pattern,
            ))
        })?;

        for entry in entries {
            let path = entry.map_err(|err| {
                CoreError::io(
                    format!(
                        "Error reading filesystem while expanding glob '{}'.\n\
                         Fix: ensure the source directory '{}' is readable.",
                        pattern,
                        source_root.display(),
                    ),
                    err.into_error(),
                )
            })?;

            // Only include files, not directories.
            if !path.is_file() {
                continue;
            }

            // Canonicalize the matched path and verify it is under source_root.
            // This prevents path-traversal via `..` or symlinks.
            let Ok(canonical_path) = path.canonicalize() else {
                continue;
            };
            let Ok(rel) = canonical_path.strip_prefix(&canonical_root) else {
                continue;
            };

            result.insert(rel.to_path_buf());
        }
    }

    Ok(result.into_iter().collect())
}

#[cfg(test)]
#[path = "worktree_init_tests.rs"]
mod worktree_init_tests;
