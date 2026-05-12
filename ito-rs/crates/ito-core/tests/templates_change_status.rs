use ito_config::ConfigContext;
use ito_core::templates::{ArtifactStatus, WorkflowError, compute_change_status};

fn find_artifact<'a>(artifacts: &'a [ArtifactStatus], id: &str) -> &'a ArtifactStatus {
    for artifact in artifacts {
        if artifact.id == id {
            return artifact;
        }
    }
    panic!("artifact not found: {id}");
}

#[test]
fn compute_change_status_marks_ready_and_blocked_based_on_generated_files() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");

    std::fs::create_dir_all(ito_path.join("changes").join("demo-change"))
        .expect("create change dir");

    std::fs::create_dir_all(project_root.join(".ito/templates/schemas/demo/templates"))
        .expect("create schema dirs");

    std::fs::write(
        project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        r#"name: demo
version: 1
apply:
  requires: ["b"]
artifacts:
  - id: a
    generates: a.md
    template: a.md
    requires: []
  - id: b
    generates: b.md
    template: b.md
    requires: ["a"]
"#,
    )
    .expect("write schema.yaml");

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");
    assert_eq!(status.schema_name, "demo");
    assert_eq!(
        status.apply_requires,
        vec!["a".to_string(), "b".to_string()]
    );
    assert!(!status.is_complete);
    assert_eq!(status.artifacts.len(), 2);

    let a = find_artifact(&status.artifacts, "a");
    let b = find_artifact(&status.artifacts, "b");
    assert_eq!(a.status, "ready");
    assert_eq!(b.status, "blocked");
    assert!(b.missing_deps.contains(&"a".to_string()));

    // Mark artifact a as done.
    std::fs::write(
        ito_path.join("changes").join("demo-change").join("a.md"),
        "done",
    )
    .expect("write a.md");

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");
    let a = find_artifact(&status.artifacts, "a");
    let b = find_artifact(&status.artifacts, "b");
    assert_eq!(a.status, "done");
    assert_eq!(b.status, "ready");

    // Mark artifact b as done.
    std::fs::write(
        ito_path.join("changes").join("demo-change").join("b.md"),
        "done",
    )
    .expect("write b.md");

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");
    assert!(status.is_complete);
}

#[test]
fn compute_change_status_rejects_invalid_change_name() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");

    std::fs::create_dir_all(project_root.join(".ito/templates/schemas/demo"))
        .expect("create schema dirs");
    std::fs::write(
        project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        "name: demo\nversion: 1\nartifacts: []\n",
    )
    .expect("write schema.yaml");

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    let err = compute_change_status(&ito_path, "../escape", Some("demo"), &ctx)
        .expect_err("invalid change name should error");
    let WorkflowError::InvalidChangeName = err else {
        panic!("expected InvalidChangeName");
    };
}

#[test]
fn compute_change_status_treats_missing_optional_artifacts_as_non_blocking() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");

    std::fs::create_dir_all(ito_path.join("changes").join("demo-change"))
        .expect("create change dir");

    std::fs::create_dir_all(project_root.join(".ito/templates/schemas/demo/templates"))
        .expect("create schema dirs");

    std::fs::write(
        project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        r#"name: demo
version: 1
artifacts:
  - id: discovery
    generates: domain-discovery.md
    template: domain-discovery.md
    optional: true
    requires: []
  - id: proposal
    generates: proposal.md
    template: proposal.md
    requires: []
"#,
    )
    .expect("write schema.yaml");

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");
    assert_eq!(status.apply_requires, vec!["proposal".to_string()]);
    assert!(!status.is_complete);
    assert_eq!(
        find_artifact(&status.artifacts, "discovery").status,
        "optional"
    );
    assert_eq!(find_artifact(&status.artifacts, "proposal").status, "ready");

    std::fs::write(
        ito_path
            .join("changes")
            .join("demo-change")
            .join("proposal.md"),
        "done",
    )
    .expect("write proposal.md");

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");
    assert!(status.is_complete);
    assert_eq!(
        find_artifact(&status.artifacts, "discovery").status,
        "optional"
    );
}

#[test]
fn compute_change_status_keeps_optional_artifact_optional_when_dependency_is_missing() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");

    std::fs::create_dir_all(ito_path.join("changes").join("demo-change"))
        .expect("create change dir");

    std::fs::create_dir_all(project_root.join(".ito/templates/schemas/demo/templates"))
        .expect("create schema dirs");

    std::fs::write(
        project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        r#"name: demo
version: 1
artifacts:
  - id: proposal
    generates: proposal.md
    template: proposal.md
    requires: []
  - id: discovery
    generates: domain-discovery.md
    template: domain-discovery.md
    optional: true
    requires: [proposal]
"#,
    )
    .expect("write schema.yaml");

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    let status = compute_change_status(&ito_path, "demo-change", Some("demo"), &ctx)
        .expect("compute_change_status");

    assert_eq!(status.apply_requires, vec!["proposal".to_string()]);
    assert_eq!(find_artifact(&status.artifacts, "proposal").status, "ready");

    let discovery = find_artifact(&status.artifacts, "discovery");
    assert_eq!(discovery.status, "optional");
    assert_eq!(discovery.missing_deps, vec!["proposal".to_string()]);
}
