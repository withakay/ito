## ADDED Requirements

### Requirement: Gitignore local session state

`ito init` SHALL ensure the repository root `.gitignore` ignores `.ito/session.json`.

#### Scenario: Adding ignore entry during initialization
- **WHEN** `ito init` completes successfully
- **THEN** the repository root `.gitignore` file contains a line `.ito/session.json`

#### Scenario: Creating .gitignore when missing
- **GIVEN** the repository root `.gitignore` file does not exist
- **WHEN** `ito init` completes successfully
- **THEN** `.gitignore` is created
- **AND** the created `.gitignore` contains a line `.ito/session.json`

#### Scenario: Idempotent ignore entry
- **GIVEN** the repository root `.gitignore` file already contains a line `.ito/session.json`
- **WHEN** `ito init` runs again
- **THEN** `.gitignore` is not modified
