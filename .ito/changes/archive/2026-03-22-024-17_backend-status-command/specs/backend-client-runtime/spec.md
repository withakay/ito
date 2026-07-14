## MODIFIED Requirements

### Requirement: Backend client runtime is configuration-gated

Ito SHALL initialize a backend API client only when backend mode is enabled in resolved configuration. When backend mode is enabled, configuration MUST be complete — incomplete configuration is a hard error, never a silent fallback.

#### Scenario: Backend mode enabled initializes client

- **GIVEN** `backend.enabled=true` and required backend settings are present
- **WHEN** Ito starts a backend-aware command
- **THEN** Ito initializes a backend client using configured base URL and project scope

#### Scenario: Backend mode disabled skips client

- **GIVEN** `backend.enabled=false`
- **WHEN** Ito starts a command
- **THEN** Ito does not initialize a backend client
- **AND** command behavior continues through filesystem pathways

#### Scenario: Backend mode enabled but config incomplete is a hard error

- **GIVEN** `backend.enabled=true` but one or more required fields (token, org, repo) are missing or empty
- **WHEN** Ito starts any command that checks backend configuration
- **THEN** Ito reports a visible error identifying the missing fields
- **AND** does NOT silently fall back to filesystem pathways

#### Scenario: Best-effort callsites emit visible warnings on backend failure

- **GIVEN** `backend.enabled=true` and runtime resolves successfully
- **WHEN** a best-effort backend operation (event forwarding, post-mutation sync, artifact materialization) fails due to a network or server error
- **THEN** Ito emits a visible warning (not just a tracing log) indicating the backend operation failed
- **AND** the primary command continues (the warning does not block the user's action)

## ADDED Requirements

### Requirement: Config override precedence is consistent

All backend configuration fields that support multiple sources MUST follow a consistent precedence order: environment variable overrides flag, flag overrides config file value.

#### Scenario: Environment variable overrides flag and config

- **GIVEN** `backend.project.org` is set in config, `--org` is passed as a flag, and `ITO_BACKEND_PROJECT_ORG` is set
- **WHEN** Ito resolves the org value
- **THEN** the environment variable value is used
