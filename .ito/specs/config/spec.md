## MODIFIED Requirements

### Requirement: Configuration schema

The CLI SHALL support a well-defined configuration schema that allows for tool-specific, agent-specific, harness-specific, backend-client, and backend-server settings.

Notes:

- Existing cascading config behavior is preserved.
- Global config at `~/.config/ito/config.json` is supported.

#### Scenario: Configuration schema supports backend client settings

- **WHEN** reading or writing configuration
- **THEN** support the existing backend client configuration structure:
  - `backend.url`: Base URL for the backend API
  - `backend.token`: Authentication token for backend API access
  - `backend.enabled`: Boolean to enable/disable backend integration
  - `backend.project.org`: Organization namespace used in backend routes
  - `backend.project.repo`: Repository namespace used in backend routes

#### Scenario: Configuration schema supports backend server settings

- **WHEN** reading or writing configuration
- **THEN** support a backend server configuration structure:
  - `backendServer.enabled`: Boolean to enable/disable backend server features
  - `backendServer.bind`: Bind address
  - `backendServer.port`: Port
  - `backendServer.dataDir`: Storage root directory
  - `backendServer.storage.kind`: Storage backend selector (`filesystem` | `sqlite`)
  - `backendServer.storage.sqlite.dbPath`: SQLite database file path (required when `kind=sqlite`)
  - `backendServer.http.maxBodyBytes`: Maximum HTTP request body size in bytes
  - `backendServer.cors.origins`: Optional allowed CORS origins list
  - `backendServer.allowed.orgs`: List of allowed organizations (required)
  - `backendServer.allowed.repos.<org>`: Either `*` or a list of allowed repos
  - `backendServer.auth.adminTokens`: List of admin bearer tokens
  - `backendServer.auth.tokenSeed`: Secret seed used to derive per-project tokens via HMAC

#### Scenario: Backend server config is overridable via env vars and CLI args

- **GIVEN** backend server config is supplied by config file
- **WHEN** an environment variable override is provided
- **THEN** the environment variable wins
- **AND** **WHEN** a CLI argument override is provided, it wins over both file and env

#### Scenario: Backend server bind and port have safe defaults and explicit override keys

- **WHEN** `backendServer.bind` is not configured
- **THEN** the backend server binds to `127.0.0.1`
- **AND** **WHEN** `backendServer.port` is not configured
- **THEN** the backend server listens on port `9010`
- **AND** `backendServer.bind` MAY be overridden via `ITO_BACKEND_SERVER_BIND`
- **AND** `backendServer.port` MAY be overridden via `ITO_BACKEND_SERVER_PORT`
- **AND** CLI flags `--bind` and `--port` override both config and env

#### Scenario: Backend server enforces maximum request body size

- **WHEN** `backendServer.http.maxBodyBytes` is not configured
- **THEN** the backend server enforces a default maximum request body size
- **AND** **WHEN** a client sends a request exceeding the maximum size
- **THEN** the backend server rejects the request with an error

#### Scenario: Backend client project namespace is overridable via env vars

- **WHEN** backend client mode is enabled
- **AND** environment variables `ITO_BACKEND_PROJECT_ORG` and `ITO_BACKEND_PROJECT_REPO` are set
- **THEN** the client uses those values for `{org}/{repo}` routing
