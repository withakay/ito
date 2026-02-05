## MODIFIED Requirements

### Requirement: Timeout monitor thread exits on process completion

The timeout monitor thread SHALL exit when the child process terminates, not only when the inactivity timeout is reached.

#### Scenario: Process exits quickly (before timeout)

- **GIVEN** a harness run with inactivity timeout configured
- **WHEN** the child process exits normally (e.g., command not found, quick completion)
- **THEN** the timeout monitor thread exits within 2 seconds of process termination
- **AND** the harness `run()` method returns promptly

#### Scenario: Process times out due to inactivity

- **GIVEN** a harness run with inactivity timeout of N seconds
- **WHEN** no output is produced for N seconds
- **THEN** the timeout monitor kills the process
- **AND** `timed_out` is set to `true` in the result

### Requirement: Tests complete in reasonable time

The full test suite SHALL complete within 60 seconds on a typical development machine.

#### Scenario: Running all tests

- **WHEN** `cargo test` is executed in the workspace
- **THEN** all tests complete within 60 seconds
- **AND** no individual test takes longer than 10 seconds (unless marked `#[ignore]`)

## ADDED Requirements

### Requirement: Test timing visibility

Test execution SHALL provide timing information for identifying slow tests.

#### Scenario: Identifying slow tests

- **WHEN** running tests with `cargo test -- --show-time`
- **THEN** each test shows its execution duration
- **AND** tests exceeding 1 second are highlighted
