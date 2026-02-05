use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub version: String,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<WorkflowRequires>,

    #[serde(
        rename = "context_files",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_files: Option<Vec<String>>,

    pub waves: Vec<WaveDefinition>,

    #[serde(
        rename = "on_complete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_complete: Option<OnComplete>,
}

impl WorkflowDefinition {
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
pub struct WorkflowRequires {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnComplete {
    #[serde(
        rename = "update_state",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_state: Option<bool>,
    #[serde(
        rename = "update_roadmap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_roadmap: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveDefinition {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub tasks: Vec<TaskDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<bool>,
}

impl WaveDefinition {
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
pub struct TaskDefinition {
    pub id: String,
    pub name: String,
    pub agent: AgentType,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<BTreeMap<String, String>>,
}

impl TaskDefinition {
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
pub enum AgentType {
    Research,
    Execution,
    Review,
    Planning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Auto,
    Checkpoint,
    Decision,
}
