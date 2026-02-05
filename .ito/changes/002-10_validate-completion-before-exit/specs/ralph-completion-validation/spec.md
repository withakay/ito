## ADDED Requirements

### Requirement: Ito task status validation

The system SHALL verify that all tasks for the change are complete or shelved before accepting a completion promise.

#### Scenario: All tasks complete

- **GIVEN** a Ralph loop running with `--change <change-id>`
- **WHEN** the agent outputs the completion promise
- **AND** all tasks in the change are marked `complete`
- **THEN** the system SHALL proceed to project validation

#### Scenario: All tasks complete or shelved

- **GIVEN** a Ralph loop running with `--change <change-id>`
- **WHEN** the agent outputs the completion promise
- **AND** all tasks are either `complete` or `shelved`
- **THEN** the system SHALL proceed to project validation

#### Scenario: Tasks remain pending

- **GIVEN** a Ralph loop running with `--change <change-id>`
- **WHEN** the agent outputs the completion promise
- **AND** one or more tasks are `pending` or `in-progress`
- **THEN** the system SHALL reject the completion
- **AND** the system SHALL inject the task status summary as context for the next iteration

#### Scenario: No change-id provided

- **GIVEN** a Ralph loop running without `--change`
- **WHEN** the agent outputs the completion promise
- **THEN** the system SHALL skip Ito task validation
- **AND** the system SHALL proceed to project validation

### Requirement: Project validation always runs

The system SHALL always run the project's configured validation commands when a completion promise is detected.

#### Scenario: Project validation succeeds

- **WHEN** the agent outputs the completion promise
- **AND** Ito task validation passes (or is skipped)
- **AND** the project validation commands exit with code 0
- **THEN** the system SHALL proceed to extra validation (if specified) or accept completion

#### Scenario: Project validation fails

- **WHEN** the agent outputs the completion promise
- **AND** the project validation commands exit with a non-zero code
- **THEN** the system SHALL reject the completion
- **AND** the system SHALL inject the validation failure output as context for the next iteration

#### Scenario: Project validation commands from configuration

- **WHEN** the system needs to run project validation
- **THEN** the system SHALL read validation commands from project configuration
- **AND** the system SHALL check the following sources in order: `ito.json`, `.ito/config.json`, `AGENTS.md`, `CLAUDE.md`
- **AND** the system SHALL use the first configured validation command found

#### Scenario: No project validation configured

- **WHEN** no project validation commands are configured
- **THEN** the system SHALL warn the user that no validation is configured
- **AND** the system SHALL proceed without project validation (graceful degradation)

### Requirement: Extra validation command

The system SHALL support an additional explicit validation command via CLI flag.

#### Scenario: Extra validation specified and succeeds

- **GIVEN** `--validation-command "custom-check"`
- **WHEN** all prior validation steps pass
- **AND** the extra validation command exits with code 0
- **THEN** the system SHALL accept the completion

#### Scenario: Extra validation specified and fails

- **GIVEN** `--validation-command "custom-check"`
- **WHEN** all prior validation steps pass
- **AND** the extra validation command exits with a non-zero code
- **THEN** the system SHALL reject the completion
- **AND** the system SHALL inject the failure output as context

### Requirement: Validation failure context injection

The system SHALL inject validation failure details into the next iteration's context so the agent can address the issues.

#### Scenario: Task status failure injected as context

- **GIVEN** task validation fails due to incomplete tasks
- **WHEN** the next iteration starts
- **THEN** the prompt SHALL include a section labeled `## Validation Failure (completion rejected)`
- **AND** the section SHALL list the incomplete tasks with their status
- **AND** the section SHALL explain that all tasks must be complete or shelved

#### Scenario: Build/test errors injected as context

- **GIVEN** project validation fails with error output
- **WHEN** the next iteration starts
- **THEN** the prompt SHALL include a section labeled `## Validation Failure (completion rejected)`
- **AND** the section SHALL contain the validation command's stderr/stdout
- **AND** the section SHALL explain that the loop continues until validation passes

### Requirement: Validation can be skipped

The system SHALL support skipping all validation for backward compatibility or edge cases.

#### Scenario: Skip validation flag

- **WHEN** `--skip-validation` is specified
- **AND** the agent outputs the completion promise
- **THEN** the system SHALL accept the completion immediately without running any validation
- **AND** the system SHALL print a warning that validation was skipped

### Requirement: Validation timeout

The system SHALL enforce a timeout on validation commands to prevent infinite hangs.

#### Scenario: Validation command times out

- **WHEN** a validation command runs longer than 5 minutes
- **THEN** the system SHALL kill the validation process
- **AND** the system SHALL treat it as a validation failure
- **AND** the system SHALL inject a timeout error message as context
