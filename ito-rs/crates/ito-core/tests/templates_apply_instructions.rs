use ito_config::ConfigContext;
use ito_core::templates::compute_apply_instructions;
use std::path::Path;

fn write(path: &Path, contents: &str) {
    let Some(parent) = path.parent() else {
        panic!("path has no parent: {}", path.display());
    };
    std::fs::create_dir_all(parent).expect("create dir should succeed");
    std::fs::write(path, contents).expect("write should succeed");
}

#[test]
fn compute_apply_instructions_reports_blocked_states_and_progress() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let project_root = td.path();
    let ito_path = project_root.join(".ito");
    let change = "demo-change";
    let change_dir = ito_path.join("changes").join(change);
    std::fs::create_dir_all(&change_dir).expect("create change dir");

    std::fs::create_dir_all(project_root.join(".ito/templates/schemas/demo/templates"))
        .expect("create schema dirs");
    write(
        &project_root.join(".ito/templates/schemas/demo/schema.yaml"),
        r#"name: demo
version: 1
description: demo
apply:
  requires: ["proposal"]
  tracks: tasks.md
  instruction: |
    Follow the tasks file.
artifacts:
  - id: proposal
    generates: proposal.md
    template: proposal.md
    requires: []
  - id: specs
    generates: specs/**/*.md
    template: spec.md
    requires: []
"#,
    );

    let ctx = ConfigContext {
        project_dir: Some(project_root.to_path_buf()),
        ..Default::default()
    };

    // 1) Missing required artifacts -> blocked.
    let r = compute_apply_instructions(&ito_path, change, Some("demo"), &ctx)
        .expect("compute_apply_instructions");
    assert_eq!(r.state, "blocked");
    assert!(
        r.missing_artifacts
            .as_ref()
            .is_some_and(|m| m.contains(&"proposal".to_string()))
    );
    assert!(r.instruction.contains("Missing artifacts"));

    // 2) Required artifact exists but tracking file is missing -> blocked.
    write(&change_dir.join("proposal.md"), "ok");
    let r = compute_apply_instructions(&ito_path, change, Some("demo"), &ctx)
        .expect("compute_apply_instructions");
    assert_eq!(r.state, "blocked");
    assert!(r.instruction.contains("tasks.md"));
    assert!(r.instruction.contains("missing"));
    assert!(r.context_files.contains_key("proposal"));

    // 3) Tracking file exists but contains no tasks -> blocked.
    write(&change_dir.join("tasks.md"), "# empty\n");
    let r = compute_apply_instructions(&ito_path, change, Some("demo"), &ctx)
        .expect("compute_apply_instructions");
    assert_eq!(r.state, "blocked");
    assert!(r.instruction.contains("contains no tasks"));
    assert_eq!(r.progress.total, 0);

    // 4) Checkbox tasks parse + progress.
    write(
        &change_dir.join("tasks.md"),
        "## 1. Implementation\n- [ ] Do A\n- [~] Do B\n- [x] Do C\n",
    );
    let r = compute_apply_instructions(&ito_path, change, Some("demo"), &ctx)
        .expect("compute_apply_instructions");
    assert_eq!(r.state, "ready");
    assert_eq!(r.tracks_format.as_deref(), Some("checkbox"));
    assert_eq!(r.progress.total, 3);
    assert_eq!(r.progress.complete, 1);
    assert_eq!(r.progress.remaining, 2);
    assert_eq!(r.progress.in_progress, Some(1));
    assert_eq!(r.progress.pending, Some(1));
    assert_eq!(r.tasks.len(), 3);

    // 5) All tasks complete -> all_done.
    write(
        &change_dir.join("tasks.md"),
        "## 1. Implementation\n- [x] A\n- [x] B\n",
    );
    let r = compute_apply_instructions(&ito_path, change, Some("demo"), &ctx)
        .expect("compute_apply_instructions");
    assert_eq!(r.state, "all_done");
    assert!(r.instruction.contains("ready to be archived"));
}
