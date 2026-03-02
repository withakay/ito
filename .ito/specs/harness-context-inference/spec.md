## ADDED Requirements

### Requirement: Infer current Ito target from local signals

The system SHALL infer the current Ito target for a harness session as one of:

- A change id (`NNN-CC_name`),
- A module id (`NNN`), or
- No target.

Inference SHALL be deterministic and conservative (prefer returning no target over a false-positive target).

#### Scenario: Infer change id from path

- **GIVEN** the current working directory path contains a change id like `023-07_harness-context-inference`
- **WHEN** the harness requests the inferred target
- **THEN** the system SHALL return target kind `change` with id `023-07_harness-context-inference`

#### Scenario: Infer change id from git branch

- **GIVEN** the current git branch name contains a change id like `023-07_harness-context-inference`
- **WHEN** the harness requests the inferred target
- **THEN** the system SHALL return target kind `change` with id `023-07_harness-context-inference`

### Requirement: Emit a continuation nudge appropriate to the inferred target

The system SHALL emit a concise continuation nudge that points the agent to the next action.

#### Scenario: Change-scoped continuation

- **GIVEN** the inferred target is change `023-07_harness-context-inference`
- **WHEN** the harness requests a continuation nudge
- **THEN** the nudge SHALL include the command `ito tasks next 023-07_harness-context-inference`

#### Scenario: No-target continuation

- **GIVEN** no target can be inferred
- **WHEN** the harness requests a continuation nudge
- **THEN** the nudge SHALL instruct the agent to re-establish a target (for example via `ito list`)

### Requirement: Provide machine-readable output for harnesses

The system SHALL provide machine-readable output suitable for harness hooks and plugins.

#### Scenario: JSON output contains target and nudge

- **GIVEN** a harness requests JSON output
- **WHEN** the system emits the inference result
- **THEN** the output SHALL include the inferred target (or null)
- **AND** the output SHALL include the continuation nudge text
