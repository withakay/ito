## ADDED Requirements

### Requirement: Backend-mode archive materializes an immutable archived change in git

When backend mode is enabled, `ito archive <change-id>` SHALL ensure the final change artifacts are present in the repo and archived under `.ito/changes/archive/`.

#### Scenario: Archive pulls final backend artifacts before archiving

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito archive <change-id>`
- **THEN** Ito pulls the backend artifact bundle for `<change-id>` into the local change directory before applying the archive workflow

#### Scenario: Archived change is present in repo

- **WHEN** backend-mode archiving completes successfully
- **THEN** the archived change exists under `.ito/changes/archive/`
- **AND** the archived artifacts are immutable from the backend perspective

### Requirement: Archive marks backend change status as archived

When backend mode is enabled and local archive succeeds, Ito SHALL mark the change archived on the backend.

#### Scenario: Backend archived status is set after local archive

- **GIVEN** backend mode is enabled
- **WHEN** local archive succeeds for `<change-id>`
- **THEN** Ito calls the backend archive operation for `<change-id>`
- **AND** subsequent backend reads show the change is archived
