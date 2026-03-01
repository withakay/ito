## MODIFIED Requirements

### Requirement: Backend State API provides HTTP access to project state

The backend SHALL expose a RESTful HTTP API that provides read and write access to Ito project state (changes, tasks, modules) via JSON.

The API MUST be multi-tenant and MUST scope all project state endpoints under a project namespace:

`/api/v1/projects/{org}/{repo}/...`

The API SHALL be backed by domain repository ports (`ChangeRepository`, `TaskRepository`, `ModuleRepository`) but MUST NOT assume a local git checkout is present on the backend host.

#### Scenario: List all changes via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes`
- **THEN** the backend returns a JSON array of `ChangeSummary` objects
- **AND** the response status is 200

#### Scenario: Get a single change via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}`
- **AND** the change exists
- **THEN** the backend returns a JSON representation of the full `Change` object
- **AND** the response status is 200

#### Scenario: List all modules via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/modules`
- **THEN** the backend returns a JSON array of module summary objects
- **AND** the response status is 200

#### Scenario: Get a single module via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/modules/{module_id}`
- **AND** the module exists
- **THEN** the backend returns a JSON representation of the module
- **AND** the response status is 200

#### Scenario: List tasks for a change via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/tasks`
- **AND** the change exists
- **THEN** the backend returns a JSON object with task items, progress info, and format metadata
- **AND** the response status is 200

#### Scenario: Get change manifest via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}`
- **THEN** the backend returns a JSON manifest for the change
- **AND** the manifest includes the set of available artifacts (proposal, design if present, tasks, and spec delta documents)
- **AND** each listed artifact includes metadata sufficient for change detection (`revision` and/or `integrity.body_sha256`, and `updated_at`)

#### Scenario: Read a single artifact via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/artifacts/{artifact_name}`
- **THEN** the backend returns the artifact as `text/markdown`
- **AND** the returned Markdown includes YAML front matter with metadata

#### Scenario: List spec delta documents for a change

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/specs`
- **THEN** the backend returns a JSON array of spec delta entries
- **AND** each entry includes `capability` and change detection metadata

#### Scenario: Read a single spec delta via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/specs/{capability}`
- **THEN** the backend returns the spec delta as `text/markdown`
- **AND** the returned Markdown includes YAML front matter with metadata

#### Scenario: Read an artifact bundle via API

- **WHEN** a client sends `GET /api/v1/projects/{org}/{repo}/changes/{change_id}/bundle`
- **THEN** the backend returns a JSON bundle containing all artifacts for the change
- **AND** the bundle contains all spec delta documents as a list keyed by capability

### Requirement: Backend supports conditional reads and header-only checks

The backend SHALL support efficient change detection through HTTP headers.

At minimum, artifact and spec read endpoints MUST support:

- `HEAD` requests that return metadata headers without a response body
- `ETag` headers that reflect the current revision of the returned content
- `If-None-Match` conditional requests that return `304 Not Modified` when content is unchanged

#### Scenario: HEAD artifact returns ETag

- **WHEN** a client sends `HEAD /api/v1/projects/{org}/{repo}/changes/{change_id}/artifacts/{artifact_name}`
- **THEN** the backend returns status 200
- **AND** includes an `ETag` header

#### Scenario: Conditional GET returns 304

- **GIVEN** the client has a previous `ETag` value for an artifact
- **WHEN** the client sends `GET` with `If-None-Match: <etag>` for the same artifact
- **AND** the artifact has not changed
- **THEN** the backend returns status `304 Not Modified`

### Requirement: Backend API uses versioned URL prefix

The backend API SHALL use a versioned URL prefix (`/api/v1/`) to allow future API evolution without breaking existing clients.

#### Scenario: All API endpoints share versioned prefix

- **WHEN** any backend API endpoint is accessed
- **THEN** the URL path starts with `/api/v1/`

### Requirement: Backend serves health and readiness endpoints

The backend SHALL expose health and readiness endpoints for operational monitoring.

#### Scenario: Health check endpoint

- **WHEN** a client sends `GET /api/v1/health`
- **THEN** the backend returns `{"status": "ok"}` with status 200

#### Scenario: Readiness endpoint validates server storage access

- **WHEN** a client sends `GET /api/v1/ready`
- **THEN** the backend returns `{"status": "ready"}` with status 200

### Requirement: Backend starts via CLI subcommand

The backend SHALL be startable via an `ito serve-api` CLI subcommand.

#### Scenario: Start backend with default settings

- **WHEN** a user runs `ito serve-api`
- **THEN** the backend starts listening on `127.0.0.1:9010`
- **AND** uses backend server configuration to select storage and auth
