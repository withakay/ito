<!-- ITO:START -->
## ADDED Requirements

### Requirement: Signal-based exit codes are classified as retriable

The system SHALL classify exit codes 128 through 143 (signal-based process termination) as retriable, meaning the harness process crashed rather than the agent's work failing.

#### Scenario: Exit code 128 is retriable

- **WHEN** a harness process exits with code 128
- **THEN** the system SHALL classify the exit as retriable

#### Scenario: Exit code 137 (SIGKILL) is retriable

- **WHEN** a harness process exits with code 137
- **THEN** the system SHALL classify the exit as retriable

#### Scenario: Exit code 1 is not retriable

- **WHEN** a harness process exits with code 1
- **THEN** the system SHALL NOT classify the exit as retriable

### Requirement: Retriable exits are retried without counting against error threshold

The system SHALL retry retriable exit codes automatically without incrementing the harness error counter or counting against the error threshold.

#### Scenario: Retriable crash followed by success

- **GIVEN** a Ralph loop with `error_threshold` set to 1
- **WHEN** the harness exits with code 128 on the first iteration
- **AND** the harness succeeds on the second iteration
- **THEN** the loop SHALL continue to completion without error

#### Scenario: Retriable crash with exit-on-error enabled

- **GIVEN** a Ralph loop with `--exit-on-error` enabled
- **WHEN** the harness exits with a retriable code
- **THEN** the loop SHALL retry instead of immediately aborting

### Requirement: Consecutive retriable retries are capped

The system SHALL limit consecutive retriable retries to a maximum of 3 to prevent infinite crash loops.

#### Scenario: Harness crashes repeatedly

- **GIVEN** a Ralph loop running
- **WHEN** the harness crashes with a retriable exit code more than 3 consecutive times
- **THEN** the system SHALL abort with an error message indicating the harness crashed repeatedly

#### Scenario: Successful iteration resets the retry counter

- **GIVEN** a Ralph loop where the harness has crashed once with a retriable code
- **WHEN** the harness succeeds on the next iteration
- **AND** the harness crashes again with a retriable code on a subsequent iteration
- **THEN** the consecutive retry counter SHALL have been reset to zero by the successful iteration

### Requirement: CLI harnesses share a common trait

All CLI-based harness implementations SHALL implement the `CliHarness` trait, which provides a blanket `Harness` implementation for process spawning, streaming I/O, and inactivity monitoring.

#### Scenario: New harness only needs three methods

- **GIVEN** a new CLI-based harness type
- **WHEN** it implements `CliHarness` with `harness_name()`, `binary()`, and `build_args()`
- **THEN** it SHALL automatically receive the full `Harness` trait implementation including `run()`, `stop()`, and `streams_output()`

#### Scenario: All existing CLI harnesses use the trait

- **GIVEN** the claude, codex, copilot, and opencode harnesses
- **THEN** each SHALL implement `CliHarness` rather than implementing `Harness` directly
<!-- ITO:END -->
