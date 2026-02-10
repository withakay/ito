## MODIFIED Requirements

### Requirement: Workflow initialization

The CLI SHALL treat `ito workflow init` as a no-op and SHALL NOT create or modify workflow template files.

#### Scenario: Workflow init is a no-op

- **WHEN** executing `ito workflow init`
- **THEN** the command SHALL succeed without creating `.ito/workflows/` content
- **AND** it SHALL NOT write `research.yaml`, `execute.yaml`, or `review.yaml`

### Requirement: Workflow listing

The CLI SHALL treat `ito workflow list` as a no-op and SHALL NOT enumerate workflow YAML files.

#### Scenario: Workflow list is a no-op

- **WHEN** executing `ito workflow list`
- **THEN** the command SHALL succeed with no workflow orchestration output
- **AND** it SHALL NOT read or parse `.ito/workflows/*.yaml`

### Requirement: Workflow display

The CLI SHALL treat `ito workflow show` as a no-op and SHALL NOT render workflow details.

#### Scenario: Workflow show is a no-op

- **WHEN** executing `ito workflow show <workflow-name>`
- **THEN** the command SHALL succeed without rendering wave/task detail output
- **AND** it SHALL NOT parse a workflow YAML definition

### Requirement: Workflow execution

The CLI SHALL NOT execute workflow orchestration via `ito workflow run`.

#### Scenario: Workflow run performs no orchestration

- **WHEN** executing `ito workflow run <workflow-name> --tool <tool-name>`
- **THEN** the command SHALL perform no workflow execution
- **AND** it SHALL NOT generate tool-specific orchestration instructions from workflow YAML

### Requirement: Workflow status tracking

The CLI SHALL NOT track workflow state under `.ito/workflows/.state`.

#### Scenario: Workflow status does not read execution state

- **WHEN** executing `ito workflow status <workflow-name>`
- **THEN** the command SHALL perform no workflow-state reporting
- **AND** it SHALL NOT read `.ito/workflows/.state/<workflow-name>.json`

### Requirement: Workflow definition format

The system SHALL NOT treat `.ito/workflows/*.yaml` as an active user workflow contract.

#### Scenario: YAML workflow files are inactive

- **WHEN** users run `ito workflow` commands
- **THEN** YAML workflow definitions SHALL NOT drive behavior
- **AND** the canonical workflow SHALL remain instruction- and skill-driven

### Requirement: Workflow validation

The CLI SHALL NOT provide active validation behavior for legacy workflow YAML through the `ito workflow` command family.

#### Scenario: Workflow commands do not validate YAML

- **WHEN** executing any `ito workflow` subcommand
- **THEN** the command SHALL NOT perform YAML schema/dependency validation

### Requirement: Error handling

The CLI SHALL keep `ito workflow` no-op behavior deterministic and side-effect free.

#### Scenario: No-op commands remain side-effect free

- **WHEN** any `ito workflow` subcommand is invoked repeatedly
- **THEN** command outcomes SHALL be deterministic
- **AND** no new files, state, or orchestration outputs SHALL be produced

### Requirement: Template quality

Workflow guidance quality SHALL be maintained in instruction artifacts and skills rather than standalone workflow templates.

#### Scenario: Guidance quality moves to instruction artifacts

- **WHEN** users consume proposal/apply/review instruction artifacts
- **THEN** the artifacts SHALL provide clear staged guidance equivalent to or better than legacy templates
- **AND** they SHALL include task/checkpoint-oriented direction where applicable

## ADDED Requirements

### Requirement: Workflow commands are explicit no-ops

The CLI SHALL preserve the `ito workflow` command namespace as compatibility no-ops while removing orchestration behavior.

#### Scenario: Root workflow command is a no-op

- **WHEN** a user executes `ito workflow`
- **THEN** the command SHALL succeed as a no-op
- **AND** it SHALL produce no workflow orchestration side effects

#### Scenario: Legacy subcommands are no-ops

- **WHEN** a user executes `ito workflow init|list|show|run|status`
- **THEN** each command SHALL complete as a no-op
- **AND** none SHALL invoke legacy workflow template plumbing
