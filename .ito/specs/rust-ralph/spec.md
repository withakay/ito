<!-- ITO:START -->
## ADDED Requirements

### Requirement: Non-zero harness exits are classified before counting

The Ralph loop SHALL classify non-zero harness exit codes into retriable (signal-based crashes) and non-retriable (logical errors) before applying error threshold or exit-on-error logic.

#### Scenario: Retriable exit skips error counting

- **GIVEN** a Ralph loop with default error handling
- **WHEN** the harness exits with a signal-based code (128-143)
- **THEN** the system SHALL retry the iteration without incrementing the harness error counter
- **AND** the system SHALL NOT feed the crash output back as validation context

#### Scenario: Non-retriable exit counts normally

- **GIVEN** a Ralph loop with default error handling
- **WHEN** the harness exits with a non-retriable code (e.g. 1, 2)
- **THEN** the system SHALL increment the harness error counter
- **AND** the system SHALL feed the failure output back as context for the next iteration
<!-- ITO:END -->
