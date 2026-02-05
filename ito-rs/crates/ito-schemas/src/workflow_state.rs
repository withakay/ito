//! Workflow execution state schema.
//!
//! This module contains serde models for tracking runtime workflow progress.

use crate::workflow::WorkflowDefinition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Runtime state for a workflow execution.
pub struct WorkflowExecution {
    /// The workflow definition being executed.
    pub workflow: WorkflowDefinition,

    /// High-level execution status.
    pub status: ExecutionStatus,

    /// Start timestamp (tool-defined format).
    pub started_at: String,

    /// Completion timestamp, if complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    /// Index of the currently active wave.
    pub current_wave_index: usize,

    /// Per-wave execution state.
    pub waves: Vec<WaveExecution>,

    /// Runtime variables captured during execution.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub variables: BTreeMap<String, String>,
}

impl WorkflowExecution {
    /// Validate semantic invariants for the execution state.
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
/// Execution state for a single wave.
pub struct WaveExecution {
    /// Wave definition.
    pub wave: crate::workflow::WaveDefinition,

    /// Wave execution status.
    pub status: ExecutionStatus,

    /// Per-task execution state.
    pub tasks: Vec<TaskExecution>,
}

impl WaveExecution {
    /// Validate semantic invariants for the wave execution.
    pub fn validate(&self) -> Result<(), String> {
        self.wave.validate()?;
        for task in &self.tasks {
            task.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Execution state for a single task.
pub struct TaskExecution {
    /// Task definition.
    pub task: crate::workflow::TaskDefinition,

    /// Task execution status.
    pub status: ExecutionStatus,

    /// Task start timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    /// Task completion timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    /// Error message, if failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Captured task output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_content: Option<String>,
}

impl TaskExecution {
    /// Validate semantic invariants for the task execution.
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
/// High-level execution status.
pub enum ExecutionStatus {
    /// Waiting to start.
    Pending,
    /// Currently running.
    Running,
    /// Completed successfully.
    Complete,
    /// Completed with failure.
    Failed,
    /// Skipped by the runner.
    Skipped,
}
