## ADDED Requirements

### Requirement: Extra validation command CLI flag

The system SHALL accept a `--validation-command` flag to specify an additional validation command beyond project defaults.

#### Scenario: Extra validation command flag accepted

- **WHEN** executing `ito ralph "<prompt>" --validation-command "custom-check" --change <change-id>`
- **THEN** the system SHALL run `custom-check` as an additional validation step after project validation
- **AND** this is in addition to (not replacing) the standard Ito task and project validation

### Requirement: Skip validation CLI flag

The system SHALL accept a `--skip-validation` flag to bypass all validation steps.

#### Scenario: Skip validation flag accepted

- **WHEN** executing `ito ralph "<prompt>" --skip-validation --change <change-id>`
- **THEN** the system SHALL NOT run any validation (task status, project, or extra)
- **AND** the system SHALL accept the completion promise immediately (legacy behavior)
- **AND** the system SHALL print a warning that validation was skipped

## MODIFIED Requirements

### Requirement: Robust completion promise detection

The system SHALL detect the completion promise in harness output even when the promise contains surrounding whitespace and newlines. When detected, the system SHALL validate the completion before accepting it.

#### Scenario: Completion promise detection ignores whitespace

- **GIVEN** `--completion-promise COMPLETE`
- **WHEN** harness output contains `<promise>\nCOMPLETE\n</promise>`
- **THEN** the system SHALL treat the completion promise as detected
- **AND** the system SHALL proceed to validation

#### Scenario: Completion accepted after all validation passes

- **GIVEN** `--completion-promise COMPLETE`
- **AND** `--change <change-id>`
- **WHEN** harness output contains `<promise>COMPLETE</promise>`
- **AND** all tasks for the change are complete or shelved
- **AND** project validation (as configured) passes
- **AND** extra validation (if specified) passes
- **THEN** the system SHALL exit the loop with a success message

#### Scenario: Completion rejected when tasks incomplete

- **GIVEN** `--completion-promise COMPLETE`
- **AND** `--change <change-id>`
- **WHEN** harness output contains `<promise>COMPLETE</promise>`
- **AND** one or more tasks are pending or in-progress
- **THEN** the system SHALL NOT exit the loop
- **AND** the system SHALL proceed to the next iteration
- **AND** the system SHALL inject the incomplete task list as context

#### Scenario: Completion rejected when project validation fails

- **GIVEN** `--completion-promise COMPLETE`
- **WHEN** harness output contains `<promise>COMPLETE</promise>`
- **AND** project validation exits with a non-zero code
- **THEN** the system SHALL NOT exit the loop
- **AND** the system SHALL proceed to the next iteration
- **AND** the system SHALL inject the validation failure as context
