use ito_schemas::WorkflowDefinition;
use std::path::{Path, PathBuf};

pub fn workflows_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("workflows")
}

pub fn workflow_state_dir(ito_path: &Path) -> PathBuf {
    workflows_dir(ito_path).join(".state")
}

pub fn commands_dir(ito_path: &Path) -> PathBuf {
    ito_path.join("commands")
}

pub fn workflow_file_path(ito_path: &Path, name: &str) -> PathBuf {
    workflows_dir(ito_path).join(format!("{name}.yaml"))
}

pub fn init_workflow_structure(ito_path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(workflows_dir(ito_path))?;
    std::fs::create_dir_all(workflow_state_dir(ito_path))?;
    std::fs::create_dir_all(commands_dir(ito_path))?;

    std::fs::write(
        workflow_file_path(ito_path, "research"),
        research_workflow_template(),
    )?;
    std::fs::write(
        workflow_file_path(ito_path, "execute"),
        execute_workflow_template(),
    )?;
    std::fs::write(
        workflow_file_path(ito_path, "review"),
        review_workflow_template(),
    )?;
    Ok(())
}

pub fn list_workflows(ito_path: &Path) -> Vec<String> {
    let dir = workflows_dir(ito_path);
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut out: Vec<String> = Vec::new();
    for e in entries.flatten() {
        let p = e.path();
        if !p.is_file() {
            continue;
        }
        let Some(ext) = p.extension().and_then(|s| s.to_str()) else {
            continue;
        };
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let Some(stem) = p.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        out.push(stem.to_string());
    }
    out.sort();
    out
}

pub fn load_workflow(ito_path: &Path, name: &str) -> Result<WorkflowDefinition, String> {
    let p = workflow_file_path(ito_path, name);
    let contents = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
    serde_yaml::from_str::<WorkflowDefinition>(&contents).map_err(|e| e.to_string())
}

pub fn count_tasks(wf: &WorkflowDefinition) -> usize {
    wf.waves.iter().map(|w| w.tasks.len()).sum()
}

fn research_workflow_template() -> &'static str {
    r#"# Research Workflow
# Parallel domain investigation before proposal creation

version: "1.0"
id: research
name: Domain Research
description: Investigate domain knowledge, stack options, architecture patterns, and pitfalls before creating a proposal.

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

      - id: feature-landscape
        name: Feature Landscape
        agent: research
        prompt: commands/research-features.md
        output: research/investigations/feature-landscape.md
        context:
          topic: "{{topic}}"

      - id: architecture
        name: Architecture Patterns
        agent: research
        prompt: commands/research-architecture.md
        output: research/investigations/architecture.md
        context:
          topic: "{{topic}}"

      - id: pitfalls
        name: Pitfall Research
        agent: research
        prompt: commands/research-pitfalls.md
        output: research/investigations/pitfalls.md
        context:
          topic: "{{topic}}"

  - id: synthesize
    name: Synthesize Findings
    tasks:
      - id: summary
        name: Create Research Summary
        agent: planning
        prompt: commands/research-synthesize.md
        inputs:
          - research/investigations/stack-analysis.md
          - research/investigations/feature-landscape.md
          - research/investigations/architecture.md
          - research/investigations/pitfalls.md
        output: research/SUMMARY.md

on_complete:
  update_state: true
"#
}

fn execute_workflow_template() -> &'static str {
    r#"# Execute Workflow
# Execute tasks from a change proposal

version: "1.0"
id: execute
name: Task Execution
description: Execute tasks from an Ito change proposal, wave by wave.

requires:
  variables:
    - change_id
  files:
    - changes/{{change_id}}/tasks.md

context_files:
  - planning/STATE.md
  - planning/PROJECT.md

waves:
  - id: execute-tasks
    name: Execute Change Tasks
    tasks:
      - id: executor
        name: Task Executor
        agent: execution
        prompt: commands/execute-task.md
        inputs:
          - changes/{{change_id}}/tasks.md
          - changes/{{change_id}}/proposal.md
        context:
          change_id: "{{change_id}}"

on_complete:
  update_state: true
  update_roadmap: true
"#
}

fn review_workflow_template() -> &'static str {
    r#"# Review Workflow
# Adversarial review of a change proposal

version: "1.0"
id: review
name: Adversarial Review
description: Stress-test a proposal from security, scale, and edge case perspectives.

requires:
  variables:
    - change_id
  files:
    - changes/{{change_id}}/proposal.md

context_files:
  - planning/PROJECT.md

waves:
  - id: parallel-review
    name: Parallel Reviews
    tasks:
      - id: security-review
        name: Security Review
        agent: review
        prompt: commands/review-security.md
        inputs:
          - changes/{{change_id}}/proposal.md
          - changes/{{change_id}}/spec.md
        output: changes/{{change_id}}/reviews/security.md
        context:
          change_id: "{{change_id}}"

      - id: scale-review
        name: Scale Review
        agent: review
        prompt: commands/review-scale.md
        inputs:
          - changes/{{change_id}}/proposal.md
          - changes/{{change_id}}/spec.md
        output: changes/{{change_id}}/reviews/scale.md
        context:
          change_id: "{{change_id}}"

      - id: edge-review
        name: Edge Case Review
        agent: review
        prompt: commands/review-edge.md
        inputs:
          - changes/{{change_id}}/proposal.md
          - changes/{{change_id}}/spec.md
        output: changes/{{change_id}}/reviews/edge-cases.md
        context:
          change_id: "{{change_id}}"

  - id: review-checkpoint
    name: Review Checkpoint
    checkpoint: true
    tasks:
      - id: compile-review
        name: Compile Review Summary
        agent: planning
        prompt: commands/review-compile.md
        inputs:
          - changes/{{change_id}}/reviews/security.md
          - changes/{{change_id}}/reviews/scale.md
          - changes/{{change_id}}/reviews/edge-cases.md
        output: changes/{{change_id}}/REVIEW.md

on_complete:
  update_state: true
"#
}
