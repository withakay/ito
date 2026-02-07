use ito_domain::schemas::{ExecutionPlan, Tool, WorkflowDefinition, WorkflowExecution};

#[test]
fn workflow_yaml_roundtrip() {
    let yaml = r#"version: "1.0"
id: research
name: Domain Research
description: Investigate domain knowledge

requires:
  variables:
    - topic

context_files:
  - planning/PROJECT.md
  - planning/STATE.md

waves:
  - id: investigate
    name: Parallel Investigation
    tasks:
      - id: stack-analysis
        name: Stack Analysis
        agent: research
        prompt: commands/research-stack.md
        output: research/investigations/stack-analysis.md
        context:
          topic: "{{topic}}"
"#;

    let a: WorkflowDefinition = serde_yaml::from_str(yaml).expect("parse yaml");
    let out = serde_yaml::to_string(&a).expect("serialize yaml");
    let b: WorkflowDefinition = serde_yaml::from_str(&out).expect("re-parse yaml");
    assert_eq!(a, b);
}

#[test]
fn workflow_execution_json_roundtrip() {
    let json = r#"{
  "workflow": {
    "version": "1.0",
    "id": "demo",
    "name": "Demo",
    "description": "",
    "waves": [
      {
        "id": "w1",
        "tasks": [
          {
            "id": "t1",
            "name": "Task 1",
            "agent": "execution",
            "prompt": "commands/do.md"
          }
        ]
      }
    ]
  },
  "status": "running",
  "started_at": "2026-01-01T00:00:00.000Z",
  "current_wave_index": 0,
  "waves": [
    {
      "wave": {
        "id": "w1",
        "tasks": [
          {
            "id": "t1",
            "name": "Task 1",
            "agent": "execution",
            "prompt": "commands/do.md"
          }
        ]
      },
      "status": "running",
      "tasks": [
        {
          "task": {
            "id": "t1",
            "name": "Task 1",
            "agent": "execution",
            "prompt": "commands/do.md"
          },
          "status": "running"
        }
      ]
    }
  ],
  "variables": {}
}"#;

    let a: WorkflowExecution = serde_json::from_str(json).expect("parse json");
    let out = serde_json::to_string_pretty(&a).expect("serialize json");
    let b: WorkflowExecution = serde_json::from_str(&out).expect("re-parse json");
    assert_eq!(a, b);
}

#[test]
fn workflow_plan_json_roundtrip() {
    let json = r#"{
  "tool": "opencode",
  "workflow": {
    "version": "1.0",
    "id": "demo",
    "name": "Demo",
    "description": "",
    "waves": []
  },
  "waves": []
}"#;

    let a: ExecutionPlan = serde_json::from_str(json).expect("parse json");
    assert_eq!(a.tool, Tool::OpenCode);
    let out = serde_json::to_string_pretty(&a).expect("serialize json");
    let b: ExecutionPlan = serde_json::from_str(&out).expect("re-parse json");
    assert_eq!(a, b);
}
