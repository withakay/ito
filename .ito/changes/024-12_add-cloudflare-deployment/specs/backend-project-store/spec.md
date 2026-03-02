## ADDED Requirements

### Requirement: Backend provides Cloudflare D1 database storage implementation

The backend MUST provide a Cloudflare D1 database implementation as a project store option for serverless edge deployments.

The D1 implementation SHALL:
- Conform to the same repository abstraction as filesystem and SQLite stores
- Maintain equivalent read/write semantics as other storage backends
- Support org/repo namespace resolution
- Support change, module, and task state persistence
- Handle D1-specific connection and query patterns

#### Scenario: D1 store provides equivalent read behavior to other backends

- **GIVEN** a project with changes and modules exists in D1 storage
- **WHEN** a client requests change and module data through the backend
- **THEN** the backend returns semantically equivalent JSON responses as filesystem or SQLite backends

#### Scenario: D1 store persists change state correctly

- **GIVEN** the backend is configured to use D1 storage
- **WHEN** a client writes change state via the backend API
- **THEN** the change state is persisted to the D1 database
- **AND** subsequent reads return the updated state

#### Scenario: D1 store enforces org/repo allowlist

- **GIVEN** the backend is configured to use D1 storage
- **AND** org `evilcorp` is not in the allowed org list
- **WHEN** a client requests `/api/v1/projects/evilcorp/anything/changes`
- **THEN** the backend returns an authorization error
- **AND** no D1 queries are executed for the disallowed namespace

### Requirement: Backend configuration supports Cloudflare D1 selection

The backend configuration schema MUST support selecting Cloudflare D1 as the project store backend.

Configuration MUST include:
- D1 database binding name or connection details
- D1-specific options (connection pool settings, query timeout, etc.)
- Fallback behavior if D1 is unavailable

#### Scenario: Backend initializes with D1 configuration

- **GIVEN** backend configuration specifies D1 as the project store
- **WHEN** the backend server starts
- **THEN** the backend initializes the D1 repository adapter
- **AND** all project store operations use D1

#### Scenario: Invalid D1 configuration is rejected at startup

- **GIVEN** backend configuration specifies D1 but provides invalid connection details
- **WHEN** the backend server attempts to start
- **THEN** the backend fails to start with a clear error message indicating the D1 configuration issue
