## ADDED Requirements

### Requirement: Deprecated change command is hidden

The CLI SHALL treat `ito change ...` as a deprecated noun-based entrypoint.

#### Scenario: Deprecated change command remains callable

- **WHEN** users execute `ito change <subcommand>`
- **THEN** the command executes successfully with its existing behavior
- **AND** prints a deprecation warning pointing to verb-first alternatives (e.g., `ito show`, `ito list`, `ito validate`)

#### Scenario: Deprecated change command is not shown in help

- **WHEN** users execute `ito --help`
- **THEN** `change` is not listed as a top-level command

#### Scenario: Deprecated change command is not suggested in completion

- **WHEN** users use shell completion
- **THEN** `change` is not suggested as a top-level command
