use ito_config::ConfigContext;
use ito_core::workflow::{
    SchemaSource, export_embedded_schemas, resolve_instructions, resolve_schema,
};

#[test]
fn resolve_schema_uses_embedded_when_no_overrides_exist() {
    let project = tempfile::tempdir().expect("tempdir should succeed");
    let ctx = ConfigContext {
        project_dir: Some(project.path().to_path_buf()),
        ..Default::default()
    };

    let resolved = resolve_schema(Some("spec-driven"), &ctx).expect("schema should resolve");
    assert_eq!(resolved.source, SchemaSource::Embedded);
    assert_eq!(resolved.schema.name, "spec-driven");
}

#[test]
fn resolve_schema_prefers_project_over_user_override() {
    let root = tempfile::tempdir().expect("tempdir should succeed");
    let project = root.path().join("project");
    let home = root.path().join("home");

    std::fs::create_dir_all(project.join(".ito/templates/schemas/spec-driven"))
        .expect("project schema dir");
    std::fs::create_dir_all(home.join(".local/share/ito/schemas/spec-driven"))
        .expect("user schema dir");

    std::fs::write(
        project.join(".ito/templates/schemas/spec-driven/schema.yaml"),
        "name: spec-driven\nversion: 1\ndescription: project\nartifacts: []\n",
    )
    .expect("write project schema");
    std::fs::write(
        home.join(".local/share/ito/schemas/spec-driven/schema.yaml"),
        "name: spec-driven\nversion: 1\ndescription: user\nartifacts: []\n",
    )
    .expect("write user schema");

    let ctx = ConfigContext {
        project_dir: Some(project),
        home_dir: Some(home),
        ..Default::default()
    };

    let resolved = resolve_schema(Some("spec-driven"), &ctx).expect("schema should resolve");
    assert_eq!(resolved.source, SchemaSource::Project);
    assert_eq!(resolved.schema.description.as_deref(), Some("project"));
}

#[test]
fn resolve_instructions_reads_embedded_templates() {
    let root = tempfile::tempdir().expect("tempdir should succeed");
    let ito_path = root.path().join(".ito");
    std::fs::create_dir_all(ito_path.join("changes/demo-change")).expect("create change dir");

    let ctx = ConfigContext {
        project_dir: Some(root.path().to_path_buf()),
        ..Default::default()
    };

    let out = resolve_instructions(
        &ito_path,
        "demo-change",
        Some("spec-driven"),
        "proposal",
        &ctx,
    )
    .expect("instructions should resolve");

    assert!(out.template.contains("## Why"));
}

#[test]
fn export_embedded_schemas_writes_then_skips_without_force() {
    let root = tempfile::tempdir().expect("tempdir should succeed");
    let out_dir = root.path().join("schemas-out");

    let first = export_embedded_schemas(&out_dir, false).expect("first export should succeed");
    assert!(first.written > 0);
    assert_eq!(first.skipped, 0);

    let second = export_embedded_schemas(&out_dir, false).expect("second export should succeed");
    assert!(second.skipped > 0);

    let forced = export_embedded_schemas(&out_dir, true).expect("forced export should succeed");
    assert!(forced.written > 0);
    assert_eq!(forced.skipped, 0);
}
