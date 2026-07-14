## ADDED Requirements

### Requirement: Repository runtime exposes artifact mutation services for every persistence mode

Ito SHALL resolve artifact mutation services through the same runtime-selection mechanism used for repository reads so command handlers can mutate change/spec artifacts without knowing whether persistence is filesystem, SQLite, or remote-backed.

#### Scenario: Filesystem mode selects filesystem-backed artifact mutation services

- **GIVEN** filesystem persistence mode is active
- **WHEN** Ito resolves the repository runtime for an artifact mutation command
- **THEN** it returns filesystem-backed artifact mutation services
- **AND** the command handler uses that contract without directly deciding filesystem layout

#### Scenario: SQLite mode selects SQLite-backed artifact mutation services

- **GIVEN** SQLite persistence mode is active
- **WHEN** Ito resolves the repository runtime for an artifact mutation command
- **THEN** it returns SQLite-backed artifact mutation services
- **AND** the command handler does not require backend HTTP runtime configuration

#### Scenario: Remote mode selects remote-backed artifact mutation services

- **GIVEN** remote persistence mode is active
- **WHEN** Ito resolves the repository runtime for an artifact mutation command
- **THEN** it returns remote-backed artifact mutation services
- **AND** active-work mutation does not require local markdown artifacts as the primary write path
