## ADDED Requirements

### Requirement: `ito list-archive` lists archived changes

The CLI SHALL provide an `ito list-archive` command that lists archived changes without requiring users to inspect `.ito/changes/archive` directly.

#### Scenario: List archived changes

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list-archive`
- **THEN** the command lists archived change identifiers
- **AND** active changes are not included

#### Scenario: List archived changes as JSON

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list-archive --json`
- **THEN** the command prints a machine-readable JSON object containing archived changes

#### Scenario: No archived changes

- **GIVEN** no archived changes exist
- **WHEN** the user runs `ito list-archive`
- **THEN** the command reports that no archived changes were found
