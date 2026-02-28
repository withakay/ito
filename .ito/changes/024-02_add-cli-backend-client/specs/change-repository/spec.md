## ADDED Requirements

### Requirement: ChangeRepository supports backend-backed reads

`ChangeRepository` SHALL support a backend-backed adapter when backend mode is enabled.

#### Scenario: List changes reads from backend in backend mode

- **GIVEN** backend mode is enabled and backend connectivity is healthy
- **WHEN** calling `change_repo.list()`
- **THEN** Ito resolves change summaries from backend state for the configured project

#### Scenario: Get change reads from backend in backend mode

- **GIVEN** backend mode is enabled and a change exists on the backend
- **WHEN** calling `change_repo.get(<change-id>)`
- **THEN** Ito resolves the change from backend state

#### Scenario: Filesystem path is used when backend mode is disabled

- **GIVEN** backend mode is disabled
- **WHEN** calling `change_repo.list()` or `change_repo.get(<change-id>)`
- **THEN** Ito uses existing filesystem-backed repository behavior
