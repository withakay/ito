## ADDED Requirements

### Requirement: Test execution performance target

The test suite SHALL complete within 75% of the established baseline wall-clock time when run via the primary test runner.

#### Scenario: Full suite meets performance target

- **WHEN** running the full test suite with `make test`
- **THEN** the wall-clock execution time SHALL be at most 75% of the recorded baseline

### Requirement: Test sleep elimination

Tests SHALL NOT use `thread::sleep` or wall-clock delays for filesystem timestamp ordering. Tests that require distinct file modification times SHALL use explicit timestamp manipulation (e.g., `filetime::set_file_mtime`).

#### Scenario: Timestamp-dependent sort tests use explicit mtime

- **WHEN** a test verifies sort ordering by modification time
- **THEN** the test sets explicit file modification times rather than sleeping between file writes

### Requirement: Timeout test efficiency

Tests that verify timeout behaviour SHALL use the minimum necessary timeout and process duration values. A test verifying that a process is killed after a timeout SHALL NOT spawn a process sleeping more than 10x the timeout duration.

#### Scenario: Timeout test uses minimal sleep duration

- **WHEN** a test spawns a long-running process to verify timeout behaviour
- **THEN** the spawned process duration SHALL be at most 10x the configured timeout

### Requirement: Nextest adoption

The workspace SHALL support `cargo-nextest` as the primary test runner, with `cargo test` as a fallback.

#### Scenario: Makefile test target prefers nextest

- **WHEN** `cargo nextest` is available on PATH
- **THEN** `make test` SHALL use `cargo nextest run`

#### Scenario: Makefile test target falls back to cargo test

- **WHEN** `cargo nextest` is NOT available on PATH
- **THEN** `make test` SHALL use `cargo test`

### Requirement: Crate-level test impact analysis

The workspace SHALL provide a script that identifies which crates are affected by recent changes and runs tests only for those crates and their dependents.

#### Scenario: Only affected crates tested

- **WHEN** running `make test-affected`
- **THEN** only crates with changed source files (and their transitive dependents) SHALL be tested
