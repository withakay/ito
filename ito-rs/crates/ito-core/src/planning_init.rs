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

/// Adapter-ready summary for `ito plan status` rendering.
#[derive(Debug, Eq, PartialEq)]
pub struct PlanningWorkspaceSummary {
    /// Planning workspace availability label.
    pub planning_status: &'static str,
    /// Path to the planning workspace.
    pub planning_dir: PathBuf,
    /// Research workspace availability label.
    pub research_status: &'static str,
    /// Path to the research workspace.
    pub research_dir: PathBuf,
    /// Planning-specific notice to display before the documents section.
    pub planning_notice: Option<String>,
    /// Research-specific notice to display before the documents section.
    pub research_notice: Option<String>,
    /// Documents-section message when there is nothing to list.
    pub documents_notice: Option<String>,
    /// Prepared planning document names for rendering.
    pub document_names: Vec<String>,
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
    let planning = planning_dir(ito_path);
    std::fs::create_dir_all(&planning)
        .map_err(|e| workspace_io_error("creating planning workspace", &planning, e))?;
    Ok(())
}

/// Convert planning workspace status into adapter-ready rendering data.
pub fn summarize_planning_workspace(status: &PlanningWorkspaceStatus) -> PlanningWorkspaceSummary {
    let document_names = status
        .planning_documents
        .iter()
        .map(|document| {
            document
                .file_name()
                .unwrap_or_else(|| document.as_os_str())
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    PlanningWorkspaceSummary {
        planning_status: workspace_label(status.planning_exists, status.planning_invalid),
        planning_dir: status.planning_dir.clone(),
        research_status: workspace_label(status.research_exists, status.research_invalid),
        research_dir: status.research_dir.clone(),
        planning_notice: status.planning_invalid.then(|| {
            "Planning path is not a directory. Rename or remove it, then run `ito plan init`."
                .to_string()
        }),
        research_notice: status.research_invalid.then(|| {
            "Research path is not a directory. Rename or remove it before storing deep-dive research."
                .to_string()
        }),
        documents_notice: if status.planning_invalid {
            None
        } else if !status.planning_exists {
            Some("No planning workspace found. Run `ito plan init` to create one.".to_string())
        } else if status.planning_documents.is_empty() {
            Some("No planning documents yet. Use /ito-plan to create the first plan.".to_string())
        } else {
            None
        },
        document_names,
    }
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
            .map_err(|e| workspace_io_error("reading planning workspace", &planning, e))?;
        for entry in entries {
            let entry = entry.map_err(|e| {
                workspace_io_error("reading planning workspace entry", &planning, e)
            })?;
            let path = entry.path();
            let file_type = entry.file_type().map_err(|e| {
                workspace_io_error("reading planning workspace entry metadata", &path, e)
            })?;
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
        Err(err) => Err(workspace_io_error(
            format!("inspecting {label} workspace"),
            path,
            err,
        )),
    }
}

fn workspace_label(exists: bool, invalid: bool) -> &'static str {
    if exists {
        "available"
    } else if invalid {
        "invalid"
    } else {
        "missing"
    }
}

fn workspace_io_error(action: impl AsRef<str>, path: &Path, err: std::io::Error) -> CoreError {
    CoreError::io(
        format!(
            "{} at {} (check permissions and parent directories)",
            action.as_ref(),
            path.display()
        ),
        err,
    )
}
