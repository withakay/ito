use ito_schemas::{
    AgentType, ExecutionPlan, ExecutionStatus, TaskDefinition, TaskExecution, TaskPlan, TaskType,
    Tool, WaveDefinition, WaveExecution, WavePlan, WorkflowDefinition, WorkflowExecution,
    WorkflowRequires,
};
use std::collections::BTreeMap;

fn base_task(id: &str) -> TaskDefinition {
    TaskDefinition {
        id: id.to_string(),
        name: format!("Task {id}"),
        agent: AgentType::Execution,
        prompt: "commands/do.md".to_string(),
        inputs: None,
        output: None,
        task_type: None,
        context: None,
    }
}

fn base_wave(id: &str) -> WaveDefinition {
    WaveDefinition {
        id: id.to_string(),
        name: None,
        tasks: vec![base_task("t1")],
        checkpoint: None,
    }
}

fn base_workflow() -> WorkflowDefinition {
    WorkflowDefinition {
        version: "1.0".to_string(),
        id: "demo".to_string(),
        name: "Demo".to_string(),
        description: String::new(),
        requires: None,
        context_files: None,
        waves: vec![base_wave("w1")],
        on_complete: None,
    }
}

#[test]
fn workflow_definition_validate_accepts_minimal_valid() {
    base_workflow().validate().expect("validate");
}

#[test]
fn workflow_definition_validate_rejects_empty_fields() {
    let mut wf = base_workflow();
    wf.version = "".to_string();
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.version"));

    let mut wf = base_workflow();
    wf.id = "".to_string();
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.id"));

    let mut wf = base_workflow();
    wf.name = "".to_string();
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.name"));

    let mut wf = base_workflow();
    wf.waves = vec![];
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.waves"));
}

#[test]
fn workflow_definition_validate_rejects_requires_and_context_files_empty_entries() {
    let mut wf = base_workflow();
    wf.requires = Some(WorkflowRequires {
        files: Some(vec!["".to_string()]),
        variables: Some(vec!["".to_string()]),
    });
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.requires"));

    let mut wf = base_workflow();
    wf.context_files = Some(vec!["".to_string()]);
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("workflow.context_files"));
}

#[test]
fn workflow_definition_validate_rejects_duplicate_wave_ids() {
    let mut wf = base_workflow();
    wf.waves = vec![base_wave("w1"), base_wave("w1")];
    let err = wf.validate().expect_err("should fail");
    assert!(err.contains("duplicate id"));
}

#[test]
fn wave_definition_validate_rejects_invalid_shapes() {
    let mut wave = base_wave("w1");
    wave.id = "".to_string();
    let err = wave.validate().expect_err("should fail");
    assert!(err.contains("wave.id"));

    let mut wave = base_wave("w1");
    wave.name = Some("".to_string());
    let err = wave.validate().expect_err("should fail");
    assert!(err.contains("wave.name"));

    let mut wave = base_wave("w1");
    wave.tasks = vec![];
    let err = wave.validate().expect_err("should fail");
    assert!(err.contains("wave.tasks"));

    let mut wave = base_wave("w1");
    wave.tasks = vec![base_task("t1"), base_task("t1")];
    let err = wave.validate().expect_err("should fail");
    assert!(err.contains("duplicate id"));
}

#[test]
fn task_definition_validate_rejects_invalid_fields() {
    let mut task = base_task("t1");
    task.id = "".to_string();
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("task.id"));

    let mut task = base_task("t1");
    task.name = "".to_string();
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("task.name"));

    let mut task = base_task("t1");
    task.prompt = "".to_string();
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("task.prompt"));

    let mut task = base_task("t1");
    task.inputs = Some(vec!["".to_string()]);
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("task.inputs"));

    let mut task = base_task("t1");
    task.output = Some("".to_string());
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("task.output"));

    let mut task = base_task("t1");
    let mut ctx = BTreeMap::new();
    ctx.insert("".to_string(), "x".to_string());
    task.context = Some(ctx);
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("empty key"));

    let mut task = base_task("t1");
    let mut ctx = BTreeMap::new();
    ctx.insert("k".to_string(), "".to_string());
    task.context = Some(ctx);
    let err = task.validate().expect_err("should fail");
    assert!(err.contains("empty value"));
}

#[test]
fn task_definition_validate_accepts_optional_fields() {
    let mut task = base_task("t1");
    task.inputs = Some(vec!["one".to_string(), "two".to_string()]);
    task.output = Some("out.md".to_string());
    task.task_type = Some(TaskType::Checkpoint);
    let mut ctx = BTreeMap::new();
    ctx.insert("k".to_string(), "v".to_string());
    task.context = Some(ctx);
    task.validate().expect("validate");
}

#[test]
fn plan_validate_rejects_empty_prompt_content() {
    let json = r#"{
  "tool": "opencode",
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
  "waves": [
    {
      "wave_id": "w1",
      "tasks": [
        {
          "task_id": "t1",
          "model": "gpt",
          "context_budget": 1,
          "prompt_content": ""
        }
      ]
    }
  ]
}"#;
    let plan: ExecutionPlan = serde_json::from_str(json).expect("parse json");
    assert_eq!(plan.tool, Tool::OpenCode);
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("prompt_content"));
}

#[test]
fn plan_validate_rejects_other_invalid_fields() {
    let wf = base_workflow();

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "".to_string(),
            tasks: vec![],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.wave_id"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "".to_string(),
                model: "gpt".to_string(),
                context_budget: 1,
                prompt_content: "hi".to_string(),
                inputs: None,
                output: None,
                context: None,
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.task_id"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "t1".to_string(),
                model: "".to_string(),
                context_budget: 1,
                prompt_content: "hi".to_string(),
                inputs: None,
                output: None,
                context: None,
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.model"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "t1".to_string(),
                model: "gpt".to_string(),
                context_budget: 1,
                prompt_content: "ok".to_string(),
                inputs: Some(vec!["".to_string()]),
                output: None,
                context: None,
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.inputs"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "t1".to_string(),
                model: "gpt".to_string(),
                context_budget: 1,
                prompt_content: "ok".to_string(),
                inputs: None,
                output: Some("".to_string()),
                context: None,
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.output"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf.clone(),
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "t1".to_string(),
                model: "gpt".to_string(),
                context_budget: 1,
                prompt_content: "ok".to_string(),
                inputs: None,
                output: None,
                context: Some(BTreeMap::from([("".to_string(), "x".to_string())])),
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.context"));

    let plan = ExecutionPlan {
        tool: Tool::OpenCode,
        workflow: wf,
        waves: vec![WavePlan {
            wave_id: "w1".to_string(),
            tasks: vec![TaskPlan {
                task_id: "t1".to_string(),
                model: "gpt".to_string(),
                context_budget: 1,
                prompt_content: "ok".to_string(),
                inputs: None,
                output: None,
                context: Some(BTreeMap::from([("k".to_string(), "".to_string())])),
            }],
        }],
    };
    let err = plan.validate().expect_err("should fail");
    assert!(err.contains("plan.context"));
}

#[test]
fn execution_validate_rejects_out_of_bounds_wave_index() {
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
  "current_wave_index": 1,
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
    let exec: WorkflowExecution = serde_json::from_str(json).expect("parse json");
    let err = exec.validate().expect_err("should fail");
    assert!(err.contains("current_wave_index"));
}

#[test]
fn execution_validate_rejects_invalid_fields_and_accepts_valid() {
    let wf = base_workflow();

    let mut exec = WorkflowExecution {
        workflow: wf.clone(),
        status: ExecutionStatus::Running,
        started_at: "".to_string(),
        completed_at: None,
        current_wave_index: 0,
        waves: vec![],
        variables: BTreeMap::new(),
    };
    let err = exec.validate().expect_err("should fail");
    assert!(err.contains("execution.started_at"));

    exec.started_at = "2026-01-01T00:00:00.000Z".to_string();
    exec.completed_at = Some("".to_string());
    let err = exec.validate().expect_err("should fail");
    assert!(err.contains("execution.completed_at"));

    let mut exec = WorkflowExecution {
        workflow: wf.clone(),
        status: ExecutionStatus::Running,
        started_at: "2026-01-01T00:00:00.000Z".to_string(),
        completed_at: None,
        current_wave_index: 0,
        waves: vec![WaveExecution {
            wave: base_wave("w1"),
            status: ExecutionStatus::Running,
            tasks: vec![],
        }],
        variables: BTreeMap::from([("".to_string(), "x".to_string())]),
    };
    let err = exec.validate().expect_err("should fail");
    assert!(err.contains("execution.variables"));

    exec.variables = BTreeMap::from([("k".to_string(), "".to_string())]);
    let err = exec.validate().expect_err("should fail");
    assert!(err.contains("execution.variables"));

    let task = base_task("t1");
    let exec = WorkflowExecution {
        workflow: wf,
        status: ExecutionStatus::Running,
        started_at: "2026-01-01T00:00:00.000Z".to_string(),
        completed_at: None,
        current_wave_index: 0,
        waves: vec![WaveExecution {
            wave: WaveDefinition {
                id: "w1".to_string(),
                name: None,
                tasks: vec![task.clone()],
                checkpoint: Some(true),
            },
            status: ExecutionStatus::Running,
            tasks: vec![TaskExecution {
                task,
                status: ExecutionStatus::Running,
                started_at: None,
                completed_at: None,
                error: None,
                output_content: None,
            }],
        }],
        variables: BTreeMap::from([("k".to_string(), "v".to_string())]),
    };

    exec.validate().expect("validate");
}

#[test]
fn task_execution_validate_rejects_empty_optional_strings() {
    let base = base_task("t1");
    let mut t = TaskExecution {
        task: base.clone(),
        status: ExecutionStatus::Running,
        started_at: Some("".to_string()),
        completed_at: None,
        error: None,
        output_content: None,
    };
    let err = t.validate().expect_err("should fail");
    assert!(err.contains("started_at"));

    t.started_at = None;
    t.completed_at = Some("".to_string());
    let err = t.validate().expect_err("should fail");
    assert!(err.contains("completed_at"));

    t.completed_at = None;
    t.error = Some("".to_string());
    let err = t.validate().expect_err("should fail");
    assert!(err.contains("error"));

    t.error = None;
    t.output_content = Some("".to_string());
    let err = t.validate().expect_err("should fail");
    assert!(err.contains("output_content"));
}
