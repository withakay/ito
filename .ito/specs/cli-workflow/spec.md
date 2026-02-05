# CLI Workflow Specification

## Purpose

The `ito workflow` command group provides YAML-based workflow orchestration capabilities, enabling teams to define, execute, and track complex multi-step workflows with waves, tasks, and checkpoints that work across AI coding assistants.

## Requirements

### Requirement: Workflow initialization

The CLI SHALL initialize the `.ito/workflows/` directory structure with example workflow definitions.

#### Scenario: Initialize workflows directory

- **WHEN** executing `ito workflow init`
- **THEN** create the `.ito/workflows/` directory if it does not exist
- **AND** create the `.ito/workflows/.state/` subdirectory for tracking execution state
- **AND** create `research.yaml` workflow definition with research phase tasks
- **AND** create `execute.yaml` workflow definition with change execution tasks
- **AND** create `review.yaml` workflow definition with adversarial review tasks
- **AND** display a success message indicating the workflow structure has been initialized
- **AND** skip creating any workflow files that already exist to preserve existing content
- **AND** print a hint to run `ito workflow list` to see available workflows

### Requirement: Workflow listing

The CLI SHALL display all available workflow definitions with their descriptions and status.

#### Scenario: List all workflows

- **WHEN** executing `ito workflow list`
- **THEN** scan `.ito/workflows/` for YAML files
- **AND** parse each workflow YAML file to extract name, description, and task count
- **AND** display a table with columns: Name, Description, Tasks, Status
- **AND** check `.ito/workflows/.state/` for execution status
- **AND** display execution status for each workflow (not started, in-progress, completed)
- **AND** print a hint to run `ito workflow show <name>` for details

### Requirement: Workflow display

The CLI SHALL display detailed information about a specific workflow, including waves, tasks, and execution instructions.

#### Scenario: Show workflow details

- **WHEN** executing `ito workflow show <workflow-name>`
- **THEN** parse `.ito/workflows/<workflow-name>.yaml`
- **AND** display the workflow name and description
- **AND** list all waves with their descriptions and task counts
- **AND** for each wave, display tasks with their dependencies, descriptions, and any checkpoint flags
- **AND** show the workflow's execution state (not started, in-progress, completed)
- **AND** display available variables that can be passed to the workflow
- **AND** print an error if the workflow does not exist

### Requirement: Workflow execution

The CLI SHALL execute workflows, tracking progress in state files and generating tool-specific instructions.

#### Scenario: Run a workflow

- **WHEN** executing `ito workflow run <workflow-name> --tool <tool-name> -v <key>=<value>`
- **THEN** parse `.ito/workflows/<workflow-name>.yaml`
- **AND** validate that the tool is supported (opencode, claude-code, codex, etc.)
- **AND** substitute variables into the workflow definition using provided values
- **AND** create or update `.ito/workflows/.state/<workflow-name>.json` with execution start time, current wave, and task status
- **AND** generate tool-specific instructions in markdown format
- **AND** display the generated instructions
- **AND** print a hint that instructions can be saved to a file or passed directly to the AI tool
- **AND** display an error if the workflow does not exist

#### Scenario: Generate tool-specific instructions

- **WHEN** executing `ito workflow run` with a specified tool
- **THEN** generate markdown instructions formatted for the target tool:
  - **OpenCode/Codex**: Format as slash command instructions
  - **Claude Code**: Format as native Claude Code instructions with proper syntax
  - **Other tools**: Format as generic markdown that can be pasted into any AI assistant
- **AND** include workflow context (name, description, variables)
- **AND** list waves and tasks in execution order
- **AND** provide guidance on how to work through tasks in each wave
- **AND** include checkpoint notifications where specified

#### Scenario: Resume workflow execution

- **WHEN** executing `ito workflow run <workflow-name>` and a state file exists
- **THEN** read `.ito/workflows/.state/<workflow-name>.json`
- **AND** determine the last completed task or wave
- **AND** generate instructions starting from the next incomplete task
- **AND** display a message indicating that workflow is being resumed
- **AND** offer option to restart from beginning instead

### Requirement: Workflow status tracking

The CLI SHALL track execution state for workflows, enabling resumption and progress monitoring.

#### Scenario: Show workflow status

- **WHEN** executing `ito workflow status <workflow-name>`
- **THEN** read `.ito/workflows/.state/<workflow-name>.json` if it exists
- **AND** display the workflow's execution status (not started, in-progress, completed)
- **AND** show the current wave and task being executed
- **AND** display progress percentage based on completed tasks
- **AND** show timestamps for start time, last update, and completion (if completed)
- **AND** list completed, pending, and remaining tasks
- **AND** display a message indicating no execution state if state file does not exist

#### Scenario: Update workflow state

- **WHEN** a workflow execution progresses through tasks
- **THEN** update `.ito/workflows/.state/<workflow-name>.json` with:
  - Current wave and task
  - Status of each task (pending, in-progress, complete)
  - Last update timestamp
  - Any variables or outputs generated during execution
- **AND** persist the state file atomically to avoid corruption

### Requirement: Workflow definition format

The CLI SHALL support a YAML workflow definition format with waves, tasks, dependencies, and checkpoints.

#### Scenario: Parse workflow YAML

- **WHEN** reading a workflow YAML file
- **THEN** parse the following structure:
  - `name`: Workflow identifier
  - `description`: Human-readable description
  - `variables`: Optional dictionary of variables with default values
  - `waves`: Array of wave definitions
    - `name`: Wave identifier or description
    - `tasks`: Array of task definitions
      - `id`: Unique task identifier
      - `description`: Task description
      - `dependencies`: Array of task IDs this task depends on (empty if none)
      - `checkpoint`: Boolean flag indicating if task requires human approval
      - `verify`: Optional verification command
- **AND** validate that all task IDs are unique
- **AND** validate that dependencies reference existing task IDs
- **AND** validate that there are no circular dependencies

### Requirement: Workflow validation

The CLI SHALL validate workflow definitions and provide clear error messages for issues.

#### Scenario: Validate workflow YAML

- **WHEN** loading a workflow YAML file
- **THEN** check that the file is valid YAML
- **AND** verify that required fields exist: name, waves
- **AND** validate that waves and tasks are properly structured
- **AND** check for duplicate task IDs
- **AND** check for invalid dependency references
- **AND** check for circular dependencies
- **AND** display specific error messages for any validation failures
- **AND** suggest corrections with examples

### Requirement: Error handling

The CLI SHALL provide clear error messages and recovery suggestions when workflow commands encounter issues.

#### Scenario: Workflow file cannot be read

- **WHEN** `.ito/workflows/<workflow-name>.yaml` cannot be read due to permissions or missing file
- **THEN** display an error message explaining the failure
- **THEN** suggest checking file permissions or running `ito workflow list` to see available workflows
- **AND** exit with code 1

#### Scenario: State file cannot be written

- **WHEN** `.ito/workflows/.state/<workflow-name>.json` cannot be written due to permissions or filesystem errors
- **THEN** display an error message explaining the failure
- **AND** suggest checking directory permissions and disk space
- **AND** continue execution but warn that state will not be persisted

#### Scenario: Invalid tool specified

- **WHEN** executing `ito workflow run` with an unsupported tool
- **THEN** display an error message listing supported tools
- **AND** suggest running with a supported tool name
- **AND** exit with code 2

#### Scenario: Required variable not provided

- **WHEN** executing `ito workflow run` without providing a required variable
- **THEN** display an error message indicating which variable is missing
- **AND** show the variable's description or default value if available
- **AND** suggest providing the variable with `-v <key>=<value>`
- **AND** exit with code 2

### Requirement: Template quality

The CLI SHALL generate high-quality workflow definitions that provide clear guidance and demonstrate best practices.

#### Scenario: Research workflow template

- **WHEN** generating `research.yaml`
- **THEN** define waves for: stack analysis, feature landscape, architecture review, pitfall research, and synthesis
- **AND** include tasks that parallelize investigations where appropriate
- **AND** include a checkpoint task after investigations but before synthesis
- **AND** provide variable for research topic
- **AND** follow the format documented in project-planning-research-proposal.md

#### Scenario: Execute workflow template

- **WHEN** generating `execute.yaml`
- **THEN** define waves for: implementation, testing, verification, and review
- **AND** include tasks for each wave with appropriate dependencies
- **AND** provide variable for change ID
- **AND** include verification commands for critical tasks

#### Scenario: Review workflow template

- **WHEN** generating `review.yaml`
- **THEN** define waves for: security review, scale review, and edge case review
- **AND** include tasks for generating review outputs
- **AND** provide variable for change ID
- **AND** include checkpoint for human approval of findings

## Why

YAML-based workflow orchestration enables teams to define complex, repeatable workflows that work across AI coding assistants. These commands provide:

1. **Workflow definitions**: Declarative YAML format for defining multi-step workflows
1. **Tool integration**: Generate tool-specific instructions from a single workflow definition
1. **State tracking**: Persist execution state across sessions for resumption
1. **Wave-based execution**: Group tasks into waves with dependencies and checkpoints
1. **Team consistency**: Share workflow definitions through version control

Without these tools, teams must manually coordinate complex workflows across sessions and tools, leading to inconsistent execution, lost state, and difficulty collaborating.
