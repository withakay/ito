//! Planning directory initialization.
//!
//! This module owns the filesystem I/O for bootstrapping the planning area.
//! Pure path helpers remain in `ito_domain::planning`.

use crate::errors::{CoreError, CoreResult};
use ito_domain::planning::{planning_dir, research_dir};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Current state of the flexible planning workspace.
#[derive(Debug, Eq, PartialEq)]
pub struct PlanningWorkspaceStatus {
    /// Path to the `.ito/planning/` workspace.
    pub planning_dir: PathBuf,
    /// Whether the planning workspace directory exists.
    pub planning_exists: bool,
    /// Whether the planning path exists but is not a directory.
    pub planning_invalid: bool,
    /// Markdown planning documents currently present directly under the workspace.
    pub planning_documents: Vec<PathBuf>,
    /// Path to the companion `.ito/research/` workspace.
    pub research_dir: PathBuf,
    /// Whether the companion research workspace exists.
    pub research_exists: bool,
    /// Whether the research path exists but is not a directory.
    pub research_invalid: bool,
}

struct WorkspacePathState {
    exists: bool,
    invalid: bool,
}

/// Initialize the planning directory structure under `ito_path`.
///
/// This is safe to call multiple times; existing planning documents are left unchanged.
///
/// # Errors
///
/// Returns an error if the planning workspace cannot be created.
pub fn init_planning_structure(ito_path: &Path) -> CoreResult<()> {
    std::fs::create_dir_all(planning_dir(ito_path))
        .map_err(|e| CoreError::io("creating planning workspace", e))?;
    Ok(())
}

/// Inspect the flexible planning workspace.
///
/// # Errors
///
/// Returns an error if the planning workspace exists but cannot be read.
pub fn read_planning_workspace_status(ito_path: &Path) -> CoreResult<PlanningWorkspaceStatus> {
    let planning = planning_dir(ito_path);
    let planning_state = inspect_workspace_path(&planning, "planning")?;
    let mut planning_documents = Vec::new();

    if planning_state.exists {
        let entries = std::fs::read_dir(&planning)
            .map_err(|e| CoreError::io("reading planning workspace", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| CoreError::io("reading planning workspace entry", e))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|e| CoreError::io("reading planning workspace entry metadata", e))?;
            let is_markdown = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
            if is_markdown && file_type.is_file() {
                planning_documents.push(path);
            }
        }
        planning_documents.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    let research = research_dir(ito_path);
    let research_state = inspect_workspace_path(&research, "research")?;

    Ok(PlanningWorkspaceStatus {
        planning_dir: planning,
        planning_exists: planning_state.exists,
        planning_invalid: planning_state.invalid,
        planning_documents,
        research_dir: research,
        research_exists: research_state.exists,
        research_invalid: research_state.invalid,
    })
}

fn inspect_workspace_path(path: &Path, label: &str) -> CoreResult<WorkspacePathState> {
    match std::fs::metadata(path) {
        Ok(metadata) => {
            let is_dir = metadata.is_dir();
            Ok(WorkspacePathState {
                exists: is_dir,
                invalid: !is_dir,
            })
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(WorkspacePathState {
            exists: false,
            invalid: false,
        }),
        Err(err) => Err(CoreError::io(format!("inspecting {label} workspace"), err)),
    }
}
