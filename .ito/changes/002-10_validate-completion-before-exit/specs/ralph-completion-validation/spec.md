## ADDED Requirements

### Requirement: Completion validation runs after promise detection

The system SHALL run a configurable validation command after detecting a completion promise, before accepting the completion as valid.

#### Scenario: Validation command succeeds

- **WHEN** the agent outputs the completion promise
- **AND** the validation command exits with code 0
- **THEN** the system SHALL accept the completion and exit the loop successfully

#### Scenario: Validation command fails

- **WHEN** the agent outputs the completion promise
- **AND** the validation command exits with a non-zero code
- **THEN** the system SHALL reject the completion
- **AND** the system SHALL continue to the next iteration
- **AND** the system SHALL inject the validation failure output as context for the next iteration

#### Scenario: Validation command not found

- **WHEN** the agent outputs the completion promise
- **AND** the configured validation command does not exist
- **THEN** the system SHALL warn the user
- **AND** the system SHALL accept the completion (graceful degradation)

### Requirement: Validation failure context injection

The system SHALL inject validation failure details into the next iteration's context so the agent can address the issues.

#### Scenario: Build errors injected as context

- **GIVEN** the validation command output contains error messages
- **WHEN** validation fails after a completion promise
- **THEN** the next iteration prompt SHALL include a section labeled `## Validation Failure (completion rejected)`
- **AND** the section SHALL contain the validation command's stderr/stdout
- **AND** the section SHALL explain that the loop continues until validation passes

### Requirement: Configurable validation command

The system SHALL support customizing the validation command via CLI flag.

#### Scenario: Default validation command

- **WHEN** no `--validation-command` is specified
- **THEN** the system SHALL use `make check` as the default validation command

#### Scenario: Custom validation command

- **WHEN** `--validation-command "npm test"` is specified
- **THEN** the system SHALL run `npm test` for validation instead of the default

#### Scenario: Multi-command validation

- **WHEN** `--validation-command "make check && make test"` is specified
- **THEN** the system SHALL execute the full command string via shell

### Requirement: Validation can be skipped

The system SHALL support skipping validation for backward compatibility or edge cases.

#### Scenario: Skip validation flag

- **WHEN** `--skip-validation` is specified
- **AND** the agent outputs the completion promise
- **THEN** the system SHALL accept the completion immediately without running validation
- **AND** the system SHALL print a warning that validation was skipped

### Requirement: Validation timeout

The system SHALL enforce a timeout on the validation command to prevent infinite hangs.

#### Scenario: Validation times out

- **WHEN** the validation command runs longer than 5 minutes
- **THEN** the system SHALL kill the validation process
- **AND** the system SHALL treat it as a validation failure
- **AND** the system SHALL inject a timeout error message as context
