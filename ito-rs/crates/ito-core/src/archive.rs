//! Archive completed changes.
//!
//! Archiving moves a change directory into the archive area and can copy spec
//! deltas back into the main `specs/` tree.
//!
//! This module also includes a small helper for determining whether a
//! `tasks.md` file is fully complete.

use std::fs;
use std::path::Path;

use chrono::Utc;

use crate::error_bridge::IntoCoreResult;
use crate::errors::{CoreError, CoreResult};
use ito_common::fs::StdFs;
use ito_common::id::parse_change_id;
use ito_common::paths;
use ito_domain::modules::ModuleRepository as DomainModuleRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Summary of task completion for a change.
pub enum TaskStatus {
    /// The file contains no recognized tasks.
    NoTasks,
    /// All recognized tasks are complete.
    AllComplete,
    /// Some tasks are incomplete.
    HasIncomplete {
        /// Number of incomplete tasks.
        pending: usize,
        /// Total number of recognized tasks.
        total: usize,
    },
}

/// Check whether the tasks in `contents` are complete.
///
/// Supports both the checkbox task format (`- [ ]`, `- [x]`, `- [~]`, `- [>]`)
/// and the enhanced format (`- **Status**: [ ] pending`).
pub fn check_task_completion(contents: &str) -> TaskStatus {
    // Support both:
    // - checkbox tasks: "- [ ]" / "- [x]" / "- [~]" / "- [>]"
    // - enhanced tasks: "- **Status**: [ ] pending" / "- **Status**: [x] completed"
    let mut total = 0usize;
    let mut pending = 0usize;

    for raw in contents.lines() {
        let line = raw.trim();
        if line.starts_with("- [ ]") || line.starts_with("* [ ]") {
            total += 1;
            pending += 1;
            continue;
        }
        if line.starts_with("- [~]")
            || line.starts_with("- [>]")
            || line.starts_with("* [~]")
            || line.starts_with("* [>]")
        {
            total += 1;
            pending += 1;
            continue;
        }
        if line.starts_with("- [x]")
            || line.starts_with("- [X]")
            || line.starts_with("* [x]")
            || line.starts_with("* [X]")
        {
            total += 1;
            continue;
        }

        if line.starts_with("- **Status**:") || line.contains("**Status**:") {
            // Expect: - **Status**: [ ] pending OR - **Status**: [x] completed
            if line.contains("[ ]") {
                total += 1;
                pending += 1;
                continue;
            }
            if line.contains("[x]") || line.contains("[X]") {
                total += 1;
                continue;
            }
        }
    }

    if total == 0 {
        return TaskStatus::NoTasks;
    }
    if pending == 0 {
        return TaskStatus::AllComplete;
    }
    TaskStatus::HasIncomplete { pending, total }
}

/// List available change directories under `{ito_path}/changes`.
pub fn list_available_changes(ito_path: &Path) -> CoreResult<Vec<String>> {
    let fs = StdFs;
    ito_domain::discovery::list_change_dir_names(&fs, ito_path).into_core()
}

/// Return `true` if the change exists.
///
/// Existence is determined by presence of `{change}/proposal.md`.
pub fn change_exists(ito_path: &Path, change_name: &str) -> bool {
    if change_name.trim().is_empty() {
        return false;
    }
    let proposal = paths::change_dir(ito_path, change_name).join("proposal.md");
    proposal.exists()
}

/// Generate an archive folder name for `change_name`.
pub fn generate_archive_name(change_name: &str) -> String {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    format!("{date}-{change_name}")
}

/// Return `true` if `{ito_path}/changes/archive/{archive_name}` exists.
pub fn archive_exists(ito_path: &Path, archive_name: &str) -> bool {
    let dir = paths::changes_archive_dir(ito_path).join(archive_name);
    dir.exists()
}

/// Discover spec ids present under `{change}/specs/*/spec.md`.
pub fn discover_change_specs(ito_path: &Path, change_name: &str) -> CoreResult<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    let specs_dir = paths::change_specs_dir(ito_path, change_name);
    if !specs_dir.exists() {
        return Ok(out);
    }

    let entries = fs::read_dir(&specs_dir)
        .map_err(|e| CoreError::io(format!("reading {}", specs_dir.display()), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| CoreError::io("reading dir entry", e))?;
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if !is_dir {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.trim().is_empty() {
            continue;
        }
        let spec_md = entry.path().join("spec.md");
        if !spec_md.exists() {
            continue;
        }
        out.push(name);
    }

    out.sort();
    Ok(out)
}

/// Split spec ids into those already present in main specs and those that are new.
pub fn categorize_specs(ito_path: &Path, spec_names: &[String]) -> (Vec<String>, Vec<String>) {
    let mut new_specs: Vec<String> = Vec::new();
    let mut existing_specs: Vec<String> = Vec::new();
    for spec in spec_names {
        let dst = paths::spec_markdown_path(ito_path, spec);
        if dst.exists() {
            existing_specs.push(spec.clone());
        } else {
            new_specs.push(spec.clone());
        }
    }
    (new_specs, existing_specs)
}

/// Copy change spec deltas to the main specs tree.
///
/// Returns the list of spec ids that were written.
pub fn copy_specs_to_main(
    ito_path: &Path,
    change_name: &str,
    spec_names: &[String],
) -> CoreResult<Vec<String>> {
    let mut updated: Vec<String> = Vec::new();
    for spec in spec_names {
        let src = paths::change_specs_dir(ito_path, change_name)
            .join(spec)
            .join("spec.md");
        if !src.exists() {
            continue;
        }
        let dst_dir = paths::specs_dir(ito_path).join(spec);
        ito_common::io::create_dir_all_std(&dst_dir)
            .map_err(|e| CoreError::io(format!("creating spec dir {}", dst_dir.display()), e))?;
        let dst = dst_dir.join("spec.md");
        let md = ito_common::io::read_to_string_std(&src)
            .map_err(|e| CoreError::io(format!("reading spec {}", src.display()), e))?;
        ito_common::io::write_std(&dst, md)
            .map_err(|e| CoreError::io(format!("writing spec {}", dst.display()), e))?;
        updated.push(spec.clone());
    }
    Ok(updated)
}

fn mark_change_complete_in_module(
    module_repo: &impl DomainModuleRepository,
    ito_path: &Path,
    change_name: &str,
) {
    let Ok(parsed) = parse_change_id(change_name) else {
        return;
    };
    let module_id = parsed.module_id;
    let Ok(Some(resolved)) =
        crate::validate::resolve_module(module_repo, ito_path, module_id.as_str())
    else {
        return;
    };
    let Ok(md) = ito_common::io::read_to_string_std(&resolved.module_md) else {
        return;
    };

    let mut out = String::new();
    for line in md.lines() {
        if line.contains(change_name) {
            out.push_str(&line.replace("- [ ]", "- [x]"));
            out.push('\n');
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    let _ = ito_common::io::write_std(&resolved.module_md, out);
}

/// Move a change directory to the archive location.
pub fn move_to_archive(
    module_repo: &impl DomainModuleRepository,
    ito_path: &Path,
    change_name: &str,
    archive_name: &str,
) -> CoreResult<()> {
    let change_dir = paths::change_dir(ito_path, change_name);
    if !change_dir.exists() {
        return Err(CoreError::not_found(format!(
            "Change '{change_name}' not found"
        )));
    }

    let archive_root = paths::changes_archive_dir(ito_path);
    ito_common::io::create_dir_all_std(&archive_root)
        .map_err(|e| CoreError::io("creating archive directory", e))?;

    let dst = archive_root.join(archive_name);
    if dst.exists() {
        return Err(CoreError::validation(format!(
            "Archive target already exists: {}",
            dst.display()
        )));
    }

    mark_change_complete_in_module(module_repo, ito_path, change_name);

    fs::rename(&change_dir, &dst).map_err(|e| CoreError::io("moving change to archive", e))?;
    Ok(())
}
