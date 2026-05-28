use super::{SchemaSource, ValidationTrackingSourceYaml, ValidatorId};

#[test]
fn schema_source_as_str_returns_expected_labels() {
    assert_eq!(SchemaSource::Project.as_str(), "project");
    assert_eq!(SchemaSource::User.as_str(), "user");
    assert_eq!(SchemaSource::Embedded.as_str(), "embedded");
    assert_eq!(SchemaSource::Package.as_str(), "package");
}

#[test]
fn validation_yaml_parses_minimal_config() {
    let src = r#"
version: 1
manual_semantic_validation_note: Semantic validation is manual.
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
"#;

    let parsed: super::ValidationYaml = serde_yaml::from_str(src).expect("parse validation");
    assert_eq!(parsed.version, 1);
    assert_eq!(
        parsed.manual_semantic_validation_note.as_deref(),
        Some("Semantic validation is manual.")
    );
    assert_eq!(parsed.artifacts.len(), 1);
    assert!(parsed.artifacts.get("specs").expect("specs").required);
    assert_eq!(
        parsed.artifacts.get("specs").and_then(|a| a.validate_as),
        Some(ValidatorId::DeltaSpecsV1)
    );

    let tracking = parsed.tracking.expect("tracking");
    assert_eq!(tracking.source, ValidationTrackingSourceYaml::ApplyTracks);
    assert!(tracking.required);
    assert_eq!(tracking.validate_as, ValidatorId::TasksTrackingV1);
    assert!(tracking.rules.is_none());
    assert!(parsed.proposal.is_none());
}

#[test]
fn validation_yaml_parses_rules_extension_without_breaking_existing_shape() {
    let src = r#"
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: error
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
  rules:
    task_quality: warning
"#;

    let parsed: super::ValidationYaml = serde_yaml::from_str(src).expect("parse validation");
    let artifact_rules = parsed
        .artifacts
        .get("specs")
        .and_then(|artifact| artifact.rules.as_ref())
        .expect("artifact rules");
    assert_eq!(
        artifact_rules.get("scenario_grammar"),
        Some(&super::ValidationLevelYaml::Error)
    );

    let tracking = parsed.tracking.expect("tracking");
    let tracking_rules = tracking.rules.expect("tracking rules");
    assert_eq!(
        tracking_rules.get("task_quality"),
        Some(&super::ValidationLevelYaml::Warning)
    );
}

#[test]
fn validation_yaml_parses_proposal_entry_with_rules() {
    let src = r#"
version: 1
proposal:
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
"#;

    let parsed: super::ValidationYaml = serde_yaml::from_str(src).expect("parse validation");
    let proposal = parsed.proposal.expect("proposal");
    assert_eq!(proposal.validate_as, Some(ValidatorId::DeltaSpecsV1));
    let rules = proposal.rules.expect("proposal rules");
    assert_eq!(
        rules.get("capabilities_consistency"),
        Some(&super::ValidationLevelYaml::Error)
    );
}
