## ADDED Requirements

### Requirement: Backend runtime configuration resolves from config plus environment

When backend mode is enabled, Ito SHALL resolve backend connection values from config and token value from the configured environment variable.

#### Scenario: Backend token is resolved from configured env var

- **GIVEN** `backend.token_env_var` is configured
- **WHEN** Ito initializes backend client runtime
- **THEN** Ito reads the bearer token from that environment variable

#### Scenario: Missing token fails backend runtime initialization

- **GIVEN** backend mode is enabled
- **AND** configured token environment variable is unset
- **WHEN** Ito initializes backend client runtime
- **THEN** Ito fails fast with an actionable backend-auth configuration error

#### Scenario: Backend disabled does not require token env var

- **GIVEN** backend mode is disabled
- **WHEN** Ito runs commands
- **THEN** Ito does not require backend token environment variables

### Requirement: Backend backups use a per-user directory outside the repo

When backend mode is enabled, Ito SHALL support configuring a per-user backup directory for change artifact snapshots.

#### Scenario: Backup directory is configurable

- **WHEN** the project config sets `backend.backup_dir`
- **THEN** Ito uses that directory for artifact backup snapshots

#### Scenario: Backup directory inside project root is rejected

- **GIVEN** `backend.backup_dir` resolves under the project root
- **WHEN** Ito initializes backend client runtime
- **THEN** Ito fails fast with an actionable configuration error requiring a path outside the repo
