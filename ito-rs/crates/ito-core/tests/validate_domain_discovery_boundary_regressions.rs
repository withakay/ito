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

fn write_context_boundary_schema(project_root: &Path, ito: &Path, change_id: &str) {
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
}

#[test]
fn context_boundary_consistency_rule_requires_relationship_for_each_affected_context() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

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
| Billing | Owns invoice creation and payment status. | Billing owner | Invoice, payment |  |

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
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("relationship pattern")
        }),
        "partial relationship framing should warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_accepts_explicit_no_translation_boundary() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

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
| Orders | Owns order lifecycle and checkout decisions. | Orders owner | Order, checkout | Shared kernel with Billing |
| Billing | Owns invoice creation and payment status. | Billing owner | Invoice, payment | Shared kernel with Orders |

## Model Ownership

- **Translation boundaries**: None
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders need to coordinate with Billing through a shared kernel.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| { issue.rule_id.as_deref() == Some("context_boundary_consistency") }),
        "explicit no-translation answer should satisfy the boundary gate, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_warns_when_cross_context_proposal_has_no_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nThis cross-context change spans multiple bounded contexts.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("missing domain discovery")
        }),
        "cross-context proposal without handoff should warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_warns_when_proposal_outgrows_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Owns order lifecycle and checkout decisions. | Orders owner | Order, checkout | Published language |

## Model Ownership

- **Translation boundaries**: None
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nThis cross-context change spans multiple bounded contexts.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("affected contexts")
        }),
        "underreported cross-context handoff should warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn embedded_discovery_handoffs_are_merged_in_artifact_order() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Orders need to coordinate with Billing.

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
        &ito.join("changes").join(change_id).join("design.md"),
        r#"## Domain Discovery Summary

- **Primary bounded context**: Orders

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Owns order lifecycle and checkout decisions. | Orders owner | Order, checkout | Published language |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("context_boundary_consistency")),
        "complete proposal handoff should not be hidden by a partial design handoff, got issues: {:?}",
        report.issues
    );
}

#[test]
fn partial_standalone_discovery_does_not_hide_complete_embedded_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Orders need to coordinate with Billing.

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

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("context_boundary_consistency")),
        "complete embedded handoff should satisfy boundary framing even with partial standalone discovery, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_only_standalone_discovery_does_not_override_embedded_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes")
            .join(change_id)
            .join("domain-discovery.md"),
        r#"# Domain Discovery

## Domain Discovery Summary

- **Primary bounded context**: Orders
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Billing needs clearer invoice status language.

## Domain Discovery Summary

- **Primary bounded context**: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Billing | Owns invoice creation and payment status. | Billing owner | Invoice, payment | Internal Billing model |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("context_boundary_consistency")),
        "context-only standalone discovery should not make richer embedded Billing discovery look cross-context, got issues: {:?}",
        report.issues
    );
}

#[test]
fn repeated_embedded_discovery_sections_are_merged_within_one_artifact() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Orders need to coordinate with Billing.

## Domain Discovery Summary

- **Primary bounded context**: Orders
- **Supporting contexts**: Billing

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Orders | Owns order lifecycle and checkout decisions. | Orders owner | Order, checkout | Customer/supplier with Billing |

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| Billing | Owns invoice creation and payment status. | Billing owner | Invoice, payment | Supplier to Orders |

## Model Ownership

- **Translation boundaries**: Billing payment status is translated into an Orders-local checkout settlement state.
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("context_boundary_consistency")),
        "repeated sections in one artifact should merge into one handoff, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_detects_lowercase_context_prose_without_handoff() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\norders coordinate with billing for settlement.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report.issues.iter().any(|issue| {
            issue.rule_id.as_deref() == Some("context_boundary_consistency")
                && issue.message.contains("missing domain discovery")
        }),
        "lowercase cross-context prose without handoff should warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn context_boundary_consistency_rule_ignores_obvious_external_vendor_integration() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_context_boundary_schema(project_root, &ito, change_id);

    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        "## Why\n\nOrders integrates with Stripe for external payment capture.\nOrders integrates with Slack for notifications.\nBilling coordinates with Salesforce.\n",
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("context_boundary_consistency")),
        "obvious vendor integration should not require bounded-context discovery, got issues: {:?}",
        report.issues
    );
}
