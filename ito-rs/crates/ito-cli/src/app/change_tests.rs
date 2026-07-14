use ito_config::types::ProposalIntegrationMode;
use ito_core::implementation_readiness::{
    AuthorityEvidence, ReadinessCondition, ReadinessPhase, ReadinessReport,
};

use super::render_readiness_text;

#[test]
fn text_report_preserves_unresolved_authority_and_remediation() {
    let report = ReadinessReport {
        change_id: "031-02_enforce-main-first-implementation".to_string(),
        phase: ReadinessPhase::Prepare,
        ready: false,
        authority: AuthorityEvidence {
            integration_mode: ProposalIntegrationMode::PullRequest,
            target_ref: None,
            oid: None,
        },
        proposal_integration_oid: None,
        conditions: vec![ReadinessCondition {
            code: "authority_ref".to_string(),
            passed: false,
            message: "Tracked upstream is unavailable.".to_string(),
            remediation: Some("Configure the target upstream and retry.".to_string()),
            path: None,
            validator_code: None,
        }],
    };

    let rendered = render_readiness_text(&report);
    assert!(rendered.contains("not ready"));
    assert!(rendered.contains("Authority ref: unresolved"));
    assert!(rendered.contains("Authority OID: unresolved"));
    assert!(rendered.contains("[FAIL] authority_ref"));
    assert!(rendered.contains("Fix: Configure the target upstream and retry."));
}
