//! Orchestrator run state persistence.
//!
//! Persists run state under `.ito/.state/orchestrate/runs/<run-id>/`.

use crate::errors::{CoreError, CoreResult};
use crate::orchestrate::plan::{PlannedGate, RunPlan};
use crate::orchestrate::types::{FailurePolicy, GateOutcome, OrchestrateRunStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Persisted `run.json` record for an orchestration run.
pub struct OrchestrateRun {
    /// Run id (`YYYYMMDD-HHMMSS-<short-uuid>`).
    pub run_id: String,
    /// RFC3339 timestamp for run start.
    pub started_at: String,
    /// RFC3339 timestamp for run completion.
    pub finished_at: Option<String>,
    /// Run status.
    pub status: OrchestrateRunStatus,
    /// Preset name used for this run.
    pub preset: String,
    /// Maximum number of concurrent change pipelines.
    pub max_parallel: usize,
    /// Failure policy applied to this run.
    pub failure_policy: FailurePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// One terminal gate result recorded for a change.
pub struct OrchestrateGateRecord {
    /// Gate identifier.
    pub gate: String,
    /// Gate outcome.
    pub outcome: GateOutcome,
    /// RFC3339 timestamp for gate completion.
    pub finished_at: String,
    #[serde(default)]
    /// Optional error payload (for failed gates).
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Persisted per-change state (`changes/<change-id>.json`).
pub struct OrchestrateChangeState {
    /// Canonical change id.
    pub change_id: String,
    #[serde(default)]
    /// Terminal gate outcomes observed so far.
    pub gates: Vec<OrchestrateGateRecord>,
    /// RFC3339 timestamp for last update.
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// One record appended to `events.jsonl`.
pub struct OrchestrateEvent {
    /// RFC3339 timestamp.
    pub ts: String,
    /// Event payload.
    pub kind: OrchestrateEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "event", rename_all = "kebab-case")]
/// Event kinds written to `events.jsonl`.
pub enum OrchestrateEventKind {
    /// Run started.
    RunStart {
        /// Run id.
        run_id: String,
        /// Preset name.
        preset: String,
        /// Maximum parallel pipelines.
        max_parallel: usize,
        /// Failure policy.
        failure_policy: FailurePolicy,
    },
    /// Run completed.
    RunComplete {
        /// Run id.
        run_id: String,
        /// Terminal status.
        status: OrchestrateRunStatus,
    },
    /// Gate started for a change.
    GateStart {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
    },
    /// Gate passed for a change.
    GatePass {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
    },
    /// Gate failed for a change.
    GateFail {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
        /// Error payload.
        error: String,
    },
    /// Gate skipped for a change.
    GateSkip {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
    },
    /// Worker dispatched.
    WorkerDispatch {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
        /// Suggested role label (preset-driven).
        role: String,
    },
    /// Worker completed.
    WorkerComplete {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name.
        gate: String,
        /// Worker-reported outcome.
        outcome: GateOutcome,
    },
    /// Remediation dispatched after a gate failure.
    RemediationDispatch {
        /// Run id.
        run_id: String,
        /// Change id.
        change_id: String,
        /// Gate name that failed.
        failed_gate: String,
    },
}

/// Initialize the on-disk run state directory and write `run.json` + `plan.json`.
pub fn init_orchestrate_run_state(
    ito_path: &Path,
    run: &OrchestrateRun,
    plan: &RunPlan,
) -> CoreResult<()> {
    let root = run_root(ito_path, &run.run_id);
    std::fs::create_dir_all(root.join("changes"))
        .map_err(|e| CoreError::io("create orchestrate run state directory", e))?;

    write_json_pretty(&root.join("run.json"), run)
        .map_err(|e| CoreError::io("write orchestrate run.json", e))?;
    write_json_pretty(&root.join("plan.json"), plan)
        .map_err(|e| CoreError::io("write orchestrate plan.json", e))?;

    // Ensure the event log exists.
    let events = root.join("events.jsonl");
    if !events.exists() {
        std::fs::write(&events, "")
            .map_err(|e| CoreError::io("create orchestrate events.jsonl", e))?;
    }

    Ok(())
}

/// Load the persisted `run.json` record for a previously started orchestrate run.
///
/// Use this when resuming a run or when an adapter needs to report the run's
/// current top-level status without re-reading the full event log.
pub fn load_orchestrate_run(ito_path: &Path, run_id: &str) -> CoreResult<OrchestrateRun> {
    let path = run_root(ito_path, run_id).join("run.json");
    read_json_file(&path, "orchestrate run.json")
}

/// Load the resolved `plan.json` for a previously started orchestrate run.
///
/// This is the canonical persisted plan used for resuming or inspecting a run,
/// so callers do not need to rebuild planning inputs from change metadata.
pub fn load_orchestrate_plan(ito_path: &Path, run_id: &str) -> CoreResult<RunPlan> {
    let path = run_root(ito_path, run_id).join("plan.json");
    read_json_file(&path, "orchestrate plan.json")
}

/// Append an event to `events.jsonl`.
pub fn append_orchestrate_event(
    ito_path: &Path,
    run_id: &str,
    event: &OrchestrateEvent,
) -> CoreResult<()> {
    let path = run_root(ito_path, run_id).join("events.jsonl");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| CoreError::io("create orchestrate events directory", e))?;
    }

    let json = serde_json::to_string(event).map_err(|e| CoreError::Serde {
        context: "serialize orchestrate event".to_string(),
        message: e.to_string(),
    })?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| {
            CoreError::io(
                format!("open orchestrate events.jsonl: {}", path.display()),
                e,
            )
        })?;
    writeln!(f, "{json}").map_err(|e| CoreError::io("append orchestrate event", e))?;
    f.flush()
        .map_err(|e| CoreError::io("flush orchestrate event log", e))?;
    Ok(())
}

/// Load the per-change state file if present.
pub fn load_orchestrate_change_state(
    ito_path: &Path,
    run_id: &str,
    change_id: &str,
) -> CoreResult<Option<OrchestrateChangeState>> {
    let path = change_state_path(ito_path, run_id, change_id);
    if !path.exists() {
        return Ok(None);
    }
    let state = read_json_file(&path, "orchestrate change state")?;
    Ok(Some(state))
}

/// Write the per-change state file.
pub fn write_orchestrate_change_state(
    ito_path: &Path,
    run_id: &str,
    state: &OrchestrateChangeState,
) -> CoreResult<()> {
    let path = change_state_path(ito_path, run_id, &state.change_id);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| CoreError::io("create orchestrate change state directory", e))?;
    }
    write_json_pretty(&path, state).map_err(|e| {
        CoreError::io(
            format!("write orchestrate change state: {}", path.display()),
            e,
        )
    })?;
    Ok(())
}

/// Generate a sortable run id in the form `YYYYMMDD-HHMMSS-<short-uuid>`.
pub fn generate_orchestrate_run_id(now: DateTime<Utc>) -> String {
    let ts = now.format("%Y%m%d-%H%M%S").to_string();
    let uuid = uuid::Uuid::new_v4().simple().to_string();
    let short = uuid.chars().take(8).collect::<String>();
    format!("{ts}-{short}")
}

/// Compute remaining gates for a change when resuming an interrupted run.
///
/// Gates that have terminal `pass` or `skip` outcomes are skipped.
/// Execution resumes from the first gate that is missing, failed, or otherwise incomplete.
pub fn remaining_gates_for_change(
    planned: &[PlannedGate],
    state: Option<&OrchestrateChangeState>,
) -> Vec<PlannedGate> {
    let Some(state) = state else {
        return planned.to_vec();
    };

    let mut outcomes: std::collections::BTreeMap<&str, GateOutcome> =
        std::collections::BTreeMap::new();
    for g in &state.gates {
        outcomes.insert(&g.gate, g.outcome);
    }

    let mut start = 0;
    for (i, g) in planned.iter().enumerate() {
        match outcomes.get(g.name.as_str()) {
            Some(GateOutcome::Pass) | Some(GateOutcome::Skip) => {
                start = i + 1;
                continue;
            }
            Some(GateOutcome::Fail) | None => {
                start = i;
                break;
            }
        }
    }

    planned[start..].to_vec()
}

fn run_root(ito_path: &Path, run_id: &str) -> PathBuf {
    ito_path
        .join(".state")
        .join("orchestrate")
        .join("runs")
        .join(run_id)
}

fn change_state_path(ito_path: &Path, run_id: &str, change_id: &str) -> PathBuf {
    run_root(ito_path, run_id)
        .join("changes")
        .join(format!("{change_id}.json"))
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, json)
}

fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path, label: &str) -> CoreResult<T> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| CoreError::io(format!("read {label}: {}", path.display()), e))?;
    serde_json::from_str(&contents).map_err(|e| CoreError::Serde {
        context: format!("parse {label}"),
        message: e.to_string(),
    })
}
