use super::*;

const BASE: &str = "# Capability Specification\n\n## Purpose\n\nStable behavior.\n\n## Requirements\n\n### Requirement: Keep\nOld keep text.\n\n#### Scenario: Kept\n- **WHEN** used\n- **THEN** works\n\n### Requirement: Change\nOld text.\n\n### Requirement: Remove\nRemove me.\n";

#[test]
fn reconciliation_adds_modifies_and_removes_without_losing_unrelated_requirements() {
    let delta = "## ADDED Requirements\n\n### Requirement: Add\nAdded text.\n\n## MODIFIED Requirements\n\n### Requirement: Change\nNew text.\n\n## REMOVED Requirements\n\n### Requirement: Remove\nRemove me.\n";
    let merged = reconcile_spec(Some(BASE), delta)
        .expect("merge")
        .expect("remaining spec");

    assert!(merged.contains("## Purpose\n\nStable behavior."));
    assert!(merged.contains("### Requirement: Keep"));
    assert!(merged.contains("### Requirement: Change\nNew text."));
    assert!(merged.contains("### Requirement: Add\nAdded text."));
    assert!(!merged.contains("Old text."));
    assert!(!merged.contains("### Requirement: Remove"));
    assert_eq!(merged.matches("## Requirements").count(), 1);
    assert!(!merged.contains("## ADDED Requirements"));
}

#[test]
fn reconciliation_renames_requirement_by_exact_heading() {
    let delta = "## RENAMED Requirements\n\nFROM: Change\nTO: Changed Name\n";
    let merged = reconcile_spec(Some(BASE), delta)
        .expect("merge")
        .expect("remaining spec");
    assert!(merged.contains("### Requirement: Changed Name\nOld text."));
    assert!(!merged.contains("### Requirement: Change\n"));
}

#[test]
fn reconciliation_renames_before_modifying_the_new_name() {
    let delta = "## MODIFIED Requirements\n\n### Requirement: Changed Name\nNew text.\n\n## RENAMED Requirements\n\nFROM: Change\nTO: Changed Name\n";
    let merged = reconcile_spec(Some(BASE), delta)
        .expect("merge")
        .expect("remaining spec");
    assert!(merged.contains("### Requirement: Changed Name\nNew text."));
    assert!(!merged.contains("### Requirement: Change\n"));
    assert!(!merged.contains("Old text."));
}

#[test]
fn reconciliation_returns_none_when_last_requirement_is_removed() {
    let base = "## Requirements\n\n### Requirement: Remove\nOld.\n";
    let delta = "## REMOVED Requirements\n\n### Requirement: Remove\nOld.\n";
    assert_eq!(reconcile_spec(Some(base), delta).expect("merge"), None);
}

#[test]
fn reconciliation_rejects_missing_modified_requirement() {
    let delta = "## MODIFIED Requirements\n\n### Requirement: Missing\nNew.\n";
    let error = reconcile_spec(Some(BASE), delta).expect_err("missing requirement");
    assert!(
        error
            .to_string()
            .contains("cannot modify missing requirement")
    );
}

#[test]
fn new_capability_is_normalized_to_current_requirements() {
    let delta = "<!-- ITO:START -->\n## ADDED Requirements\n\n### Requirement: New\nNew behavior.\n<!-- ITO:END -->\n";
    let merged = reconcile_spec(None, delta)
        .expect("merge")
        .expect("new spec");
    assert!(merged.contains("<!-- ITO:START -->\n\n## Requirements"));
    assert!(merged.contains("### Requirement: New"));
    assert!(merged.ends_with("<!-- ITO:END -->\n"));
}

#[test]
fn reconciliation_preserves_legacy_prefix_without_existing_requirements() {
    let base = "# Legacy Capability\n\n## Purpose\n\nStable context.\n";
    let delta = "## ADDED Requirements\n\n### Requirement: New\nNew behavior.\n";
    let merged = reconcile_spec(Some(base), delta)
        .expect("merge")
        .expect("remaining spec");
    assert!(merged.starts_with("# Legacy Capability\n\n## Purpose\n\nStable context."));
    assert!(merged.contains("## Requirements\n\n### Requirement: New"));
}

#[test]
fn reconciliation_rejects_raw_delta_as_current_spec() {
    let base = "## ADDED Requirements\n\n### Requirement: Existing\nOld.\n";
    let delta = "## MODIFIED Requirements\n\n### Requirement: Existing\nNew.\n";
    let error = reconcile_spec(Some(base), delta).expect_err("raw delta base");
    assert!(error.to_string().contains("current spec contains delta"));
}

#[test]
fn managed_delta_wraps_unmanaged_current_spec_with_balanced_markers() {
    let delta = "<!-- ITO:START -->\n## ADDED Requirements\n\n### Requirement: Add\nAdded text.\n<!-- ITO:END -->\n";
    let merged = reconcile_spec(Some(BASE), delta)
        .expect("merge")
        .expect("remaining spec");
    assert_eq!(merged.matches(ito_templates::ITO_START_MARKER).count(), 1);
    assert_eq!(merged.matches(ito_templates::ITO_END_MARKER).count(), 1);
    assert!(merged.starts_with("<!-- ITO:START -->\n\n# Capability Specification"));
    assert!(merged.ends_with("<!-- ITO:END -->\n"));
}

#[test]
fn reconciliation_normalizes_crlf_without_corrupting_the_prefix() {
    let base = BASE.replace('\n', "\r\n");
    let delta = "## MODIFIED Requirements\r\n\r\n### Requirement: Change\r\nWindows text.\r\n";
    let merged = reconcile_spec(Some(&base), delta)
        .expect("merge")
        .expect("remaining spec");
    assert!(merged.starts_with("# Capability Specification\n\n## Purpose\n\nStable behavior."));
    assert!(merged.contains("### Requirement: Change\nWindows text."));
    assert!(!merged.contains('\r'));
}

#[test]
fn reconciliation_rejects_malformed_requirement_heading_as_no_op() {
    let delta = "## ADDED Requirements\n\n### RequirementFoo: Not valid\nText.\n";
    let error = reconcile_spec(Some(BASE), delta).expect_err("malformed heading");
    assert!(error.to_string().contains("malformed requirement heading"));
}

#[test]
fn reconciliation_rejects_malformed_current_requirement_heading() {
    let base = "## Requirements\n\n### RequirementFoo: Existing\nOld.\n";
    let delta = "## ADDED Requirements\n\n### Requirement: New\nNew.\n";
    let error = reconcile_spec(Some(base), delta).expect_err("malformed current heading");
    assert!(
        error
            .to_string()
            .contains("current spec contains malformed")
    );
}

#[test]
fn reconciliation_rejects_duplicate_from_and_empty_to_tokens() {
    let duplicate_from = "## RENAMED Requirements\n\nFROM: Change\nFROM: Keep\nTO: Changed\n";
    let error = reconcile_spec(Some(BASE), duplicate_from).expect_err("duplicate FROM");
    assert!(error.to_string().contains("consecutive FROM"));

    let empty_to = "## RENAMED Requirements\n\nFROM: Change\nTO:\n";
    let error = reconcile_spec(Some(BASE), empty_to).expect_err("empty TO");
    assert!(error.to_string().contains("empty TO"));
}

#[test]
fn reconciliation_rejects_empty_delta_instead_of_deleting_current_spec() {
    let error = reconcile_spec(Some(BASE), "## ADDED Requirements\n").expect_err("empty delta");
    assert!(
        error
            .to_string()
            .contains("contains no valid requirement payload")
    );
}

#[test]
fn reconciliation_rejects_malformed_rename_section_mixed_with_valid_addition() {
    let delta = "## ADDED Requirements\n\n### Requirement: Add\nAdded.\n\n## RENAMED Requirements\n\nFROM Change\nTO Changed\n";
    let error = reconcile_spec(Some(BASE), delta).expect_err("malformed rename section");
    assert!(error.to_string().contains("malformed entry"));
}

#[test]
fn reconciliation_rejects_unknown_operation_heading_without_inheriting_state() {
    let delta = "## ADDED Requirements\n\n### Requirement: Add\nAdded.\n\n## MODIFED Requirements\n\n### Requirement: Change\nChanged.\n";
    let error = reconcile_spec(Some(BASE), delta).expect_err("unknown operation heading");
    assert!(
        error
            .to_string()
            .contains("unrecognized requirement operation heading")
    );
}

#[test]
fn reconciliation_requires_payload_in_every_declared_operation_section() {
    let delta = "## ADDED Requirements\n\n### Requirement: Add\nAdded.\n\n## MODIFIED Requirements\n\n## REMOVED Requirements\n\n### Requirement: Remove\nRemove.\n";
    let error = reconcile_spec(Some(BASE), delta).expect_err("empty modified section");
    assert!(
        error
            .to_string()
            .contains("MODIFIED Requirements section contains no valid")
    );
}
