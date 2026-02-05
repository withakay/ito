## ADDED Requirements

### Requirement: Validation command CLI flag

The system SHALL accept a `--validation-command` flag to specify the command used to validate completion.

#### Scenario: Validation command flag accepted

- **WHEN** executing `ito ralph "<prompt>" --validation-command "npm test" --change <change-id>`
- **THEN** the system SHALL use `npm test` as the validation command
- **AND** the system SHALL run this command after detecting a completion promise

### Requirement: Skip validation CLI flag

The system SHALL accept a `--skip-validation` flag to bypass the validation step.

#### Scenario: Skip validation flag accepted

- **WHEN** executing `ito ralph "<prompt>" --skip-validation --change <change-id>`
- **THEN** the system SHALL NOT run any validation command after detecting a completion promise
- **AND** the system SHALL accept the completion promise immediately (legacy behavior)

## MODIFIED Requirements

### Requirement: Robust completion promise detection

The system SHALL detect the completion promise in harness output even when the promise contains surrounding whitespace and newlines. When detected, the system SHALL validate the completion before accepting it.

#### Scenario: Completion promise detection ignores whitespace

- **GIVEN** `--completion-promise COMPLETE`
- **WHEN** harness output contains `<promise>\nCOMPLETE\n</promise>`
- **THEN** the system SHALL treat the completion promise as detected
- **AND** the system SHALL proceed to run the validation command

#### Scenario: Completion accepted after validation passes

- **GIVEN** `--completion-promise COMPLETE`
- **AND** `--validation-command "make check"` (or default)
- **WHEN** harness output contains `<promise>COMPLETE</promise>`
- **AND** the validation command exits with code 0
- **THEN** the system SHALL exit the loop with a success message

#### Scenario: Completion rejected when validation fails

- **GIVEN** `--completion-promise COMPLETE`
- **WHEN** harness output contains `<promise>COMPLETE</promise>`
- **AND** the validation command exits with a non-zero code
- **THEN** the system SHALL NOT exit the loop
- **AND** the system SHALL proceed to the next iteration
- **AND** the system SHALL inject the validation failure as context
