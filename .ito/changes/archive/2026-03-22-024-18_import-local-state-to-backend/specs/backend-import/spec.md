## ADDED Requirements

### Requirement: CLI imports local active and archived changes into backend state

When backend mode is configured, Ito SHALL provide a command that imports local active and archived change artifacts into backend-managed state.

The command SHALL be `ito backend import`.

#### Scenario: Import uploads active and archived local changes

- **GIVEN** backend mode is enabled
- **AND** local changes exist under `.ito/changes/` and `.ito/changes/archive/`
- **WHEN** the user runs `ito backend import`
- **THEN** Ito imports both active and archived changes into backend-managed storage
- **AND** the command reports imported and skipped counts

#### Scenario: Import in local mode is rejected

- **GIVEN** backend mode is disabled
- **WHEN** the user runs `ito backend import`
- **THEN** Ito exits with an actionable error indicating backend mode is required

### Requirement: Import supports dry-run preview

Ito MUST allow users to preview import scope without mutating backend-managed state.

#### Scenario: Dry-run reports scope without writes

- **GIVEN** backend mode is enabled
- **AND** local changes exist to import
- **WHEN** the user runs `ito backend import --dry-run`
- **THEN** Ito reports the active and archived changes it would import
- **AND** backend-managed state remains unchanged

### Requirement: Import is idempotent and resumable

Repeated imports MUST be safe to rerun when a project has already been partially or fully migrated.

#### Scenario: Rerun skips already imported changes safely

- **GIVEN** local changes were previously imported into backend state
- **WHEN** the user runs `ito backend import` again
- **THEN** Ito does not duplicate imported changes
- **AND** reports already-imported items as skipped or unchanged

#### Scenario: Rerun can continue after partial failure

- **GIVEN** a previous import stopped after importing only some changes
- **WHEN** the user reruns `ito backend import`
- **THEN** Ito resumes by importing the remaining changes
- **AND** preserves the items that already succeeded
