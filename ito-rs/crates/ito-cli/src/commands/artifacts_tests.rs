use super::*;

#[test]
fn artifact_kind_from_selector_maps_expected_variants() {
    assert!(matches!(
        artifact_kind_from_selector(&ChangeArtifactSelector::Proposal),
        ChangeArtifactKind::Proposal
    ));
    assert!(matches!(
        artifact_kind_from_selector(&ChangeArtifactSelector::Design),
        ChangeArtifactKind::Design
    ));
    assert!(matches!(
        artifact_kind_from_selector(&ChangeArtifactSelector::Tasks),
        ChangeArtifactKind::Tasks
    ));
    assert_eq!(
        artifact_kind_from_selector(&ChangeArtifactSelector::Spec {
            capability: "backend-agent-instructions".to_string(),
        }),
        ChangeArtifactKind::SpecDelta {
            capability: "backend-agent-instructions".to_string(),
        }
    );
}
