//! Execution plan schema.
//!
//! A plan is derived from a workflow definition by choosing concrete models,
//! context budgets, and prompt content for each task.

use crate::workflow::WorkflowDefinition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A concrete execution plan for a workflow.
pub struct ExecutionPlan {
    /// The target tool/harness that will run the workflow.
    pub tool: Tool,

    /// The workflow definition this plan was generated from.
    pub workflow: WorkflowDefinition,

    /// Planned waves (in order).
    pub waves: Vec<WavePlan>,
}

impl ExecutionPlan {
    /// Validate semantic invariants for the execution plan.
    pub fn validate(&self) -> Result<(), String> {
        self.workflow.validate()?;

        for wave in &self.waves {
            wave.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Planned execution for a single wave.
pub struct WavePlan {
    /// The referenced wave id from the workflow definition.
    pub wave_id: String,

    /// Tasks to run in this wave.
    pub tasks: Vec<TaskPlan>,
}

impl WavePlan {
    /// Validate semantic invariants for the wave plan.
    pub fn validate(&self) -> Result<(), String> {
        if self.wave_id.trim().is_empty() {
            return Err("plan.wave_id must not be empty".to_string());
        }
        for task in &self.tasks {
            task.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Planned execution for a single task.
pub struct TaskPlan {
    /// The referenced task id from the workflow definition.
    pub task_id: String,

    /// Concrete model identifier (tool-specific).
    pub model: String,

    /// Context token budget.
    pub context_budget: usize,

    /// Fully-rendered prompt content.
    pub prompt_content: String,

    /// Optional input names expected by the prompt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,

    /// Optional output artifact name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,

    /// Optional tool-specific context key/value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<BTreeMap<String, String>>,
}

impl TaskPlan {
    /// Validate semantic invariants for the task plan.
    pub fn validate(&self) -> Result<(), String> {
        if self.task_id.trim().is_empty() {
            return Err("plan.task_id must not be empty".to_string());
        }
        if self.model.trim().is_empty() {
            return Err(format!(
                "plan.model must not be empty (task {})",
                self.task_id
            ));
        }
        if self.prompt_content.trim().is_empty() {
            return Err(format!(
                "plan.prompt_content must not be empty (task {})",
                self.task_id
            ));
        }
        if let Some(inputs) = &self.inputs {
            for i in inputs {
                if i.trim().is_empty() {
                    return Err(format!(
                        "plan.inputs contains empty entry (task {})",
                        self.task_id
                    ));
                }
            }
        }
        if let Some(out) = &self.output
            && out.trim().is_empty()
        {
            return Err(format!(
                "plan.output must not be empty (task {})",
                self.task_id
            ));
        }
        if let Some(ctx) = &self.context {
            for (k, v) in ctx {
                if k.trim().is_empty() {
                    return Err(format!(
                        "plan.context has empty key (task {})",
                        self.task_id
                    ));
                }
                if v.trim().is_empty() {
                    return Err(format!(
                        "plan.context has empty value for '{k}' (task {})",
                        self.task_id
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Tool/harness selector for an execution plan.
pub enum Tool {
    /// OpenCode.
    #[serde(rename = "opencode")]
    OpenCode,
    /// Claude Code.
    #[serde(rename = "claude-code")]
    ClaudeCode,
    /// Codex.
    #[serde(rename = "codex")]
    Codex,
}
