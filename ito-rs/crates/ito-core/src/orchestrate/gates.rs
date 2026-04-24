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
mod tests {
    use super::*;

    fn gate(name: &str, policy: GatePolicy) -> PlannedGate {
        PlannedGate {
            name: name.to_string(),
            policy,
        }
    }

    #[test]
    fn remediation_includes_failed_gate_and_downstream_run_gates() {
        let gates = vec![
            gate("apply-complete", GatePolicy::Run),
            gate("format", GatePolicy::Run),
            gate("tests", GatePolicy::Run),
            gate("code-review", GatePolicy::Run),
        ];

        let pkt = remediation_packet_for_failure("001-01_demo", &gates, "tests", "boom");
        assert_eq!(pkt.change_id, "001-01_demo");
        assert_eq!(pkt.failed_gate, "tests");
        assert_eq!(pkt.error, "boom");
        assert_eq!(
            pkt.rerun_gates,
            vec!["tests".to_string(), "code-review".to_string()]
        );
    }

    #[test]
    fn remediation_includes_failed_gate_even_when_policy_is_skip() {
        let gates = vec![
            gate("apply-complete", GatePolicy::Run),
            gate("tests", GatePolicy::Skip),
            gate("code-review", GatePolicy::Run),
        ];

        let pkt = remediation_packet_for_failure("001-01_demo", &gates, "tests", "boom");
        assert_eq!(
            pkt.rerun_gates,
            vec!["tests".to_string(), "code-review".to_string()]
        );
    }

    #[test]
    fn remediation_skips_downstream_skip_gates() {
        let gates = vec![
            gate("apply-complete", GatePolicy::Run),
            gate("tests", GatePolicy::Run),
            gate("style", GatePolicy::Skip),
            gate("code-review", GatePolicy::Run),
        ];

        let pkt = remediation_packet_for_failure("001-01_demo", &gates, "tests", "boom");
        assert_eq!(
            pkt.rerun_gates,
            vec!["tests".to_string(), "code-review".to_string()]
        );
    }

    #[test]
    fn remediation_returns_empty_when_failed_gate_not_found() {
        let gates = vec![
            gate("apply-complete", GatePolicy::Run),
            gate("tests", GatePolicy::Run),
        ];

        let pkt = remediation_packet_for_failure("001-01_demo", &gates, "unknown-gate", "boom");
        assert!(pkt.rerun_gates.is_empty());
    }
}
