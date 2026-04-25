use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/// Parsed `orchestrate.md` user prompt.
#[derive(Debug, Clone)]
pub struct OrchestrateUserPrompt {
    /// Path to the source file.
    pub path: PathBuf,
    /// Raw file contents.
    pub raw: String,
    /// Parsed YAML front matter.
    pub front_matter: OrchestrateUserPromptFrontMatter,
    /// Content under `## MUST`.
    pub must: String,
    /// Content under `## PREFER`.
    pub prefer: String,
    /// Content under `## Notes`.
    pub notes: String,
}

/// YAML front matter for `orchestrate.md`.
///
/// Unknown keys are captured in `extra`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrchestrateUserPromptFrontMatter {
    /// Built-in preset name to use (e.g. `rust`).
    #[serde(default)]
    pub preset: Option<String>,

    /// Parallelism selection.
    ///
    /// Accepts either a positive integer or a string alias (e.g. `auto`, `serial`, `parallel`).
    #[serde(default)]
    pub max_parallel: Option<serde_yaml::Value>,

    /// Failure policy label (e.g. `remediate`, `stop`, `continue`).
    #[serde(default)]
    pub failure_policy: Option<String>,

    /// Per-gate overrides.
    #[serde(default)]
    pub gate_overrides: BTreeMap<String, serde_yaml::Value>,

    /// Unrecognised keys.
    #[serde(flatten, default)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

/// One built-in orchestrator preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratePreset {
    /// Preset name.
    pub name: String,
    /// Gate execution order.
    #[serde(default)]
    pub gate_order: Vec<String>,
    /// Gate configuration by name.
    #[serde(default)]
    pub gates: BTreeMap<String, serde_yaml::Value>,
    /// Recommended skills.
    #[serde(default)]
    pub recommended_skills: Vec<String>,
    /// Agent role suggestions.
    #[serde(default)]
    pub agent_roles: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Run-level behavior when a gate fails.
pub enum FailurePolicy {
    /// Attempt remediation and continue the run.
    #[default]
    Remediate,
    /// Stop the run immediately.
    Stop,
    /// Continue to the next change/gate without remediation.
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// High-level run status persisted in `run.json`.
pub enum OrchestrateRunStatus {
    /// Run is in progress.
    Running,
    /// Run completed successfully.
    Complete,
    /// Run completed with failures.
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Terminal outcome for a gate execution.
pub enum GateOutcome {
    /// Gate completed successfully.
    Pass,
    /// Gate failed.
    Fail,
    /// Gate was skipped by policy.
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Whether a gate should run or be recorded as skipped.
pub enum GatePolicy {
    /// Gate should be executed.
    Run,
    /// Gate should be skipped and recorded as such.
    Skip,
}

#[derive(Debug, Clone)]
/// Resolved run configuration used to build a [`RunPlan`](crate::orchestrate::RunPlan).
pub struct OrchestrateRunConfig {
    /// Optional `max_parallel` selector from `orchestrate.md`.
    pub max_parallel: Option<serde_yaml::Value>,
    /// Maximum cap used when `max_parallel` resolves to `auto` / `parallel`.
    pub max_parallel_cap: usize,
    /// Optional failure policy override.
    pub failure_policy: Option<FailurePolicy>,
    /// Optional default gate order override for the run.
    pub gate_order: Vec<String>,
    /// Gate names that should be skipped for the run.
    pub skip_gates: BTreeSet<String>,
}

impl Default for OrchestrateRunConfig {
    fn default() -> Self {
        Self {
            max_parallel: None,
            max_parallel_cap: 4,
            failure_policy: None,
            gate_order: Vec::new(),
            skip_gates: BTreeSet::new(),
        }
    }
}

/// Resolve a `max_parallel` selector into an integer concurrency limit.
///
/// Accepts either a positive integer or a string alias:
/// - `serial`, `sync`, `synchronous` -> `1`
/// - `parallel`, `fan-out`, `swarm`, `distributed` -> `cap`
/// - `auto` or empty -> `cap`
pub fn parse_max_parallel(
    value: Option<serde_yaml::Value>,
    cap: usize,
) -> Result<usize, crate::errors::CoreError> {
    let Some(value) = value else {
        return Ok(cap);
    };

    match value {
        serde_yaml::Value::Number(number) => parse_max_parallel_number(number),
        serde_yaml::Value::String(s) => parse_max_parallel_str(&s, cap),
        serde_yaml::Value::Null
        | serde_yaml::Value::Bool(_)
        | serde_yaml::Value::Sequence(_)
        | serde_yaml::Value::Mapping(_)
        | serde_yaml::Value::Tagged(_) => Err(invalid_max_parallel_type()),
    }
}

fn parse_max_parallel_number(
    number: serde_yaml::Number,
) -> Result<usize, crate::errors::CoreError> {
    let Some(number) = number.as_u64() else {
        return Err(invalid_max_parallel_number());
    };
    let Ok(number) = usize::try_from(number) else {
        return Err(invalid_max_parallel_number());
    };
    if number == 0 {
        return Err(invalid_max_parallel_number());
    }

    Ok(number)
}

fn parse_max_parallel_str(input: &str, cap: usize) -> Result<usize, crate::errors::CoreError> {
    let input = input.trim().to_lowercase();
    if input.is_empty() || input == "auto" {
        return Ok(cap);
    }

    match input.as_str() {
        "serial" | "sync" | "synchronous" => Ok(1),
        "parallel" | "fan-out" | "swarm" | "distributed" => Ok(cap),
        _ => parse_explicit_max_parallel(&input, cap),
    }
}

fn parse_explicit_max_parallel(
    input: &str,
    _cap: usize,
) -> Result<usize, crate::errors::CoreError> {
    let Ok(input) = input.parse::<usize>() else {
        return Err(crate::errors::CoreError::Validation(format!(
            "invalid max_parallel value: '{input}'"
        )));
    };
    if input == 0 {
        return Err(crate::errors::CoreError::Validation(
            "invalid max_parallel value: '0'".to_string(),
        ));
    }

    Ok(input)
}

fn invalid_max_parallel_number() -> crate::errors::CoreError {
    crate::errors::CoreError::Validation("max_parallel must be a positive integer".to_string())
}

fn invalid_max_parallel_type() -> crate::errors::CoreError {
    crate::errors::CoreError::Validation(
        "max_parallel must be a positive integer or string alias".to_string(),
    )
}
