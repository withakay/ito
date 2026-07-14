//! Orchestrator configuration and shared helpers.

mod gates;
mod plan;
mod preset;
mod state;
mod types;
mod user_prompt;

pub use gates::{
    GATE_IMPLEMENTATION_READINESS, default_gate_order, ensure_implementation_readiness_first,
    remediation_packet_for_failure,
};
pub use plan::{ChangePlanInput, PlannedChange, PlannedGate, RunPlan, build_run_plan};
pub use preset::{list_orchestrate_presets, load_orchestrate_preset};
pub use state::{
    OrchestrateChangeState, OrchestrateEvent, OrchestrateEventKind, OrchestrateGateRecord,
    OrchestrateRun, append_orchestrate_event, generate_orchestrate_run_id,
    init_orchestrate_run_state, load_orchestrate_change_state, load_orchestrate_plan,
    load_orchestrate_run, orchestrate_readiness_gate_record, remaining_gates_for_change,
    write_orchestrate_change_state,
};
pub use types::{
    FailurePolicy, GateOutcome, GatePolicy, OrchestratePreset, OrchestrateRunConfig,
    OrchestrateRunStatus, OrchestrateUserPrompt, OrchestrateUserPromptFrontMatter,
    parse_max_parallel,
};
pub use user_prompt::load_orchestrate_user_prompt;
