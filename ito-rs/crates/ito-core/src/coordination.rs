//! Symlink wiring for coordination worktrees.
//!
//! When Ito operates in a coordination-worktree layout, the canonical `.ito/`
//! subdirectories (`changes`, `specs`, `modules`, `workflows`, `audit`) are
//! replaced by symlinks that point into a shared coordination worktree.  This
//! module provides the helpers to create, verify, and tear down those symlinks,
//! as well as health-check utilities for detecting missing or broken setups.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use ito_config::types::CoordinationStorage;

use crate::errors::{CoreError, CoreResult};

/// Subdirectories of `.ito/` that are wired to the coordination worktree.
pub const COORDINATION_DIRS: &[&str] = &["changes", "specs", "modules", "workflows", "audit"];

// ── Platform-abstracted symlink creation ─────────────────────────────────────

/// Create a directory symlink `dst` → `src` in a platform-appropriate way.
///
/// On Unix this calls [`std::os::unix::fs::symlink`].  On Windows this calls
/// `std::os::windows::fs::symlink_dir`.  The function returns an
/// [`io::Error`] if the underlying OS call fails.
///
/// # Errors
///
/// Returns an error when the OS rejects the symlink creation, for example
/// because `dst` already exists or the caller lacks the required privileges.
pub fn create_dir_link(src: &Path, dst: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(src, dst)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (src, dst);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "directory symlinks are not supported on this platform",
        ))
    }
}

// ── Symlink target resolution ─────────────────────────────────────────────────

/// Return the resolved symlink target for `path`, or `None` if `path` is not a
/// symlink (including when `path` does not exist).
fn read_link_opt(path: &Path) -> io::Result<Option<PathBuf>> {
    match fs::read_link(path) {
        Ok(target) => Ok(Some(target)),
        // Path does not exist — not a symlink.
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        // Path exists but is not a symlink (EINVAL on Unix).
        Err(e) if e.kind() == io::ErrorKind::InvalidInput => Ok(None),
        // Some platforms return `Other` for non-symlink paths.
        Err(_) if path.exists() => Ok(None),
        Err(e) => Err(e),
    }
}

// ── wire_coordination_symlinks ────────────────────────────────────────────────

/// Wire `.ito/<dir>` → `<worktree_ito_path>/<dir>` for every coordination
/// directory.
///
/// For each directory in [`COORDINATION_DIRS`] the function applies the
/// following logic:
///
/// 1. **Already a correct symlink** — skip (idempotent).
/// 2. **Real directory with content** — create the target directory in the
///    worktree, move all entries across, remove the now-empty source directory,
///    then create the symlink.
/// 3. **Real directory that is empty** — remove it and create the symlink.
/// 4. **Does not exist** — create the symlink directly.
///
/// # Errors
///
/// Returns [`CoreError::Io`] when any filesystem operation fails, with a
/// message that includes the affected path and a suggested remediation.
pub fn wire_coordination_symlinks(ito_path: &Path, worktree_ito_path: &Path) -> CoreResult<()> {
    for dir in COORDINATION_DIRS {
        let src = ito_path.join(dir);
        let dst = worktree_ito_path.join(dir);

        // Check whether `src` is already a symlink pointing at `dst`.
        let existing_link = read_link_opt(&src).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot read symlink status of '{}': check filesystem permissions",
                    src.display()
                ),
                e,
            )
        })?;

        if let Some(target) = existing_link {
            // Resolve both paths for comparison so relative vs absolute doesn't
            // cause spurious mismatches.
            let resolved_target = if target.is_absolute() {
                target.clone()
            } else {
                // Symlink targets are relative to the directory containing the
                // link, i.e. `ito_path`.
                ito_path.join(&target)
            };
            let resolved_dst = if dst.is_absolute() {
                dst.clone()
            } else {
                std::env::current_dir()
                    .map_err(|e| CoreError::io("cannot determine current directory", e))?
                    .join(&dst)
            };

            if resolved_target == resolved_dst || target == dst {
                // Already wired correctly — nothing to do.
                continue;
            }

            // Symlink exists but points somewhere else — remove it so we can
            // re-create it pointing at the right place.
            fs::remove_file(&src).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot remove stale symlink '{}': delete it manually and retry",
                        src.display()
                    ),
                    e,
                )
            })?;
        } else if src.exists() {
            // `src` is a real directory (possibly with content).
            migrate_dir_to_worktree(&src, &dst)?;
        }
        // `src` does not exist at this point — create the symlink.

        // Ensure the target directory exists in the worktree.
        fs::create_dir_all(&dst).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot create coordination directory '{}': ensure the worktree path is \
                     writable",
                    dst.display()
                ),
                e,
            )
        })?;

        create_dir_link(&dst, &src).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot create symlink '{}' → '{}': on Linux/macOS ensure you have write \
                     permission to '{}'; on Windows you may need Developer Mode or elevated \
                     privileges",
                    src.display(),
                    dst.display(),
                    ito_path.display()
                ),
                e,
            )
        })?;
    }

    Ok(())
}

/// Move all entries from `src_dir` into `dst_dir`, then remove `src_dir`.
///
/// `dst_dir` is created if it does not already exist.
///
/// # Cross-filesystem moves
///
/// `fs::rename` is attempted first.  When the source and destination reside on
/// different filesystems the OS returns `EXDEV` ("Invalid cross-device link").
/// In that case the function falls back to a recursive copy followed by
/// deletion of the source, so migration works even when the XDG data directory
/// (or any other configured path) is on a separate partition or mount point.
fn migrate_dir_to_worktree(src_dir: &Path, dst_dir: &Path) -> CoreResult<()> {
    fs::create_dir_all(dst_dir).map_err(|e| {
        CoreError::io(
            format!(
                "cannot create target directory '{}' for content migration: ensure the \
                 worktree path is writable",
                dst_dir.display()
            ),
            e,
        )
    })?;

    let entries = fs::read_dir(src_dir).map_err(|e| {
        CoreError::io(
            format!(
                "cannot read directory '{}' for migration: check filesystem permissions",
                src_dir.display()
            ),
            e,
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            CoreError::io(
                format!(
                    "cannot read directory entry in '{}' during migration",
                    src_dir.display()
                ),
                e,
            )
        })?;
        let from = entry.path();
        let to = dst_dir.join(entry.file_name());

        let rename_err = match fs::rename(&from, &to) {
            Ok(()) => continue,
            Err(e) => e,
        };

        // Fall back to copy-then-delete when `rename` fails with EXDEV (cross-
        // device link), which happens when `src_dir` and `dst_dir` are on
        // different filesystems.
        let is_cross_device = rename_err.kind() == io::ErrorKind::CrossesDevices;
        if !is_cross_device {
            return Err(CoreError::io(
                format!(
                    "cannot move '{}' to '{}' during coordination migration: ensure both \
                     paths are accessible and retry",
                    from.display(),
                    to.display()
                ),
                rename_err,
            ));
        }

        let file_type = entry.file_type().map_err(|e| {
            CoreError::io(
                format!(
                    "cannot determine file type of '{}' during cross-filesystem migration",
                    from.display()
                ),
                e,
            )
        })?;

        if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
            fs::remove_dir_all(&from).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot remove source directory '{}' after cross-filesystem copy: \
                         remove it manually and retry",
                        from.display()
                    ),
                    e,
                )
            })?;
        } else {
            fs::copy(&from, &to).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot copy '{}' to '{}' during cross-filesystem migration: ensure \
                         '{}' is writable",
                        from.display(),
                        to.display(),
                        dst_dir.display()
                    ),
                    e,
                )
            })?;
            fs::remove_file(&from).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot remove source file '{}' after cross-filesystem copy: \
                         remove it manually and retry",
                        from.display()
                    ),
                    e,
                )
            })?;
        }
    }

    fs::remove_dir(src_dir).map_err(|e| {
        CoreError::io(
            format!(
                "cannot remove now-empty directory '{}' after migration: remove it manually \
                 and retry",
                src_dir.display()
            ),
            e,
        )
    })?;

    Ok(())
}

/// Recursively copy the directory tree rooted at `src` into `dst`.
///
/// `dst` is created if it does not already exist.  Files are copied with
/// [`fs::copy`], which preserves content but not all metadata (e.g. ownership
/// and extended attributes are not transferred).
///
/// # Errors
///
/// Returns [`CoreError::Io`] if any directory cannot be created, any file
/// cannot be read or written, or a directory entry cannot be inspected.
fn copy_dir_recursive(src: &Path, dst: &Path) -> CoreResult<()> {
    fs::create_dir_all(dst).map_err(|e| {
        CoreError::io(
            format!(
                "cannot create directory '{}' during recursive copy: ensure the target \
                 path is writable",
                dst.display()
            ),
            e,
        )
    })?;

    let entries = fs::read_dir(src).map_err(|e| {
        CoreError::io(
            format!(
                "cannot read directory '{}' during recursive copy: check filesystem \
                 permissions",
                src.display()
            ),
            e,
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            CoreError::io(
                format!(
                    "cannot read directory entry in '{}' during recursive copy",
                    src.display()
                ),
                e,
            )
        })?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        let file_type = entry.file_type().map_err(|e| {
            CoreError::io(
                format!(
                    "cannot determine file type of '{}' during recursive copy",
                    from.display()
                ),
                e,
            )
        })?;

        if file_type.is_symlink() {
            let target = fs::read_link(&from).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot read symlink '{}' during recursive copy",
                        from.display()
                    ),
                    e,
                )
            })?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&target, &to).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot recreate symlink '{}' -> '{}' during recursive copy: ensure \
                         '{}' is writable",
                        to.display(),
                        target.display(),
                        dst.display()
                    ),
                    e,
                )
            })?;
            #[cfg(windows)]
            {
                if from.is_dir() {
                    std::os::windows::fs::symlink_dir(&target, &to).map_err(|e| {
                        CoreError::io(
                            format!(
                                "cannot recreate directory symlink '{}' -> '{}' during \
                                 recursive copy: ensure '{}' is writable",
                                to.display(),
                                target.display(),
                                dst.display()
                            ),
                            e,
                        )
                    })?;
                } else {
                    std::os::windows::fs::symlink_file(&target, &to).map_err(|e| {
                        CoreError::io(
                            format!(
                                "cannot recreate file symlink '{}' -> '{}' during recursive \
                                 copy: ensure '{}' is writable",
                                to.display(),
                                target.display(),
                                dst.display()
                            ),
                            e,
                        )
                    })?;
                }
            }
        } else if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            fs::copy(&from, &to).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot copy '{}' to '{}' during recursive copy: ensure '{}' is \
                         writable",
                        from.display(),
                        to.display(),
                        dst.display()
                    ),
                    e,
                )
            })?;
        }
    }

    Ok(())
}

// ── update_gitignore_for_symlinks ─────────────────────────────────────────────

/// Append coordination-symlink entries to `<project_root>/.gitignore`.
///
/// The following block is added when it is not already present:
///
/// ```text
/// # Ito coordination worktree symlinks
/// .ito/changes
/// .ito/specs
/// .ito/modules
/// .ito/workflows
/// .ito/audit
/// ```
///
/// Entries that already appear anywhere in the file are not duplicated.
///
/// # Errors
///
/// Returns [`CoreError::Io`] when the `.gitignore` file cannot be read or
/// written, with the file path included in the message.
pub fn update_gitignore_for_symlinks(project_root: &Path) -> CoreResult<()> {
    let gitignore_path = project_root.join(".gitignore");

    let existing = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot read '{}': check filesystem permissions",
                    gitignore_path.display()
                ),
                e,
            )
        })?
    } else {
        String::new()
    };

    // Build the lines we want to ensure are present.
    let desired_lines: Vec<String> = COORDINATION_DIRS
        .iter()
        .map(|dir| format!(".ito/{dir}"))
        .collect();

    // Collect lines that are genuinely missing.
    let missing: Vec<&str> = desired_lines
        .iter()
        .filter(|line| !existing.lines().any(|l| l.trim() == line.as_str()))
        .map(String::as_str)
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    let mut content = existing;

    // Ensure there is a trailing newline before appending.
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    content.push_str("\n# Ito coordination worktree symlinks\n");
    for line in &missing {
        content.push_str(line);
        content.push('\n');
    }

    fs::write(&gitignore_path, &content).map_err(|e| {
        CoreError::io(
            format!(
                "cannot write '{}': check filesystem permissions",
                gitignore_path.display()
            ),
            e,
        )
    })?;

    Ok(())
}

// ── remove_coordination_symlinks ──────────────────────────────────────────────

/// Tear down coordination symlinks and restore real directories.
///
/// For each entry in [`COORDINATION_DIRS`], if `<ito_path>/<dir>` is a symlink
/// the function:
///
/// 1. Reads the content from the symlink target (`<worktree_ito_path>/<dir>`).
/// 2. Removes the symlink.
/// 3. Creates a real directory at `<ito_path>/<dir>`.
/// 4. Moves all content from the worktree directory back.
///
/// Entries that are already real directories (or do not exist) are left
/// untouched.
///
/// # Errors
///
/// Returns [`CoreError::Io`] when any filesystem operation fails, with a
/// message that includes the affected path and a suggested remediation.
pub fn remove_coordination_symlinks(ito_path: &Path, worktree_ito_path: &Path) -> CoreResult<()> {
    for dir in COORDINATION_DIRS {
        let link_path = ito_path.join(dir);
        let worktree_dir = worktree_ito_path.join(dir);

        let existing_link = read_link_opt(&link_path).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot read symlink status of '{}': check filesystem permissions",
                    link_path.display()
                ),
                e,
            )
        })?;

        let Some(_target) = existing_link else {
            // Not a symlink — nothing to restore.
            continue;
        };

        // Remove the symlink.
        fs::remove_file(&link_path).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot remove symlink '{}': delete it manually and retry",
                    link_path.display()
                ),
                e,
            )
        })?;

        // Create the real directory.
        fs::create_dir_all(&link_path).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot create directory '{}' after removing symlink: check filesystem \
                     permissions",
                    link_path.display()
                ),
                e,
            )
        })?;

        // Move content back from the worktree directory if it exists.
        if worktree_dir.exists() {
            let entries = fs::read_dir(&worktree_dir).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot read worktree directory '{}' during symlink teardown",
                        worktree_dir.display()
                    ),
                    e,
                )
            })?;

            for entry in entries {
                let entry = entry.map_err(|e| {
                    CoreError::io(
                        format!(
                            "cannot read directory entry in '{}' during teardown",
                            worktree_dir.display()
                        ),
                        e,
                    )
                })?;
                let from = entry.path();
                let to = link_path.join(entry.file_name());
                fs::rename(&from, &to).map_err(|e| {
                    CoreError::io(
                        format!(
                            "cannot move '{}' to '{}' during coordination teardown: ensure \
                             both paths are on the same filesystem or move them manually",
                            from.display(),
                            to.display()
                        ),
                        e,
                    )
                })?;
            }
        }
    }

    Ok(())
}

// ── check_coordination_health ─────────────────────────────────────────────────

/// Health status of the coordination worktree setup.
///
/// Returned by [`check_coordination_health`] to describe whether the
/// coordination symlinks and worktree directory are in a usable state.
#[derive(Debug, PartialEq)]
pub enum CoordinationHealthStatus {
    /// Everything is fine — worktree directory exists and all symlinks resolve.
    Healthy,
    /// Storage mode is `Embedded` — no worktree or symlinks are expected.
    Embedded,
    /// The coordination worktree directory is missing.
    WorktreeMissing {
        /// The path where the worktree was expected.
        expected_path: PathBuf,
    },
    /// One or more `.ito/<dir>` symlinks point at targets that do not exist.
    BrokenSymlinks {
        /// Each entry is `(link_path, target_path)` for a broken symlink.
        broken: Vec<(PathBuf, PathBuf)>,
    },
    /// One or more `.ito/<dir>` entries are real directories instead of symlinks.
    NotWired {
        /// Paths of the real directories that should be symlinks.
        dirs: Vec<PathBuf>,
    },
}

/// Inspect the coordination worktree and symlink state, returning a
/// [`CoordinationHealthStatus`] that describes any problem found.
///
/// # Check logic
///
/// 1. If `storage` is [`CoordinationStorage::Embedded`], return
///    [`CoordinationHealthStatus::Embedded`] immediately — no worktree is
///    expected in this mode.
/// 2. If the worktree directory at `worktree_ito_path` does not exist, return
///    [`CoordinationHealthStatus::WorktreeMissing`].
/// 3. For each of the five coordination directories (`changes`, `specs`,
///    `modules`, `workflows`, `audit`):
///    - If `<ito_path>/<dir>` is a symlink whose target does not exist, record
///      it as broken.
///    - If `<ito_path>/<dir>` is a real directory (not a symlink), record it as
///      not-wired.
/// 4. Return [`CoordinationHealthStatus::BrokenSymlinks`] or
///    [`CoordinationHealthStatus::NotWired`] when problems are found, or
///    [`CoordinationHealthStatus::Healthy`] when everything looks good.
///
/// Note: broken symlinks take precedence over not-wired directories in the
/// return value.  If both are present, `BrokenSymlinks` is returned.
pub fn check_coordination_health(
    ito_path: &Path,
    worktree_ito_path: &Path,
    storage: &CoordinationStorage,
) -> CoordinationHealthStatus {
    if *storage == CoordinationStorage::Embedded {
        return CoordinationHealthStatus::Embedded;
    }

    if !worktree_ito_path.exists() {
        return CoordinationHealthStatus::WorktreeMissing {
            expected_path: worktree_ito_path.to_path_buf(),
        };
    }

    let mut broken: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut not_wired: Vec<PathBuf> = Vec::new();

    for dir in COORDINATION_DIRS {
        let link_path = ito_path.join(dir);

        match fs::read_link(&link_path) {
            Ok(target) => {
                // It is a symlink — check whether the target resolves.
                let resolved = if target.is_absolute() {
                    target.clone()
                } else {
                    ito_path.join(&target)
                };
                if !resolved.exists() {
                    broken.push((link_path, target));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
                // Path exists but is not a symlink (EINVAL on Unix).
                if link_path.exists() {
                    not_wired.push(link_path);
                }
            }
            Err(_) => {
                // On some platforms a non-symlink path returns a different
                // error kind.  Treat an existing path as not-wired.
                if link_path.exists() {
                    not_wired.push(link_path);
                }
                // If the path does not exist at all, it is neither broken nor
                // not-wired — skip it.
            }
        }
    }

    if !broken.is_empty() {
        return CoordinationHealthStatus::BrokenSymlinks { broken };
    }

    if !not_wired.is_empty() {
        return CoordinationHealthStatus::NotWired { dirs: not_wired };
    }

    CoordinationHealthStatus::Healthy
}

/// Return an actionable error message for an unhealthy coordination state, or
/// `None` when the status is [`CoordinationHealthStatus::Healthy`] or
/// [`CoordinationHealthStatus::Embedded`].
///
/// Messages follow the **What / Why / How** pattern so that both humans and
/// AI agents can act on them immediately.
pub fn format_health_message(status: &CoordinationHealthStatus) -> Option<String> {
    match status {
        CoordinationHealthStatus::Healthy => None,
        CoordinationHealthStatus::Embedded => None,
        CoordinationHealthStatus::WorktreeMissing { expected_path } => Some(format!(
            "Coordination worktree not found at {}. \
             Run `ito init` to set it up.",
            expected_path.display()
        )),
        CoordinationHealthStatus::BrokenSymlinks { broken } => {
            let lines: Vec<String> = broken
                .iter()
                .map(|(link, target)| {
                    format!(
                        "Broken symlink: {} → {} (target does not exist). \
                         Run `ito init` to repair.",
                        link.display(),
                        target.display()
                    )
                })
                .collect();
            Some(lines.join("\n"))
        }
        CoordinationHealthStatus::NotWired { dirs } => {
            let lines: Vec<String> = dirs
                .iter()
                .map(|dir| {
                    format!(
                        "{} is a regular directory, not a symlink to the coordination worktree. \
                         Run `ito init` to wire symlinks.",
                        dir.display()
                    )
                })
                .collect();
            Some(lines.join("\n"))
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_dirs() -> (TempDir, PathBuf, PathBuf) {
        let tmp = TempDir::new().expect("tempdir");
        let ito = tmp.path().join(".ito");
        let worktree_ito = tmp.path().join("worktree").join(".ito");
        fs::create_dir_all(&ito).unwrap();
        fs::create_dir_all(&worktree_ito).unwrap();
        (tmp, ito, worktree_ito)
    }

    // ── create_dir_link ───────────────────────────────────────────────────────

    #[test]
    #[cfg(unix)]
    fn create_dir_link_creates_symlink() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("real_dir");
        let dst = tmp.path().join("link");
        fs::create_dir_all(&src).unwrap();

        create_dir_link(&src, &dst).expect("symlink creation should succeed");

        assert!(dst.exists(), "link path should resolve");
        let target = fs::read_link(&dst).expect("should be a symlink");
        assert_eq!(target, src);
    }

    #[test]
    #[cfg(unix)]
    fn create_dir_link_fails_when_dst_exists() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("real_dir");
        let dst = tmp.path().join("existing");
        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(&dst).unwrap();

        let result = create_dir_link(&src, &dst);
        assert!(result.is_err(), "should fail when dst already exists");
    }

    // ── wire_coordination_symlinks ────────────────────────────────────────────

    #[test]
    #[cfg(unix)]
    fn wire_creates_symlinks_for_all_dirs() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

        for dir in COORDINATION_DIRS {
            let link = ito.join(dir);
            assert!(link.exists(), "link '{dir}' should exist");
            let target = fs::read_link(&link).expect("should be a symlink");
            assert_eq!(
                target,
                worktree_ito.join(dir),
                "link '{dir}' points at wrong target"
            );
        }
    }

    #[test]
    #[cfg(unix)]
    fn wire_is_idempotent() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("first wire");
        wire_coordination_symlinks(&ito, &worktree_ito).expect("second wire should be idempotent");

        for dir in COORDINATION_DIRS {
            let link = ito.join(dir);
            let target = fs::read_link(&link).expect("should still be a symlink");
            assert_eq!(target, worktree_ito.join(dir));
        }
    }

    #[test]
    #[cfg(unix)]
    fn wire_migrates_real_dir_content() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // Pre-populate `.ito/changes` with a file.
        let changes_dir = ito.join("changes");
        fs::create_dir_all(&changes_dir).unwrap();
        let sentinel = changes_dir.join("sentinel.md");
        fs::write(&sentinel, "hello").unwrap();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

        // The symlink should now exist.
        let link = ito.join("changes");
        assert!(fs::read_link(&link).is_ok(), "changes should be a symlink");

        // The sentinel file should have been moved to the worktree.
        let migrated = worktree_ito.join("changes").join("sentinel.md");
        assert!(migrated.exists(), "sentinel.md should be in the worktree");
        assert_eq!(fs::read_to_string(&migrated).unwrap(), "hello");

        // And it should be accessible through the symlink.
        let via_link = link.join("sentinel.md");
        assert!(
            via_link.exists(),
            "sentinel.md should be accessible via symlink"
        );
    }

    #[test]
    #[cfg(unix)]
    fn wire_handles_empty_real_dir() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // Pre-create an empty `.ito/specs` directory.
        let specs_dir = ito.join("specs");
        fs::create_dir_all(&specs_dir).unwrap();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire should succeed");

        let link = ito.join("specs");
        assert!(fs::read_link(&link).is_ok(), "specs should be a symlink");
    }

    // ── update_gitignore_for_symlinks ─────────────────────────────────────────

    #[test]
    fn gitignore_entries_added_when_missing() {
        let tmp = TempDir::new().unwrap();
        let project_root = tmp.path();

        update_gitignore_for_symlinks(project_root).expect("should succeed");

        let content = fs::read_to_string(project_root.join(".gitignore")).unwrap();
        for dir in COORDINATION_DIRS {
            assert!(
                content.contains(&format!(".ito/{dir}")),
                ".gitignore should contain .ito/{dir}"
            );
        }
    }

    #[test]
    fn gitignore_no_duplicates_on_second_call() {
        let tmp = TempDir::new().unwrap();
        let project_root = tmp.path();

        update_gitignore_for_symlinks(project_root).expect("first call");
        update_gitignore_for_symlinks(project_root).expect("second call");

        let content = fs::read_to_string(project_root.join(".gitignore")).unwrap();
        for dir in COORDINATION_DIRS {
            let entry = format!(".ito/{dir}");
            let count = content.lines().filter(|l| l.trim() == entry).count();
            assert_eq!(
                count, 1,
                ".ito/{dir} should appear exactly once, found {count}"
            );
        }
    }

    #[test]
    fn gitignore_preserves_existing_content() {
        let tmp = TempDir::new().unwrap();
        let project_root = tmp.path();
        let gitignore_path = project_root.join(".gitignore");

        fs::write(&gitignore_path, "target/\n*.log\n").unwrap();

        update_gitignore_for_symlinks(project_root).expect("should succeed");

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(
            content.contains("target/"),
            "existing entry should be preserved"
        );
        assert!(
            content.contains("*.log"),
            "existing entry should be preserved"
        );
    }

    #[test]
    fn gitignore_skips_already_present_entries() {
        let tmp = TempDir::new().unwrap();
        let project_root = tmp.path();
        let gitignore_path = project_root.join(".gitignore");

        // Pre-populate with some of the entries.
        let pre = ".ito/changes\n.ito/specs\n";
        fs::write(&gitignore_path, pre).unwrap();

        update_gitignore_for_symlinks(project_root).expect("should succeed");

        let content = fs::read_to_string(&gitignore_path).unwrap();
        let changes_count = content
            .lines()
            .filter(|l| l.trim() == ".ito/changes")
            .count();
        assert_eq!(changes_count, 1, ".ito/changes should not be duplicated");
        let specs_count = content.lines().filter(|l| l.trim() == ".ito/specs").count();
        assert_eq!(specs_count, 1, ".ito/specs should not be duplicated");
        // The remaining entries should have been added.
        assert!(content.contains(".ito/modules"));
        assert!(content.contains(".ito/workflows"));
        assert!(content.contains(".ito/audit"));
    }

    #[test]
    fn gitignore_created_when_absent() {
        let tmp = TempDir::new().unwrap();
        let project_root = tmp.path();

        assert!(!project_root.join(".gitignore").exists());
        update_gitignore_for_symlinks(project_root).expect("should succeed");
        assert!(project_root.join(".gitignore").exists());
    }

    // ── remove_coordination_symlinks ──────────────────────────────────────────

    #[test]
    #[cfg(unix)]
    fn remove_restores_real_dirs_with_content() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // Wire symlinks first.
        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");

        // Write a file through the symlink (ends up in worktree).
        let via_link = ito.join("changes").join("task.md");
        fs::write(&via_link, "task content").unwrap();

        // Tear down.
        remove_coordination_symlinks(&ito, &worktree_ito).expect("remove");

        // `.ito/changes` should now be a real directory.
        let changes = ito.join("changes");
        assert!(changes.is_dir(), "changes should be a real directory");
        assert!(
            fs::read_link(&changes).is_err(),
            "changes should not be a symlink"
        );

        // Content should have been moved back.
        let restored = changes.join("task.md");
        assert!(restored.exists(), "task.md should be restored");
        assert_eq!(fs::read_to_string(&restored).unwrap(), "task content");
    }

    #[test]
    #[cfg(unix)]
    fn remove_is_noop_for_real_dirs() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // Create real directories (no symlinks).
        for dir in COORDINATION_DIRS {
            fs::create_dir_all(ito.join(dir)).unwrap();
        }

        // remove_coordination_symlinks should not touch real directories.
        remove_coordination_symlinks(&ito, &worktree_ito).expect("remove");

        for dir in COORDINATION_DIRS {
            let path = ito.join(dir);
            assert!(path.is_dir(), "{dir} should still be a real directory");
            assert!(
                fs::read_link(&path).is_err(),
                "{dir} should not be a symlink"
            );
        }
    }

    #[test]
    #[cfg(unix)]
    fn remove_is_noop_when_dirs_absent() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // No directories exist — remove should succeed without error.
        remove_coordination_symlinks(&ito, &worktree_ito).expect("remove on empty ito dir");
    }

    // ── check_coordination_health ─────────────────────────────────────────────

    #[test]
    fn health_embedded_returns_embedded() {
        let tmp = TempDir::new().unwrap();
        let ito = tmp.path().join(".ito");
        let worktree_ito = tmp.path().join("worktree").join(".ito");
        fs::create_dir_all(&ito).unwrap();
        // Deliberately do NOT create worktree_ito — it should not matter.

        let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Embedded);
        assert_eq!(status, CoordinationHealthStatus::Embedded);
    }

    #[test]
    fn health_worktree_missing_when_dir_absent() {
        let tmp = TempDir::new().unwrap();
        let ito = tmp.path().join(".ito");
        let worktree_ito = tmp.path().join("nonexistent").join(".ito");
        fs::create_dir_all(&ito).unwrap();

        let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);
        assert_eq!(
            status,
            CoordinationHealthStatus::WorktreeMissing {
                expected_path: worktree_ito.clone()
            }
        );
    }

    #[test]
    #[cfg(unix)]
    fn health_healthy_when_all_symlinks_correct() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");

        let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);
        assert_eq!(status, CoordinationHealthStatus::Healthy);
    }

    #[test]
    #[cfg(unix)]
    fn health_broken_symlinks_when_target_missing() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        wire_coordination_symlinks(&ito, &worktree_ito).expect("wire");

        // Remove the worktree target for "changes" so the symlink is broken.
        let target = worktree_ito.join("changes");
        fs::remove_dir_all(&target).unwrap();

        let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

        let CoordinationHealthStatus::BrokenSymlinks { broken } = status else {
            panic!("expected BrokenSymlinks, got {status:?}");
        };
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].0, ito.join("changes"));
    }

    #[test]
    #[cfg(unix)]
    fn health_not_wired_when_real_dirs_present() {
        let (_tmp, ito, worktree_ito) = make_dirs();

        // Create real directories instead of symlinks.
        for dir in COORDINATION_DIRS {
            fs::create_dir_all(ito.join(dir)).unwrap();
        }

        let status = check_coordination_health(&ito, &worktree_ito, &CoordinationStorage::Worktree);

        let CoordinationHealthStatus::NotWired { dirs } = status else {
            panic!("expected NotWired, got {status:?}");
        };
        assert_eq!(dirs.len(), COORDINATION_DIRS.len());
    }

    // ── format_health_message ─────────────────────────────────────────────────

    #[test]
    fn format_message_healthy_is_none() {
        assert!(format_health_message(&CoordinationHealthStatus::Healthy).is_none());
    }

    #[test]
    fn format_message_embedded_is_none() {
        assert!(format_health_message(&CoordinationHealthStatus::Embedded).is_none());
    }

    #[test]
    fn format_message_worktree_missing_contains_path_and_hint() {
        let path = PathBuf::from("/some/path/.ito");
        let msg = format_health_message(&CoordinationHealthStatus::WorktreeMissing {
            expected_path: path.clone(),
        })
        .expect("should produce a message");

        assert!(
            msg.contains(&path.display().to_string()),
            "message should contain the expected path"
        );
        assert!(
            msg.contains("ito init"),
            "message should mention `ito init`"
        );
    }

    #[test]
    fn format_message_broken_symlinks_contains_paths_and_hint() {
        let link = PathBuf::from("/project/.ito/changes");
        let target = PathBuf::from("../worktree/.ito/changes");
        let msg = format_health_message(&CoordinationHealthStatus::BrokenSymlinks {
            broken: vec![(link.clone(), target.clone())],
        })
        .expect("should produce a message");

        assert!(
            msg.contains(&link.display().to_string()),
            "message should contain the link path"
        );
        assert!(
            msg.contains(&target.display().to_string()),
            "message should contain the target path"
        );
        assert!(
            msg.contains("ito init"),
            "message should mention `ito init`"
        );
    }

    #[test]
    fn format_message_not_wired_contains_dir_and_hint() {
        let dir = PathBuf::from("/project/.ito/specs");
        let msg = format_health_message(&CoordinationHealthStatus::NotWired {
            dirs: vec![dir.clone()],
        })
        .expect("should produce a message");

        assert!(
            msg.contains(&dir.display().to_string()),
            "message should contain the directory path"
        );
        assert!(
            msg.contains("ito init"),
            "message should mention `ito init`"
        );
    }
}
