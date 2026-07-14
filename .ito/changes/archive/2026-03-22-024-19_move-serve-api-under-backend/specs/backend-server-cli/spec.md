## ADDED Requirements

### Requirement: Backend server startup is exposed through `ito backend serve`

Ito SHALL expose backend server startup through the backend command group.

The canonical command SHALL be `ito backend serve`.

#### Scenario: User starts backend server through backend command group

- **WHEN** the user runs `ito backend serve`
- **THEN** Ito starts the multi-tenant backend API server
- **AND** the command accepts the same startup flags and config inputs as the prior dev command

### Requirement: Top-level `serve-api` is no longer a supported entrypoint

Ito MUST stop treating `ito serve-api` as a supported command path.

#### Scenario: User invokes removed top-level command

- **WHEN** the user runs `ito serve-api`
- **THEN** Ito exits with actionable guidance telling the user to run `ito backend serve`

### Requirement: Backend CLI help and completion reference the canonical serve command

The CLI help and completion surfaces MUST point users to `ito backend serve` as the backend server entrypoint.

#### Scenario: Backend help shows serve subcommand

- **WHEN** the user runs `ito backend --help`
- **THEN** the help output lists `serve` as a backend subcommand
- **AND** describes it as the way to start the backend API server
