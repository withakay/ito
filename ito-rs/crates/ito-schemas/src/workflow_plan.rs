use crate::workflow::WorkflowDefinition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub tool: Tool,
    pub workflow: WorkflowDefinition,
    pub waves: Vec<WavePlan>,
}

impl ExecutionPlan {
    pub fn validate(&self) -> Result<(), String> {
        self.workflow.validate()?;

        for wave in &self.waves {
            wave.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WavePlan {
    pub wave_id: String,
    pub tasks: Vec<TaskPlan>,
}

impl WavePlan {
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
pub struct TaskPlan {
    pub task_id: String,
    pub model: String,
    pub context_budget: usize,
    pub prompt_content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<BTreeMap<String, String>>,
}

impl TaskPlan {
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
pub enum Tool {
    #[serde(rename = "opencode")]
    OpenCode,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "codex")]
    Codex,
}
