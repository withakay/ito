## ADDED Requirements

### Requirement: Configuration can select local SQLite persistence mode

Ito configuration SHALL allow selecting `sqlite` as a client-side persistence mode and SHALL provide the local SQLite runtime settings needed to initialize that mode.

#### Scenario: SQLite mode requires database path configuration

- **GIVEN** the user selects local SQLite persistence mode
- **WHEN** Ito resolves repository runtime configuration
- **THEN** it requires a configured local SQLite database path or equivalent runtime setting

#### Scenario: SQLite mode fails fast on invalid local database configuration

- **GIVEN** local SQLite persistence mode is selected
- **AND** the SQLite runtime configuration is missing or invalid
- **WHEN** Ito resolves the repository runtime
- **THEN** Ito fails fast with an actionable configuration error
