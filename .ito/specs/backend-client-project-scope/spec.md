## ADDED Requirements

### Requirement: Backend client requests are scoped to a configured org/repo project namespace

When backend client mode is enabled, Ito clients MUST address backend project state via a configured `{org}/{repo}` namespace.

Clients MUST include `{org}` and `{repo}` in backend API request paths under `/api/v1/projects/{org}/{repo}/...`.

#### Scenario: Backend client is configured with org and repo

- **GIVEN** backend client mode is enabled
- **WHEN** the client loads configuration
- **THEN** it resolves `backend.project.org` and `backend.project.repo`

#### Scenario: Missing org/repo configuration is an error

- **GIVEN** backend client mode is enabled
- **AND** `backend.project.org` or `backend.project.repo` is missing
- **WHEN** the client attempts a backend API operation
- **THEN** the client returns an error indicating the backend project namespace is not configured

### Requirement: Backend client project namespace is overridable via env vars

The backend client MUST allow overriding the project namespace via environment variables to support CI and ephemeral environments.

#### Scenario: Env vars override config file

- **GIVEN** `backend.project.org` and `backend.project.repo` are set in config
- **AND** environment variables `ITO_BACKEND_PROJECT_ORG` and `ITO_BACKEND_PROJECT_REPO` are set
- **WHEN** the client resolves the backend project namespace
- **THEN** it uses the environment variable values
