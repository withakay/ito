use std::path::{Path, PathBuf};

use crate::errors::{CoreError, CoreResult};

/// Collect proposal artifacts for a change into a single markdown document.
pub fn collect_proposal_artifacts(change_id: &str, ito_root: &Path) -> CoreResult<String> {
    let change_dir = ito_common::paths::change_dir(ito_root, change_id);
    if !change_dir.is_dir() {
        return Err(CoreError::not_found(format!(
            "Change '{change_id}' not found"
        )));
    }

    let mut sections = Vec::new();

    for relative_path in artifact_paths(&change_dir)? {
        let absolute_path = change_dir.join(&relative_path);
        let content = ito_common::io::read_to_string(&absolute_path).map_err(|e| {
            CoreError::io(
                format!("reading proposal artifact {}", absolute_path.display()),
                std::io::Error::other(e),
            )
        })?;
        sections.push(render_section(&relative_path, &content));
    }

    Ok(sections.join("\n\n"))
}

fn artifact_paths(change_dir: &Path) -> CoreResult<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for file_name in ["proposal.md", "tasks.md"] {
        let path = change_dir.join(file_name);
        if path.is_file() {
            paths.push(PathBuf::from(file_name));
        }
    }

    let specs_dir = change_dir.join("specs");
    if specs_dir.is_dir() {
        let mut spec_dirs = Vec::new();
        let entries = std::fs::read_dir(&specs_dir)
            .map_err(|e| CoreError::io(format!("reading {}", specs_dir.display()), e))?;
        for entry in entries {
            let entry =
                entry.map_err(|e| CoreError::io(format!("reading {}", specs_dir.display()), e))?;
            if entry.path().is_dir() {
                spec_dirs.push(entry);
            }
        }
        spec_dirs.sort_by_key(|entry| entry.file_name());

        for entry in spec_dirs {
            let relative_path = PathBuf::from("specs")
                .join(entry.file_name())
                .join("spec.md");
            let absolute_path = change_dir.join(&relative_path);
            if absolute_path.is_file() {
                paths.push(relative_path);
            }
        }
    }

    Ok(paths)
}

fn render_section(relative_path: &Path, content: &str) -> String {
    format!(
        "---\n# {}\n\n{}",
        relative_path.to_string_lossy(),
        content.trim_end()
    )
}

#[cfg(test)]
#[path = "collector_tests.rs"]
mod collector_tests;
