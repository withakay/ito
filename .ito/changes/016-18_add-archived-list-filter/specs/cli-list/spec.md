## MODIFIED Requirements

### Requirement: `ito list-archive` lists archived changes

The CLI SHALL provide both an `ito list-archive` command and an `ito list --archived` filter that list archived changes without requiring users to inspect `.ito/changes/archive` directly.

#### Scenario: List archived changes

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list-archive`
- **THEN** the command lists archived change identifiers
- **AND** active changes are not included

#### Scenario: List archived changes with `ito list --archived`

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list --archived`
- **THEN** the command lists archived change identifiers
- **AND** active changes are not included

#### Scenario: List archived changes as JSON

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list-archive --json`
- **THEN** the command prints a machine-readable JSON object containing archived changes

#### Scenario: List archived changes with `ito list --archived --json`

- **GIVEN** archived changes exist
- **WHEN** the user runs `ito list --archived --json`
- **THEN** the command prints a machine-readable JSON object containing archived changes

#### Scenario: No archived changes

- **GIVEN** no archived changes exist
- **WHEN** the user runs `ito list-archive`
- **THEN** the command reports that no archived changes were found

#### Scenario: No archived changes with `ito list --archived`

- **GIVEN** no archived changes exist
- **WHEN** the user runs `ito list --archived`
- **THEN** the command reports that no archived changes were found
