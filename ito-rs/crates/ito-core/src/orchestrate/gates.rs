use crate::orchestrate::plan::PlannedGate;
use crate::orchestrate::types::GatePolicy;

pub(crate) const GATE_APPLY_COMPLETE: &str = "apply-complete";
pub(crate) const GATE_FORMAT: &str = "format";
pub(crate) const GATE_LINT: &str = "lint";
pub(crate) const GATE_TESTS: &str = "tests";
pub(crate) const GATE_STYLE: &str = "style";
pub(crate) const GATE_CODE_REVIEW: &str = "code-review";
pub(crate) const GATE_SECURITY_REVIEW: &str = "security-review";

/// Default orchestrator gate order.
pub fn default_gate_order() -> Vec<String> {
    vec![
        GATE_APPLY_COMPLETE.to_string(),
        GATE_FORMAT.to_string(),
        GATE_LINT.to_string(),
        GATE_TESTS.to_string(),
        GATE_STYLE.to_string(),
        GATE_CODE_REVIEW.to_string(),
        GATE_SECURITY_REVIEW.to_string(),
    ]
}

/// Minimal remediation payload constructed after a gate failure.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RemediationPacket {
    /// Canonical change id.
    pub change_id: String,
    /// Gate that failed.
    pub failed_gate: String,
    /// Error payload captured from the gate.
    pub error: String,
    /// Gate names to rerun (failed gate + downstream run gates).
    pub rerun_gates: Vec<String>,
}

/// Construct a remediation packet for a specific gate failure.
pub fn remediation_packet_for_failure(
    change_id: &str,
    gates: &[PlannedGate],
    failed_gate: &str,
    error: &str,
) -> RemediationPacket {
    let mut rerun = Vec::new();
    let mut found = false;
    for gate in gates {
        if gate.name == failed_gate {
            found = true;
            // Always include the failed gate itself, even if its policy
            // was Skip — remediation must rerun what actually failed.
            rerun.push(gate.name.clone());
            continue;
        }
        if found && gate.policy == GatePolicy::Run {
            rerun.push(gate.name.clone());
        }
    }

    RemediationPacket {
        change_id: change_id.to_string(),
        failed_gate: failed_gate.to_string(),
        error: error.to_string(),
        rerun_gates: rerun,
    }
}

#[cfg(test)]
#[path = "gates_tests.rs"]
mod gates_tests;
