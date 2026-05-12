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

fn write_domain_doc_schema(project_root: &Path, ito: &Path, change_id: &str) {
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
}

#[test]
fn compact_canonical_terms_use_primary_context_for_doc_consistency() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_domain_doc_schema(project_root, &ito, change_id);
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Orders need clearer workspace language.

## Domain Discovery Summary

- Primary bounded context: Orders
- Canonical terms: Workspace -> An Orders-owned collaboration boundary.
"#,
    );
    write(
        &project_root.join("docs").join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A billing account container. | Billing | Existing Billing meaning. |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        !report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("domain_documentation_consistency")),
        "same compact term in a different owner context should not warn, got issues: {:?}",
        report.issues
    );
}

#[test]
fn compact_terms_keep_their_own_embedded_summary_context() {
    let td = tempfile::tempdir().unwrap();
    let project_root = td.path();
    let ito = project_root.join(".ito");
    let change_id = "001-01_demo";
    write_domain_doc_schema(project_root, &ito, change_id);
    write(
        &ito.join("changes").join(change_id).join("proposal.md"),
        r#"## Why

Orders need clearer workspace language.

## Domain Discovery Summary

- Primary bounded context: Orders
- Canonical terms: Order -> An Orders-owned checkout commitment.
"#,
    );
    write(
        &ito.join("changes").join(change_id).join("design.md"),
        r#"## Domain Discovery Summary

- Primary bounded context: Billing
- Canonical terms: Workspace -> A Billing-owned subscription container.
"#,
    );
    write(
        &project_root.join("docs").join("CONTEXT.md"),
        r#"# Context

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| Workspace | A billing account container. | Billing | Existing Billing meaning. |
"#,
    );

    let change_repo = FsChangeRepository::new(&ito);
    let report = validate_change(&change_repo, &ito, change_id, false).unwrap();

    assert!(
        report
            .issues
            .iter()
            .any(|issue| issue.rule_id.as_deref() == Some("domain_documentation_consistency")),
        "later embedded compact terms should keep their own primary context and report same-context drift, got issues: {:?}",
        report.issues
    );
}
