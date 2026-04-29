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

use ito_config::ito_dir::lexical_normalize;

use ito_config::types::CoordinationStorage;

use crate::errors::{CoreError, CoreResult};

/// Subdirectories of `.ito/` that are wired to the coordination worktree.
pub const COORDINATION_DIRS: &[&str] = &["changes", "specs", "modules", "workflows", "audit"];

// ── Platform-abstracted symlink creation ─────────────────────────────────────

/// Create a directory symlink `dst` → `src` in a platform-appropriate way.
///
/// On Unix this calls [`std::os::unix::fs::symlink`].  On Windows this creates
/// an NTFS junction so standard users do not need Developer Mode or elevated
/// privileges. The function returns an
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
        junction::create(src, dst)
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
    match read_dir_link(path) {
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
/// 1. **Already a correct symlink** — ensure the target exists, then skip.
/// 2. **Wrong symlink** — fail with explicit actual/expected target guidance.
/// 3. **Real directory that is empty** — remove it and create the symlink.
/// 4. **Real directory with content** — fail so duplicate state is not merged
///    implicitly.
/// 5. **Does not exist** — create the symlink directly.
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
                lexical_normalize(&target)
            } else {
                // Symlink targets are relative to the directory containing the
                // link, i.e. `ito_path`.
                lexical_normalize(&ito_path.join(&target))
            };
            let resolved_dst = lexical_normalize(&dst);

            if resolved_target == resolved_dst || target == dst {
                // Already wired correctly. Recreate the target directory if it
                // was removed, repairing a broken-but-correct link.
                fs::create_dir_all(&dst).map_err(|e| {
                    CoreError::io(
                        format!(
                            "cannot create coordination directory '{}' for existing symlink '{}': ensure the worktree path is writable",
                            dst.display(),
                            src.display()
                        ),
                        e,
                    )
                })?;
                continue;
            }

            return Err(CoreError::process(format!(
                "Coordination symlink '{}' points to '{}' but should point to '{}'. Delete or move the wrong symlink manually, then run `ito init` again.",
                src.display(),
                resolved_target.display(),
                resolved_dst.display()
            )));
        } else if src.exists() {
            if !src.is_dir() {
                return Err(CoreError::process(format!(
                    "Coordination path '{}' exists but is not a directory or symlink. Move it aside, then run `ito init` again to wire '{}'.",
                    src.display(),
                    dst.display()
                )));
            }

            if !is_dir_empty(&src)? {
                return Err(CoreError::process(format!(
                    "Coordination path '{}' is a non-empty directory, not a symlink to '{}'. Move or merge its contents manually, remove the directory, then run `ito init` again.",
                    src.display(),
                    dst.display()
                )));
            }

            fs::remove_dir(&src).map_err(|e| {
                CoreError::io(
                    format!(
                        "cannot remove empty coordination directory '{}': remove it manually and retry",
                        src.display()
                    ),
                    e,
                )
            })?;
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

fn is_dir_empty(path: &Path) -> CoreResult<bool> {
    let mut entries = fs::read_dir(path).map_err(|e| {
        CoreError::io(
            format!(
                "cannot read coordination directory '{}' to verify it is empty: check filesystem permissions",
                path.display()
            ),
            e,
        )
    })?;

    Ok(entries.next().is_none())
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
            let target = read_dir_link(&from).map_err(|e| {
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
                junction::create(&target, &to).map_err(|e| {
                    CoreError::io(
                        format!(
                            "cannot recreate directory junction '{}' -> '{}' during recursive \
                             copy: ensure '{}' is writable",
                            to.display(),
                            target.display(),
                            dst.display()
                        ),
                        e,
                    )
                })?;
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
                move_entry_with_fallback(&from, &to, &link_path)?;
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
    /// One or more `.ito/<dir>` symlinks resolve, but point at the wrong target.
    WrongTargets {
        /// Each entry is `(link_path, actual_target, expected_target)`.
        mismatched: Vec<(PathBuf, PathBuf, PathBuf)>,
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
///    - If `<ito_path>/<dir>` is a symlink whose resolved target exists but is
///      not the expected `<worktree_ito_path>/<dir>`, record it as mismatched.
///    - If `<ito_path>/<dir>` is a real directory (not a symlink), record it as
///      not-wired.
/// 4. Return [`CoordinationHealthStatus::BrokenSymlinks`],
///    [`CoordinationHealthStatus::WrongTargets`], or
///    [`CoordinationHealthStatus::NotWired`] when problems are found, or
///    [`CoordinationHealthStatus::Healthy`] when everything looks good.
///
/// Note: broken symlinks take precedence over mismatched targets, which take
/// precedence over not-wired directories in the return value.
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
    let mut mismatched: Vec<(PathBuf, PathBuf, PathBuf)> = Vec::new();
    let mut not_wired: Vec<PathBuf> = Vec::new();

    for dir in COORDINATION_DIRS {
        let link_path = ito_path.join(dir);
        let expected_target = lexical_normalize(&worktree_ito_path.join(dir));

        if !link_path.exists() && fs::read_link(&link_path).is_err() {
            not_wired.push(link_path);
            continue;
        }

        match read_dir_link(&link_path) {
            Ok(target) => {
                // It is a symlink — check whether the target resolves.
                let resolved = if target.is_absolute() {
                    lexical_normalize(&target)
                } else {
                    lexical_normalize(&ito_path.join(&target))
                };
                if !resolved.exists() {
                    broken.push((link_path, target));
                } else if resolved != expected_target {
                    mismatched.push((link_path, resolved, expected_target));
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

    if !mismatched.is_empty() {
        return CoordinationHealthStatus::WrongTargets { mismatched };
    }

    if !not_wired.is_empty() {
        return CoordinationHealthStatus::NotWired { dirs: not_wired };
    }

    CoordinationHealthStatus::Healthy
}

fn move_entry_with_fallback(from: &Path, to: &Path, target_root: &Path) -> CoreResult<()> {
    let rename_err = match fs::rename(from, to) {
        Ok(()) => return Ok(()),
        Err(e) => e,
    };

    if rename_err.kind() != io::ErrorKind::CrossesDevices {
        return Err(CoreError::io(
            format!(
                "cannot move '{}' to '{}' during coordination teardown: ensure both paths are accessible and retry",
                from.display(),
                to.display()
            ),
            rename_err,
        ));
    }

    let file_type = fs::symlink_metadata(from).map_err(|e| {
        CoreError::io(
            format!(
                "cannot inspect '{}' during cross-filesystem coordination teardown",
                from.display()
            ),
            e,
        )
    })?;

    if file_type.file_type().is_symlink() {
        copy_symlink(from, to)?;
        remove_copied_symlink(from)?;
    } else if file_type.is_dir() {
        copy_dir_recursive(from, to)?;
        fs::remove_dir_all(from).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot remove source directory '{}' after cross-filesystem teardown copy: remove it manually and retry",
                    from.display()
                ),
                e,
            )
        })?;
    } else {
        fs::copy(from, to).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot copy '{}' to '{}' during coordination teardown: ensure '{}' is writable",
                    from.display(),
                    to.display(),
                    target_root.display()
                ),
                e,
            )
        })?;
        fs::remove_file(from).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot remove source file '{}' after cross-filesystem teardown copy: remove it manually and retry",
                    from.display()
                ),
                e,
            )
        })?;
    }

    Ok(())
}

fn copy_symlink(from: &Path, to: &Path) -> CoreResult<()> {
    let target = read_dir_link(from).map_err(|e| {
        CoreError::io(
            format!("cannot read symlink '{}' during copy", from.display()),
            e,
        )
    })?;

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target, to).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot recreate symlink '{}' -> '{}' during copy",
                    to.display(),
                    target.display()
                ),
                e,
            )
        })?;
    }

    #[cfg(windows)]
    {
        let _ = from;
        junction::create(&target, to).map_err(|e| {
            CoreError::io(
                format!(
                    "cannot recreate directory junction '{}' -> '{}' during copy",
                    to.display(),
                    target.display()
                ),
                e,
            )
        })?;
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (target, to);
        return Err(CoreError::io(
            "cannot recreate symlink on this platform",
            io::Error::new(io::ErrorKind::Unsupported, "symlinks unsupported"),
        ));
    }

    Ok(())
}

fn remove_copied_symlink(path: &Path) -> CoreResult<()> {
    remove_symlink_path(path).map_err(|e| {
        CoreError::io(
            format!(
                "cannot remove source symlink '{}' after copy: remove it manually and retry",
                path.display()
            ),
            e,
        )
    })
}

fn remove_symlink_path(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        junction::delete(path)
    }

    #[cfg(not(windows))]
    {
        fs::remove_file(path)
    }
}

fn read_dir_link(path: &Path) -> io::Result<PathBuf> {
    #[cfg(windows)]
    {
        junction::get_target(path)
    }

    #[cfg(not(windows))]
    {
        fs::read_link(path)
    }
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
        CoordinationHealthStatus::WrongTargets { mismatched } => {
            let lines: Vec<String> = mismatched
                .iter()
                .map(|(link, actual, expected)| {
                    format!(
                        "{} points to {} but should point to {}. \
                         Run `ito init` to repair.",
                        link.display(),
                        actual.display(),
                        expected.display()
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

#[cfg(test)]
#[path = "coordination_tests.rs"]
mod tests;
