# Ralph Crate Specification

## Purpose

Define the `ralph-crate` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Ralph lives in ito-core

The Ralph loop implementation SHALL live in the `ito-core` crate as the `ralph` module.

#### Scenario: Ralph module exists in ito-core

- **WHEN** the workspace is built
- **THEN** `ito-core` provides a `ralph` module
- **AND** `ito-cli` uses `ito_core::ralph` to implement `ito ralph`

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
