## ADDED Requirements

### Requirement: CLI provides a backend status command

Ito SHALL provide an `ito backend status` command that validates backend configuration, checks server connectivity, and verifies token authentication.

#### Scenario: Backend not enabled

- **GIVEN** `backend.enabled` is `false` in resolved configuration
- **WHEN** user runs `ito backend status`
- **THEN** the command reports that backend mode is disabled
- **AND** exits with a zero exit code

#### Scenario: Backend enabled but config incomplete

- **GIVEN** `backend.enabled` is `true` but required fields (token, org, repo) are missing
- **WHEN** user runs `ito backend status`
- **THEN** the command reports which fields are missing
- **AND** exits with a non-zero exit code

#### Scenario: Backend enabled but server unreachable

- **GIVEN** `backend.enabled` is `true` and backend runtime resolves successfully
- **WHEN** user runs `ito backend status`
- **AND** the health endpoint is unreachable (connection refused, timeout, DNS failure)
- **THEN** the command reports configuration as valid
- **AND** reports the server as unreachable with the connection error
- **AND** exits with a non-zero exit code

#### Scenario: Backend enabled and server healthy

- **GIVEN** `backend.enabled` is `true` and backend runtime resolves successfully
- **WHEN** user runs `ito backend status`
- **AND** the `/api/v1/health` endpoint returns status `ok`
- **AND** the `/api/v1/ready` endpoint returns status `ready`
- **AND** the `/api/v1/projects/{org}/{repo}/auth/verify` endpoint returns 200
- **THEN** the command reports configuration as valid, server as healthy, server as ready, and auth as verified
- **AND** exits with a zero exit code

#### Scenario: Backend enabled, server healthy but auth fails

- **GIVEN** `backend.enabled` is `true` and backend runtime resolves successfully
- **WHEN** user runs `ito backend status`
- **AND** the health and ready endpoints succeed
- **AND** the auth verify endpoint returns 401
- **THEN** the command reports configuration as valid, server as reachable, but auth as failed
- **AND** includes guidance about checking the token or seed
- **AND** exits with a non-zero exit code

#### Scenario: Backend enabled, server healthy but not ready

- **GIVEN** `backend.enabled` is `true` and backend runtime resolves successfully
- **WHEN** user runs `ito backend status`
- **AND** the `/api/v1/health` endpoint returns status `ok`
- **AND** the `/api/v1/ready` endpoint returns status `not_ready`
- **THEN** the command reports configuration as valid, server as healthy, and server as not ready
- **AND** includes the readiness failure reason
- **AND** exits with a non-zero exit code

### Requirement: Backend status command supports JSON output

The `ito backend status` command MUST support a `--json` flag for machine-readable output.

#### Scenario: JSON output with all fields

- **WHEN** user runs `ito backend status --json`
- **THEN** the command outputs a JSON object containing at minimum: `enabled`, `url`, `config_valid`, `config_errors`, `server_reachable`, `server_healthy`, `server_ready`, `server_version`, `auth_verified`, and `token_scope`
- **AND** fields that could not be determined (e.g., version when server is unreachable) SHALL be `null`

### Requirement: CLI provides a token generation command

Ito SHALL provide an `ito backend generate-token` command that derives a project-scoped bearer token from an HMAC seed.

#### Scenario: Generate token with seed from config

- **GIVEN** `backendServer.auth.tokenSeed` is set in the global config
- **WHEN** user runs `ito backend generate-token`
- **AND** org and repo are resolvable from project config or flags
- **THEN** the command outputs the derived HMAC-SHA256 token for that org/repo

#### Scenario: Generate token with explicit seed flag

- **WHEN** user runs `ito backend generate-token --seed <value>`
- **THEN** the seed flag overrides the config value
- **AND** the command outputs the derived token

#### Scenario: Generate token with interactive org/repo prompts

- **GIVEN** org or repo are not set in project config and not provided via flags
- **WHEN** user runs `ito backend generate-token`
- **THEN** the command interactively prompts for the missing values
- **AND** offers to save the entered values to the project config

#### Scenario: Generate token with flag overrides

- **WHEN** user runs `ito backend generate-token --org acme --repo widgets`
- **THEN** the flags override any values in project config
- **AND** environment variables (`ITO_BACKEND_PROJECT_ORG`, `ITO_BACKEND_PROJECT_REPO`) override the flags

### Requirement: Backend status health check uses core health-check client

The health-check logic MUST be implemented in `ito-core` as a reusable function, not inline in the CLI handler.

#### Scenario: Core health-check function is callable independently

- **GIVEN** a `BackendRuntime` is resolved
- **WHEN** the core health-check function is called with the runtime
- **THEN** it returns a structured result with health, readiness, and auth verification status
- **AND** the function is usable by both CLI and programmatic consumers

### Requirement: Backend server provides an auth verify endpoint

The backend server SHALL expose a `GET /api/v1/projects/{org}/{repo}/auth/verify` endpoint that validates the caller's bearer token and returns the token scope.

#### Scenario: Valid admin token

- **GIVEN** a valid admin bearer token
- **WHEN** `GET /api/v1/projects/{org}/{repo}/auth/verify` is called
- **THEN** the server returns 200 with `{"scope": "admin"}`

#### Scenario: Valid project token

- **GIVEN** a valid project-derived bearer token for the requested org/repo
- **WHEN** `GET /api/v1/projects/{org}/{repo}/auth/verify` is called
- **THEN** the server returns 200 with `{"scope": "project", "org": "<org>", "repo": "<repo>"}`

#### Scenario: Invalid token

- **GIVEN** an invalid bearer token
- **WHEN** `GET /api/v1/projects/{org}/{repo}/auth/verify` is called
- **THEN** the server returns 401

### Requirement: Token security warnings in help and errors

The `ito backend` help text and error messages MUST guide users toward secure token practices.

#### Scenario: Warning when token is in committed config

- **GIVEN** `backend.token` is set in `.ito/config.json` (not `.ito/config.local.json`)
- **WHEN** user runs `ito backend status`
- **THEN** the command emits a warning that the token may be committed to git
- **AND** recommends using the `ITO_BACKEND_TOKEN` env var or `.ito/config.local.json` instead

#### Scenario: Help text emphasizes env var usage

- **WHEN** user runs `ito backend --help`
- **THEN** the help output explains the token resolution order (env var overrides config)
- **AND** recommends environment variables for secrets
