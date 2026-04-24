//! Change worktree initialization: file copy-over and include-pattern resolution.
//!
//! Resolves include patterns from two sources:
//!
//! 1. `worktrees.init.include` in the Ito config — a list of glob patterns.
//! 2. `.worktree-include` file at the repo root — one glob per line, `#`-prefixed
//!    comment lines and blank lines are ignored.
//!
//! The union of both sources is used. Patterns are expanded against the source
//! worktree root and matched files are copied into the destination worktree.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use ito_config::types::WorktreeInitConfig;

use crate::errors::{CoreError, CoreResult};

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
fn expand_globs(patterns: &[String], source_root: &Path) -> CoreResult<Vec<PathBuf>> {
    let mut result = BTreeSet::new();

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
            if path.is_file() {
                if let Ok(rel) = path.strip_prefix(source_root) {
                    result.insert(rel.to_path_buf());
                }
            }
        }
    }

    Ok(result.into_iter().collect())
}

#[cfg(test)]
#[path = "worktree_init_tests.rs"]
mod worktree_init_tests;
