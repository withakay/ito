use crate::workflow::WorkflowDefinition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub workflow: WorkflowDefinition,
    pub status: ExecutionStatus,
    pub started_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub current_wave_index: usize,
    pub waves: Vec<WaveExecution>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub variables: BTreeMap<String, String>,
}

impl WorkflowExecution {
    pub fn validate(&self) -> Result<(), String> {
        self.workflow.validate()?;

        if self.started_at.trim().is_empty() {
            return Err("execution.started_at must not be empty".to_string());
        }
        if let Some(ts) = &self.completed_at
            && ts.trim().is_empty()
        {
            return Err("execution.completed_at must not be empty".to_string());
        }
        if !self.waves.is_empty() && self.current_wave_index >= self.waves.len() {
            return Err(format!(
                "execution.current_wave_index out of bounds: {} (len {})",
                self.current_wave_index,
                self.waves.len()
            ));
        }

        for wave in &self.waves {
            wave.validate()?;
        }
        for (k, v) in &self.variables {
            if k.trim().is_empty() {
                return Err("execution.variables has empty key".to_string());
            }
            if v.trim().is_empty() {
                return Err(format!("execution.variables has empty value for '{k}'"));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveExecution {
    pub wave: crate::workflow::WaveDefinition,
    pub status: ExecutionStatus,
    pub tasks: Vec<TaskExecution>,
}

impl WaveExecution {
    pub fn validate(&self) -> Result<(), String> {
        self.wave.validate()?;
        for task in &self.tasks {
            task.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskExecution {
    pub task: crate::workflow::TaskDefinition,
    pub status: ExecutionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_content: Option<String>,
}

impl TaskExecution {
    pub fn validate(&self) -> Result<(), String> {
        self.task.validate()?;
        if let Some(ts) = &self.started_at
            && ts.trim().is_empty()
        {
            return Err(format!(
                "execution.task.started_at must not be empty ({})",
                self.task.id
            ));
        }

        if let Some(ts) = &self.completed_at
            && ts.trim().is_empty()
        {
            return Err(format!(
                "execution.task.completed_at must not be empty ({})",
                self.task.id
            ));
        }

        if let Some(e) = &self.error
            && e.trim().is_empty()
        {
            return Err(format!(
                "execution.task.error must not be empty ({})",
                self.task.id
            ));
        }

        if let Some(out) = &self.output_content
            && out.trim().is_empty()
        {
            return Err(format!(
                "execution.task.output_content must not be empty ({})",
                self.task.id
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Complete,
    Failed,
    Skipped,
}
