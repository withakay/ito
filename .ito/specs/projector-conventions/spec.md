# Projector Conventions Specification

## Purpose

Define the `projector-conventions` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Verb–Noun CLI Command Structure

Ito CLI design SHALL use verbs as top-level commands with nouns provided as arguments or flags for scoping.

#### Scenario: Verb-first command discovery

- **WHEN** a user runs a command like `ito list`
- **THEN** the verb communicates the action clearly
- **AND** nouns refine scope via flags or arguments (e.g., `--changes`, `--specs`)

#### Scenario: Backward compatibility for noun commands

- **WHEN** users run noun-prefixed commands such as `ito spec ...`, `ito change ...`, `ito config ...`, `ito module ...`, `ito completion ...`, or `ito skills ...`
- **THEN** the CLI SHALL continue to support them for at least one release
- **AND** display a deprecation warning that points to verb-first alternatives

#### Scenario: Deprecated commands hidden from help and completion

- **GIVEN** a command entrypoint is deprecated compatibility
- **WHEN** a user runs `ito --help` or uses shell completion
- **THEN** the deprecated entrypoint is not shown/suggested
- **AND** the preferred verb-first entrypoint is shown/suggested instead

#### Scenario: Disambiguation guidance

- **WHEN** item names are ambiguous between changes and specs
- **THEN** `ito show` and `ito validate` SHALL accept `--type spec|change`
- **AND** the help text SHALL document this clearly
