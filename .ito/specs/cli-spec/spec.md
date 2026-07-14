# Cli Spec Specification

## Purpose

Define the `cli-spec` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Deprecated spec command is hidden

The CLI SHALL treat `ito spec ...` as a deprecated noun-based entrypoint.

#### Scenario: Deprecated spec command remains callable

- **WHEN** users execute `ito spec <subcommand>`
- **THEN** the command executes successfully with its existing behavior
- **AND** prints a deprecation warning pointing to verb-first alternatives (e.g., `ito show`, `ito list --specs`, `ito validate --specs`)

#### Scenario: Deprecated spec command is not shown in help

- **WHEN** users execute `ito --help`
- **THEN** `spec` is not listed as a top-level command

#### Scenario: Deprecated spec command is not suggested in completion

- **WHEN** users use shell completion
- **THEN** `spec` is not suggested as a top-level command
