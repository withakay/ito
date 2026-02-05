## ADDED Requirements

### Requirement: Ralph loop command

The system SHALL provide a `ito ralph` command (alias: `ito loop`) that runs an iterative agent loop.

#### Scenario: Run against a change proposal

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop`
- **THEN** the system loads `.ito/changes/002-01_add-ralph-loop/proposal.md` as primary context
- **AND** the system runs the selected harness at least once

#### Scenario: Alias command

- **WHEN** executing `ito loop "<prompt>" --change 002-01_add-ralph-loop`
- **THEN** the system behaves identically to `ito ralph`

### Requirement: Change/module targeting defaults

The command SHALL support explicit targeting via `--change` and `--module`.

#### Scenario: Resolve module from change

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop`
- **THEN** the system infers module id `002` from the change identifier

#### Scenario: Interactive selection when omitted

- **GIVEN** stdin is a TTY and `--no-interactive` is not set
- **WHEN** executing `ito ralph "<prompt>"` without `--change`
- **THEN** the system prompts the user to select an active change

#### Scenario: Non-interactive error when omitted

- **GIVEN** stdin is not a TTY or `--no-interactive` is set
- **WHEN** executing `ito ralph "<prompt>"` without `--change`
- **THEN** the system prints a helpful error indicating `--change` is required
- **AND** sets a failing exit code

### Requirement: Harness selection and model

The command SHALL support selecting an agent harness and model.

#### Scenario: Use OpenCode harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --harness opencode`
- **THEN** the system invokes `opencode run` to execute the prompt

#### Scenario: Pass model to harness

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --model anthropic/claude-sonnet`
- **THEN** the system passes the model identifier to the selected harness

### Requirement: Loop control and completion promise

The loop SHALL run until a completion promise is detected or `--max-iterations` is reached.

#### Scenario: Completion promise ends the loop

- **WHEN** the harness output contains `<promise>COMPLETE</promise>`
- **THEN** the system stops iterating (subject to `--min-iterations`)

#### Scenario: Minimum iterations

- **GIVEN** `--min-iterations 3`
- **WHEN** the completion promise is detected on iteration 1
- **THEN** the system continues iterating until at least iteration 3 completes

### Requirement: Per-change state persistence

The system SHALL persist loop state and context per change.

#### Scenario: State stored per change

- **WHEN** running `ito ralph` with `--change 002-01_add-ralph-loop`
- **THEN** the system writes loop state under `.ito/.state/ralph/002-01_add-ralph-loop/`

#### Scenario: Status command

- **WHEN** executing `ito ralph --status --change 002-01_add-ralph-loop`
- **THEN** the system prints the current iteration and recent history for that change

### Requirement: Safety and permissions

The command SHALL support a non-interactive approval mode.

#### Scenario: Allow-all flag enables auto-approval

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --allow-all`
- **THEN** the system configures the harness to auto-approve tool permissions

#### Scenario: Allow-all aliases

- **WHEN** executing `ito ralph "<prompt>" --change 002-01_add-ralph-loop --yolo`
- **THEN** the system behaves as if `--allow-all` was provided
