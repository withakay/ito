use ito_domain::workflow;
use std::path::Path;

#[test]
fn parse_workflow_extracts_waves_and_tasks() {
    let contents = r#"---
version: "1"
id: test
name: Test Workflow
waves:
  - id: wave-1
    name: Wave 1
    tasks:
      - id: task-1
        name: Do something
        agent: execution
        prompt: Do something
      - id: task-2
        name: Do another thing
        agent: execution
        prompt: Do another thing
  - id: wave-2
    name: Wave 2
    tasks:
      - id: task-3
        name: Final task
        agent: review
        prompt: Final task
"#;
    let wf = workflow::parse_workflow(contents).expect("parse");
    assert_eq!(wf.id, "test");
    assert_eq!(wf.waves.len(), 2);
    assert_eq!(workflow::count_tasks(&wf), 3);
}

#[test]
fn workflow_path_helpers_build_expected_locations() {
    let ito_path = Path::new("/tmp/project/.ito");

    assert_eq!(
        workflow::workflows_dir(ito_path),
        Path::new("/tmp/project/.ito/workflows")
    );
    assert_eq!(
        workflow::workflow_state_dir(ito_path),
        Path::new("/tmp/project/.ito/workflows/.state")
    );
    assert_eq!(
        workflow::commands_dir(ito_path),
        Path::new("/tmp/project/.ito/commands")
    );
    assert_eq!(
        workflow::workflow_file_path(ito_path, "research"),
        Path::new("/tmp/project/.ito/workflows/research.yaml")
    );
}

#[test]
fn parse_workflow_returns_error_for_invalid_yaml() {
    let err = workflow::parse_workflow("not: [valid").expect_err("should fail parsing");
    assert!(!err.trim().is_empty());
}
