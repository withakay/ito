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
