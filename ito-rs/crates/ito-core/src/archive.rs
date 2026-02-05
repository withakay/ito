use std::fs;
use std::path::Path;

use chrono::Utc;
use miette::{Result, miette};

use ito_common::fs::StdFs;
use ito_common::id::parse_change_id;
use ito_common::paths;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    NoTasks,
    AllComplete,
    HasIncomplete { pending: usize, total: usize },
}

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

pub fn list_available_changes(ito_path: &Path) -> Result<Vec<String>> {
    let fs = StdFs;
    ito_domain::discovery::list_change_dir_names(&fs, ito_path)
}

pub fn change_exists(ito_path: &Path, change_name: &str) -> bool {
    if change_name.trim().is_empty() {
        return false;
    }
    let proposal = paths::change_dir(ito_path, change_name).join("proposal.md");
    proposal.exists()
}

pub fn generate_archive_name(change_name: &str) -> String {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    format!("{date}-{change_name}")
}

pub fn archive_exists(ito_path: &Path, archive_name: &str) -> bool {
    let dir = paths::changes_archive_dir(ito_path).join(archive_name);
    dir.exists()
}

pub fn discover_change_specs(ito_path: &Path, change_name: &str) -> Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    let specs_dir = paths::change_specs_dir(ito_path, change_name);
    if !specs_dir.exists() {
        return Ok(out);
    }

    let entries = fs::read_dir(&specs_dir)
        .map_err(|e| miette!("I/O error reading {}: {e}", specs_dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| miette!("I/O error reading dir entry: {e}"))?;
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

pub fn copy_specs_to_main(
    ito_path: &Path,
    change_name: &str,
    spec_names: &[String],
) -> Result<Vec<String>> {
    let mut updated: Vec<String> = Vec::new();
    for spec in spec_names {
        let src = paths::change_specs_dir(ito_path, change_name)
            .join(spec)
            .join("spec.md");
        if !src.exists() {
            continue;
        }
        let dst_dir = paths::specs_dir(ito_path).join(spec);
        ito_common::io::create_dir_all(&dst_dir)?;
        let dst = dst_dir.join("spec.md");
        let md = ito_common::io::read_to_string(&src)?;
        ito_common::io::write(&dst, md.as_bytes())?;
        updated.push(spec.clone());
    }
    Ok(updated)
}

fn mark_change_complete_in_module(ito_path: &Path, change_name: &str) {
    let Ok(parsed) = parse_change_id(change_name) else {
        return;
    };
    let module_id = parsed.module_id;
    let Ok(Some(resolved)) = crate::validate::resolve_module(ito_path, module_id.as_str()) else {
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

pub fn move_to_archive(ito_path: &Path, change_name: &str, archive_name: &str) -> Result<()> {
    let change_dir = paths::change_dir(ito_path, change_name);
    if !change_dir.exists() {
        return Err(miette!("Change '{change_name}' not found"));
    }

    let archive_root = paths::changes_archive_dir(ito_path);
    ito_common::io::create_dir_all(&archive_root)?;

    let dst = archive_root.join(archive_name);
    if dst.exists() {
        return Err(miette!("Archive target already exists: {}", dst.display()));
    }

    mark_change_complete_in_module(ito_path, change_name);

    fs::rename(&change_dir, &dst)
        .map_err(|e| miette!("I/O error moving change to archive: {e}"))?;
    Ok(())
}
