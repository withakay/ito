## ADDED Requirements

### Requirement: Archive List Command

Ito SHALL provide a command for listing archived changes.

#### Scenario: List archived changes

- **WHEN** a user runs `ito archive list --json`
- **THEN** Ito SHALL return archived change summaries in JSON
- **AND** each summary SHALL include canonical change ID, archive path, archived date when available, module ID when available, and completion status.

### Requirement: Archive Show Command

Ito SHALL provide a command for showing an archived change without requiring a filesystem path.

#### Scenario: Show archived change by ID

- **WHEN** a user runs `ito archive show <change-id> --json`
- **THEN** Ito SHALL resolve the archived change by full or unambiguous partial ID
- **AND** Ito SHALL return the proposal, specs, tasks path, archive path, and summary metadata.

### Requirement: Active And Archived List Scopes

The list command SHALL support explicit archived and all-change scopes.

#### Scenario: List archived scope

- **WHEN** a user runs `ito list --archived --json`
- **THEN** Ito SHALL list only archived changes.

#### Scenario: List all scopes

- **WHEN** a user runs `ito list --all --json`
- **THEN** Ito SHALL list active and archived changes
- **AND** each item SHALL include a scope field.

### Requirement: Scoped Change Resolution

Change resolution SHALL distinguish active, archived, and all scopes.

#### Scenario: Active miss but archived match

- **WHEN** a command searches only active changes and the target exists only in archive
- **THEN** Ito SHALL return a structured message explaining that the change is archived
- **AND** Ito SHALL suggest the archive-aware command.
