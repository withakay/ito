use ito_core::change_repository::FsChangeRepository;
use ito_core::validate::validate_change;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
- **Supporting contexts**: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Order lifecycle. |  | Order |  |
| Billing |  |  | Invoice |  |

## Model Ownership

- **Translation required**: <!-- Where external concepts become local concepts, or None. -->
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders need to coordinate with Billing.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.level == "WARNING"
                && issue.message.contains("context ownership")
                && issue.message.contains("relationship pattern")
                && issue.message.contains("translation boundary")
        }),
        "expected context boundary warning, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
- **Supporting contexts**: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Owns order lifecycle and checkout decisions. | Orders owner | Order, checkout | Customer/supplier with Billing |
| Billing | Owns invoice creation and payment status. | Billing owner | Invoice, payment | Supplier to Orders |

## Model Ownership

- **Translation boundaries**: Billing payment status is translated into an Orders-local checkout settlement state.
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders need to coordinate with Billing.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("context_boundary_consistency") }),
        "framed cross-context discovery should not warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_rules_parse_embedded_compact_discovery_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n    context_boundary_consistency: warning\n    domain_documentation_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Users need a project space for checkout collaboration.

## Domain Discovery Summary

- Primary problem: checkout needs consistent collaboration language.
- Discovery depth: bounded-context because checkout crosses contexts.
- Business/domain capability: checkout collaboration
- Primary bounded context: Orders
- Supporting contexts: Billing
- Model ownership: Orders owns checkout rules; Billing owns invoice rules.
- Canonical terms: Workspace -> A tenant-scoped collaboration boundary.
- Rejected aliases / overloaded terms: project space -> Workspace
- Bounded contexts: Orders -> checkout lifecycle, Orders owner, Order and checkout, ; Billing -> invoice settlement, Billing owner, Invoice and payment,
- Cross-context relationships: customer/supplier with published language and translation boundary.
- Translation boundaries: Billing payment status becomes an Orders-local checkout settlement state.
"#,
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("docs")
            .join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A billing account container. | Orders | proposed update |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
                && issue.message.contains("project space")
        }),
        "compact rejected alias should warn, got issues: {:?}",
        report.issues
    );
    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("domain_documentation_consistency")
                && issue.message.contains("Workspace")
        }),
        "compact canonical term should drive doc consistency, got issues: {:?}",
        report.issues
    );
    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("context_boundary_consistency") }),
        "compact context handoff is fully framed and should not warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_rules_parse_embedded_full_discovery_sections() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n    context_boundary_consistency: warning\n    domain_documentation_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Users need a project space for checkout collaboration.

## Domain Discovery Summary

- Primary problem: checkout needs consistent collaboration language.
- Discovery depth: bounded-context because checkout crosses contexts.
- Business/domain capability: checkout collaboration
- Primary bounded context: Orders
- Supporting contexts: Billing

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A tenant-scoped collaboration boundary. | Orders | canonical |

## Rejected Aliases / Overloaded Terms

| Alias / Term | Prefer | Reason |
| --- | --- | --- |
| project space | Workspace | Avoids conflict with Ito project terminology. |

## Bounded Context Map

| Context | Responsibilities | Owner | Owned language | Relationship pattern |
| --- | --- | --- | --- | --- |
| Orders | checkout lifecycle | Orders owner | Order and checkout | customer/supplier |
| Billing | invoice settlement | Billing owner | Invoice and payment | supplier |

## Model Ownership

- Translation boundaries: Billing payment status becomes an Orders-local checkout settlement state.
"#,
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("docs")
            .join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A billing account container. | Orders | proposed update |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
                && issue.message.contains("project space")
        }),
        "embedded full rejected alias table should warn, got issues: {:?}",
        report.issues
    );
    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("domain_documentation_consistency")
                && issue.message.contains("Workspace")
        }),
        "embedded full canonical term table should drive doc consistency, got issues: {:?}",
        report.issues
    );
    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("context_boundary_consistency") }),
        "embedded full context handoff is fully framed and should not warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_uses_header_columns_for_owner_tables() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nCheckout collaboration crosses Orders and Billing.\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- Primary problem: checkout crosses Orders and Billing.
- Discovery depth: bounded-context because checkout crosses contexts.
- Primary bounded context: Orders
- Supporting contexts: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned language | Relationship pattern |
| --- | --- | --- | --- | --- |
| Orders | checkout lifecycle | Orders owner | Order and checkout | |
| Billing | invoice settlement | Billing owner | Invoice and payment | |

## Model Ownership

- Translation boundaries: Billing payment status becomes an Orders-local checkout settlement state.
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("relationship pattern")
        }),
        "missing relationship column values should warn even when owner columns are populated, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_requires_ownership_for_each_affected_context() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nCheckout collaboration crosses Orders, Billing, and Fulfillment.\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- Primary bounded context: Orders
- Supporting contexts: Billing, Fulfillment

## Bounded Context Map

| Context | Responsibilities | Owner | Owned language | Relationship pattern |
| --- | --- | --- | --- | --- |
| Orders | checkout lifecycle | Orders owner | Order and checkout | customer/supplier |
| Billing | invoice settlement | Billing owner | Invoice and payment | supplier |
| Fulfillment | shipment release |  | Shipment | customer/supplier |

## Model Ownership

- Translation boundaries: Billing and Fulfillment concepts become Orders-local settlement and release states.
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("context ownership")
        }),
        "each affected context should require explicit ownership, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_is_silent_for_single_context_discovery() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
- **Supporting contexts**: None

## Bounded Context Map

| Context | Responsibilities | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- |
| Orders | Owns order lifecycle. | Order | None |
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders need clearer lifecycle rules.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("context_boundary_consistency") }),
        "single-context discovery should not warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_rules_can_run_from_artifact_rules_for_event_driven_schemas() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("schema.yaml"),
        r#"
name: event-like
version: 1
artifacts:
  - id: domain-discovery
    generates: domain-discovery.md
    template: domain-discovery.md
    optional: true
    requires: []
  - id: specs
    generates: specs/**/*.md
    template: specs/spec.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  domain-discovery:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      context_boundary_consistency: warning
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: event-like\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
- **Supporting contexts**: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Order lifecycle. |  | Order |  |
| Billing |  |  | Invoice |  |
"#,
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("specs")
            .join("orders")
            .join("spec.md"),
        r#"
## ADDED Requirements

### Requirement: Coordinate billing
The system SHALL coordinate checkout with billing.

#### Scenario: Billing event arrives
- **GIVEN** an order is pending settlement
- **WHEN** a billing event arrives
- **THEN** checkout state is updated
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.path == "domain-discovery.bounded-context-map"
        }),
        "artifact-level domain rule should run without proposal.md, got issues: {:?}",
        report.issues
    );
}

#[test]
fn spec_only_artifact_rules_are_rejected_for_domain_discovery_artifacts() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("schema.yaml"),
        r#"
name: event-like
version: 1
artifacts:
  - id: domain-discovery
    generates: domain-discovery.md
    template: domain-discovery.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  domain-discovery:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: warning
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: event-like\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        "# Domain Discovery\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.path == "schema.validation.artifacts.domain-discovery.rules.scenario_grammar"
                && issue
                    .message
                    .contains("Unknown validation rule 'scenario_grammar'")
        }),
        "spec-only artifact rule should be rejected outside specs, got issues: {:?}",
        report.issues
    );
}

#[test]
fn non_domain_discovery_artifacts_still_accept_spec_artifact_rules() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("schema.yaml"),
        r#"
name: event-like
version: 1
artifacts:
  - id: event-modeling
    generates: event-modeling.md
    template: event-modeling.md
    requires: []
"#,
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("event-like")
            .join("validation.yaml"),
        r#"
version: 1
artifacts:
  event-modeling:
    required: true
    validate_as: ito.delta-specs.v1
    rules:
      scenario_grammar: warning
"#,
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: event-like\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("event-modeling.md"),
        "# Event Modeling\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report.issues.iter().any(|issue| {
            issue.path == "schema.validation.artifacts.event-modeling.rules.scenario_grammar"
                && issue.message.contains("Unknown validation rule")
        }),
        "non-domain-discovery artifacts should keep spec artifact rules, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_documentation_consistency_rule_checks_existing_project_context_docs() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    domain_documentation_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A tenant-scoped collaboration boundary. | Collaboration | canonical term |
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nUsers need clearer collaboration language.\n",
    );
    write(
        &project_root.join("docs").join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A billing account container. | Collaboration | existing docs |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("domain_documentation_consistency")
                && issue.path == "docs/CONTEXT.md"
                && issue.message.contains("Workspace")
        }),
        "existing context docs should be checked, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_documentation_consistency_rule_allows_same_term_in_different_contexts() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    domain_documentation_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Account | A tenant billing contract. | Billing | canonical term |
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nBilling needs account language.\n",
    );
    write(
        &project_root.join("docs").join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Account | A login identity. | Identity | existing docs |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("domain_documentation_consistency")
                && issue.path == "docs/CONTEXT.md"
        }),
        "same term in a different context should not be treated as drift, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_warns_for_direct_context_coordination_without_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    context_boundary_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders coordinates with Billing during checkout settlement.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.path == "domain-discovery.bounded-context-map"
        }),
        "direct context coordination should require boundary framing, got issues: {:?}",
        report.issues
    );
}

#[test]
fn domain_rules_are_silent_without_domain_discovery_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n    context_boundary_consistency: warning\n    domain_documentation_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nUsers need a project space for collaboration.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
                || issue.rule_id.as_deref() == Some("context_boundary_consistency")
                || issue.rule_id.as_deref() == Some("domain_documentation_consistency")
        }),
        "domain rules should be no-op without handoff, got issues: {:?}",
        report.issues
    );
}

#[test]
fn ubiquitous_language_rule_ignores_rejected_alias_declaration_itself() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Users need clearer checkout collaboration language.

## Domain Discovery Summary

- Primary problem: checkout needs consistent collaboration language.
- Canonical terms: Workspace -> A tenant-scoped collaboration boundary.
- Rejected aliases / overloaded terms: project space -> Workspace
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("ubiquitous_language_consistency") }),
        "rejected alias declaration should not self-trigger, got issues: {:?}",
        report.issues
    );
}

#[test]
fn placeholder_standalone_discovery_does_not_hide_embedded_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";

    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("schema.yaml"),
        "name: proposal-rules\nversion: 1\nartifacts: []\n",
    );
    write(
        &project_root
            .join(".ito")
            .join("templates")
            .join("schemas")
            .join("proposal-rules")
            .join("validation.yaml"),
        "version: 1\nproposal:\n  required: true\n  validate_as: ito.delta-specs.v1\n  rules:\n    ubiquitous_language_consistency: warning\n",
    );
    write(
        &ito.join("changes").join(change_id).join(".ito.yaml"),
        "schema: proposal-rules\n",
    );
    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary problem**: <!-- One sentence describing the domain problem. -->
- **Discovery depth**: <!-- direct, lightweight, bounded-context, or rigorous domain-grill; include trigger rationale. -->
- **Supporting contexts**: None
- **Translation boundaries**: None
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Users need a project space for checkout collaboration.

## Domain Discovery Summary

- Primary problem: checkout needs consistent collaboration language.
- Canonical terms: Workspace -> A tenant-scoped collaboration boundary.
- Rejected aliases / overloaded terms: project space -> Workspace
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("ubiquitous_language_consistency")
                && issue.message.contains("project space")
        }),
        "meaningless standalone artifact should fall through to embedded handoff, got issues: {:?}",
        report.issues
    );
}
