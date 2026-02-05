//! Workflow schema.
//!
//! This module contains serde models for the workflow definition file.
//! Workflows are used to describe multi-wave execution plans for an agent tool.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A workflow definition.
///
/// This is the user-authored input that describes waves, tasks, and required
/// inputs.
pub struct WorkflowDefinition {
    /// Workflow schema version.
    pub version: String,

    /// Stable identifier for this workflow.
    pub id: String,

    /// Human-friendly name.
    pub name: String,

    /// Optional longer description.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    /// Optional prerequisites (files/variables) required to run this workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<WorkflowRequires>,

    /// Optional list of files that should be loaded into context.
    #[serde(
        rename = "context_files",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_files: Option<Vec<String>>,

    /// Ordered list of waves.
    pub waves: Vec<WaveDefinition>,

    /// Optional actions to take after the workflow completes.
    #[serde(
        rename = "on_complete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_complete: Option<OnComplete>,
}

impl WorkflowDefinition {
    /// Validate semantic invariants for the workflow.
    pub fn validate(&self) -> Result<(), String> {
        if self.version.trim().is_empty() {
            return Err("workflow.version must not be empty".to_string());
        }
        if self.id.trim().is_empty() {
            return Err("workflow.id must not be empty".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("workflow.name must not be empty".to_string());
        }
        if self.waves.is_empty() {
            return Err("workflow.waves must not be empty".to_string());
        }

        if let Some(requires) = &self.requires {
            if let Some(vars) = &requires.variables {
                for v in vars {
                    if v.trim().is_empty() {
                        return Err("workflow.requires.variables contains empty entry".to_string());
                    }
                }
            }
            if let Some(files) = &requires.files {
                for f in files {
                    if f.trim().is_empty() {
                        return Err("workflow.requires.files contains empty entry".to_string());
                    }
                }
            }
        }

        if let Some(files) = &self.context_files {
            for f in files {
                if f.trim().is_empty() {
                    return Err("workflow.context_files contains empty entry".to_string());
                }
            }
        }

        let mut seen_waves: Vec<&str> = Vec::new();
        for wave in &self.waves {
            wave.validate()?;
            if seen_waves.contains(&wave.id.as_str()) {
                return Err(format!("workflow.waves has duplicate id: {}", wave.id));
            }
            seen_waves.push(wave.id.as_str());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Prerequisites for running a workflow.
pub struct WorkflowRequires {
    /// Required files (paths) that must exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    /// Required variables that must be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Optional side effects after a workflow completes.
pub struct OnComplete {
    /// Whether to update the workflow state file.
    #[serde(
        rename = "update_state",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_state: Option<bool>,

    /// Whether to update the roadmap/task tracker.
    #[serde(
        rename = "update_roadmap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_roadmap: Option<bool>,

    /// Notification targets (tool-specific).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A wave within a workflow.
pub struct WaveDefinition {
    /// Unique wave id.
    pub id: String,

    /// Optional human-friendly name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tasks executed in this wave.
    pub tasks: Vec<TaskDefinition>,

    /// Optional checkpoint marker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<bool>,
}

impl WaveDefinition {
    /// Validate semantic invariants for the wave.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("wave.id must not be empty".to_string());
        }
        if let Some(name) = &self.name
            && name.trim().is_empty()
        {
            return Err(format!("wave.name must not be empty (wave {})", self.id));
        }
        if self.tasks.is_empty() {
            return Err(format!("wave.tasks must not be empty (wave {})", self.id));
        }

        let mut seen_tasks: Vec<&str> = Vec::new();
        for task in &self.tasks {
            task.validate()?;
            if seen_tasks.contains(&task.id.as_str()) {
                return Err(format!(
                    "wave.tasks has duplicate id: {} (wave {})",
                    task.id, self.id
                ));
            }
            seen_tasks.push(task.id.as_str());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A task within a wave.
pub struct TaskDefinition {
    /// Unique task id (stable within a workflow).
    pub id: String,

    /// Human-friendly task name.
    pub name: String,

    /// Agent category the task should run under.
    pub agent: AgentType,

    /// Prompt content for the task.
    pub prompt: String,

    /// Optional additional required inputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,

    /// Optional output artifact name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,

    /// Optional task type modifier.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskType>,

    /// Optional tool-specific context key/value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<BTreeMap<String, String>>,
}

impl TaskDefinition {
    /// Validate semantic invariants for the task.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("task.id must not be empty".to_string());
        }
        if self.name.trim().is_empty() {
            return Err(format!("task.name must not be empty (task {})", self.id));
        }
        if self.prompt.trim().is_empty() {
            return Err(format!("task.prompt must not be empty (task {})", self.id));
        }
        if let Some(inputs) = &self.inputs {
            for i in inputs {
                if i.trim().is_empty() {
                    return Err(format!(
                        "task.inputs contains empty entry (task {})",
                        self.id
                    ));
                }
            }
        }
        if let Some(out) = &self.output
            && out.trim().is_empty()
        {
            return Err(format!("task.output must not be empty (task {})", self.id));
        }
        if let Some(ctx) = &self.context {
            for (k, v) in ctx {
                if k.trim().is_empty() {
                    return Err(format!("task.context has empty key (task {})", self.id));
                }
                if v.trim().is_empty() {
                    return Err(format!(
                        "task.context has empty value for '{k}' (task {})",
                        self.id
                    ));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// High-level agent role used to route tasks.
pub enum AgentType {
    /// Research / discovery.
    Research,
    /// Execution / implementation.
    Execution,
    /// Review / validation.
    Review,
    /// Planning / proposal writing.
    Planning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Optional modifiers for how a task should be run.
pub enum TaskType {
    /// Automatic default behavior.
    Auto,
    /// A checkpoint (pause for review).
    Checkpoint,
    /// A decision point.
    Decision,
}
