use crate::orchestrate::plan::PlannedGate;
use crate::orchestrate::types::GatePolicy;

/// Non-skippable gate proving a dispatch checkout is based on accepted main.
pub const GATE_IMPLEMENTATION_READINESS: &str = "implementation-readiness";
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
        GATE_IMPLEMENTATION_READINESS.to_string(),
        GATE_APPLY_COMPLETE.to_string(),
        GATE_FORMAT.to_string(),
        GATE_LINT.to_string(),
        GATE_TESTS.to_string(),
        GATE_STYLE.to_string(),
        GATE_CODE_REVIEW.to_string(),
        GATE_SECURITY_REVIEW.to_string(),
    ]
}

/// Return a gate order with implementation readiness present exactly once at the front.
pub fn ensure_implementation_readiness_first(gates: &[String]) -> Vec<String> {
    let mut ordered = gates.to_vec();
    ordered.retain(|gate| gate != GATE_IMPLEMENTATION_READINESS);
    ordered.insert(0, GATE_IMPLEMENTATION_READINESS.to_string());
    ordered
}

pub(crate) fn ensure_planned_implementation_readiness_first(
    gates: &[PlannedGate],
) -> Vec<PlannedGate> {
    let mut ordered = gates.to_vec();
    ordered.retain(|gate| gate.name != GATE_IMPLEMENTATION_READINESS);
    ordered.insert(
        0,
        PlannedGate {
            name: GATE_IMPLEMENTATION_READINESS.to_string(),
            policy: GatePolicy::Run,
        },
    );
    ordered
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
    /// Gate names to rerun (readiness + failed gate + downstream run gates).
    pub rerun_gates: Vec<String>,
}

/// Construct a remediation packet for a specific gate failure.
pub fn remediation_packet_for_failure(
    change_id: &str,
    gates: &[PlannedGate],
    failed_gate: &str,
    error: &str,
) -> RemediationPacket {
    let gates = ensure_planned_implementation_readiness_first(gates);
    let failed_index = gates.iter().position(|gate| gate.name == failed_gate);
    let rerun = match failed_index {
        Some(failed_index) => {
            let mut rerun = vec![GATE_IMPLEMENTATION_READINESS.to_string()];
            for gate in gates.iter().skip(failed_index) {
                if gate.name == GATE_IMPLEMENTATION_READINESS {
                    continue;
                }
                if gate.name == failed_gate || gate.policy == GatePolicy::Run {
                    rerun.push(gate.name.clone());
                }
            }
            rerun
        }
        None => Vec::new(),
    };

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
