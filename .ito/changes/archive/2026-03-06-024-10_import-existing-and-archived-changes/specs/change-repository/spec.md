## ADDED Requirements

### Requirement: Backend-backed repository exposes imported lifecycle state

After local-to-backend import completes, `ChangeRepository` backend reads SHALL expose imported active and archived changes as canonical backend state.

#### Scenario: Imported active changes are discoverable from backend

- **GIVEN** local active changes were imported successfully
- **WHEN** calling `change_repo.list()` in backend mode
- **THEN** imported active changes appear in the returned change summaries
- **AND** `change_repo.get(<change-id>)` returns full imported artifacts for those changes

#### Scenario: Imported archived changes remain visible as archived history

- **GIVEN** local archived changes were imported successfully
- **WHEN** calling `change_repo.list()` in backend mode
- **THEN** imported archived changes are returned with archived lifecycle metadata
- **AND** their artifacts remain readable through backend-backed `change_repo.get(<change-id>)`

#### Scenario: Backend reads do not require local change files after cutover

- **GIVEN** changes were imported and backend mode is enabled
- **WHEN** local change directories are absent or stale
- **THEN** `change_repo.list()` and `change_repo.get(<change-id>)` continue to resolve from backend state
