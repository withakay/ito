//! Workflow directory helpers and default workflow templates.
//!
//! Workflows are stored under `{ito_path}/workflows` and describe multi-wave
//! task execution.
//!
//! This module provides **pure** path helpers, template content, and parsing
//! utilities.  Filesystem I/O (init, list, load) lives in `ito-core`.

use ito_schemas::WorkflowDefinition;
use std::path::{Path, PathBuf};

/// Path to the workflows directory (`{ito_path}/workflows`).
pub fn workflows_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("workflows")
}

/// Path to the workflow state directory (`{ito_path}/workflows/.state`).
pub fn workflow_state_dir(ito_path: &Path) -> PathBuf {
    workflows_dir(ito_path).join(".state")
}

/// Path to the commands directory (`{ito_path}/commands`).
pub fn commands_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("commands")
}

/// Path to a specific workflow file (`{ito_path}/workflows/{name}.yaml`).
pub fn workflow_file_path(ito_path: &Path, name: &str) -> PathBuf {
    workflows_dir(ito_path).join(format!("{name}.yaml"))
}

/// Parse a workflow definition from YAML content.
pub fn parse_workflow(contents: &str) -> Result<WorkflowDefinition, String> {
    serde_yaml::from_str::<WorkflowDefinition>(contents).map_err(|e| e.to_string())
}

/// Count the number of tasks across all waves in the workflow.
pub fn count_tasks(wf: &WorkflowDefinition) -> usize {
    wf.waves.iter().map(|w| w.tasks.len()).sum()
}

// Workflow templates (research, execute, review) and I/O functions
// (init_workflow_structure, list_workflows, load_workflow) live in
// `ito_core::workflow_templates`.
