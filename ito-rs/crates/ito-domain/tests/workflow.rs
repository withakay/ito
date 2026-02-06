use ito_domain::workflow;

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
