//! Workflow directory helpers and default workflow templates.
//!
//! Workflows are stored under `{ito_path}/workflows` and describe multi-wave
//! task execution.
//!
//! This module provides **pure** path helpers, template content, and parsing
//! utilities.  Filesystem I/O (init, list, load) lives in `ito-core`.

use crate::schemas::WorkflowDefinition;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn path_helpers_build_expected_locations() {
        let ito_path = Path::new("/tmp/project/.ito");
        assert_eq!(
            workflows_dir(ito_path),
            Path::new("/tmp/project/.ito/workflows")
        );
        assert_eq!(
            workflow_state_dir(ito_path),
            Path::new("/tmp/project/.ito/workflows/.state")
        );
        assert_eq!(
            commands_dir(ito_path),
            Path::new("/tmp/project/.ito/commands")
        );
        assert_eq!(
            workflow_file_path(ito_path, "research"),
            Path::new("/tmp/project/.ito/workflows/research.yaml")
        );
    }

    #[test]
    fn parse_and_count_tasks_from_yaml() {
        let yaml = r#"
version: "1"
id: research
name: Research Workflow
waves:
  - id: discover
    name: Discover
    tasks:
      - id: t1
        name: Read code
        agent: execution
        prompt: Read repository structure
      - id: t2
        name: Gather context
        agent: execution
        prompt: Gather design context
"#;

        let wf = parse_workflow(yaml).expect("workflow should parse");
        assert_eq!(count_tasks(&wf), 2);
    }

    #[test]
    fn parse_workflow_returns_error_for_invalid_yaml() {
        let err = parse_workflow("not: [valid").expect_err("invalid yaml should fail");
        assert!(!err.trim().is_empty());
    }
}
