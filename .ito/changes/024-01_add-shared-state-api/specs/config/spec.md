## ADDED Requirements

### Requirement: Configuration supports backend connectivity

The configuration schema SHALL support backend connectivity settings for shared-state API mode.

Supported keys SHALL include:

- `backend.enabled` (boolean)
- `backend.base_url` (string URL)
- `backend.project_id` (string)
- `backend.token_env_var` (string, default `ITO_BACKEND_TOKEN`)
- `backend.request_timeout_ms` (integer)
- `backend.data_dir` (string path, per-user backend data directory for local backend deployments)
- `backend.backup_dir` (string path, per-user backup/snapshot directory outside the repo)

Defaults SHOULD be XDG-compatible on Unix-like systems (for example under `~/.local/share/ito/`) and SHOULD be overridable.

#### Scenario: Backend config keys load from project config

- **WHEN** a project defines backend keys in `.ito/config.json`
- **THEN** Ito loads those keys into resolved configuration
- **AND** missing backend keys use defaults

#### Scenario: Token resolves from configured environment variable

- **GIVEN** `backend.token_env_var` is set to `ITO_BACKEND_TOKEN`
- **WHEN** Ito resolves backend authentication credentials
- **THEN** Ito reads the token from the `ITO_BACKEND_TOKEN` environment variable

#### Scenario: Backend mode disabled bypasses backend client

- **WHEN** `backend.enabled` is false
- **THEN** Ito uses existing filesystem-based behavior
- **AND** backend connectivity settings are not required

#### Scenario: Backend base URL is not constrained to loopback

- **WHEN** backend mode is enabled
- **THEN** `backend.base_url` MAY point to a remote host
- **AND** Ito does not assume `127.0.0.1` or local-only addressing
