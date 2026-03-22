## ADDED Requirements

### Requirement: Repository runtime supports a local SQLite mode

The repository runtime SHALL support `sqlite` as a client-side persistence mode in addition to `filesystem` and `remote`.

#### Scenario: SQLite mode selects SQLite-backed repositories

- **GIVEN** local SQLite persistence mode is active
- **WHEN** Ito resolves the repository runtime
- **THEN** it returns SQLite-backed repository implementations
- **AND** command handlers continue to use the same repository contracts as in other modes

#### Scenario: SQLite mode does not require remote transport

- **GIVEN** local SQLite persistence mode is active
- **WHEN** Ito constructs repositories for a command
- **THEN** it does not require backend HTTP runtime configuration
- **AND** it does not call remote transport adapters for normal repository operations

#### Scenario: SQLite-backed repositories are shared with backend-server composition

- **GIVEN** SQLite-backed persistence is selected
- **WHEN** Ito uses SQLite mode locally and the backend server uses SQLite-backed storage remotely
- **THEN** both paths compose the same concrete SQLite-backed repository implementations
- **AND** the backend server adds HTTP transport without duplicating repository behavior
