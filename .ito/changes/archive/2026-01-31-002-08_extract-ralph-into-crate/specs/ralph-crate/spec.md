## ADDED Requirements

### Requirement: Ralph is extracted into its own crate

The project SHALL provide a dedicated Rust crate that contains the Ralph loop implementation.

#### Scenario: Ralph crate exists and is used by the CLI

- **WHEN** the workspace is built
- **THEN** a `ito-ralph` crate is built as part of the workspace
- **AND** `ito-cli` uses that crate to implement `ito ralph`

### Requirement: Ralph behavior is preserved

This refactor MUST NOT change user-visible behavior of the `ito ralph` command.

#### Scenario: Ralph still runs and persists state

- **WHEN** a user runs `ito ralph` against a change
- **THEN** the loop executes as before
- **AND** it reads/writes state under `.ito/.state/ralph/<change-id>/`

### Requirement: Ralph tests remain covered

The project SHALL retain automated test coverage for Ralph logic after extraction.

#### Scenario: Tests continue to pass

- **WHEN** CI runs the test suite
- **THEN** Ralph-related tests pass
