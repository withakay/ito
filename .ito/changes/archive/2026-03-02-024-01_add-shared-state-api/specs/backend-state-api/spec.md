## ADDED Requirements

### Requirement: Backend State API provides HTTP access to project state

The backend SHALL expose a RESTful HTTP API that provides read and write access to Ito project state (changes, tasks, modules) via JSON. The API SHALL be backed by the existing domain repository ports (`ChangeRepository`, `TaskRepository`, `ModuleRepository`) from `ito-domain`.

#### Scenario: List all changes via API

- **WHEN** a client sends `GET /api/v1/changes`
- **THEN** the backend returns a JSON array of `ChangeSummary` objects
- **AND** each summary includes `id`, `module_id`, `completed_tasks`, `total_tasks`, `work_status`, and `last_modified`
- **AND** the response status is 200

#### Scenario: Get a single change via API

- **WHEN** a client sends `GET /api/v1/changes/{change_id}`
- **AND** the change exists
- **THEN** the backend returns a JSON representation of the full `Change` object
- **AND** the response includes proposal content, design content, spec deltas, and task list
- **AND** the response status is 200

#### Scenario: Get a non-existent change via API

- **WHEN** a client sends `GET /api/v1/changes/{change_id}`
- **AND** the change does not exist
- **THEN** the backend returns a 404 status with an error message

#### Scenario: List all modules via API

- **WHEN** a client sends `GET /api/v1/modules`
- **THEN** the backend returns a JSON array of `ModuleSummary` objects
- **AND** each summary includes `id`, `name`, and `change_count`
- **AND** the response status is 200

#### Scenario: Get a single module via API

- **WHEN** a client sends `GET /api/v1/modules/{module_id}`
- **AND** the module exists
- **THEN** the backend returns a JSON representation of the `Module` object
- **AND** the response status is 200

#### Scenario: List tasks for a change via API

- **WHEN** a client sends `GET /api/v1/changes/{change_id}/tasks`
- **AND** the change exists
- **THEN** the backend returns a JSON object with task items, progress info, and format metadata
- **AND** the response status is 200

### Requirement: Backend API uses versioned URL prefix

The backend API SHALL use a versioned URL prefix (`/api/v1/`) to allow future API evolution without breaking existing clients.

#### Scenario: All API endpoints share versioned prefix

- **WHEN** any API endpoint is accessed
- **THEN** the URL path starts with `/api/v1/`
- **AND** requests to non-versioned paths (e.g., `/api/changes`) return 404

### Requirement: Backend serves health and readiness endpoints

The backend SHALL expose health and readiness endpoints for operational monitoring.

#### Scenario: Health check endpoint

- **WHEN** a client sends `GET /api/v1/health`
- **THEN** the backend returns `{"status": "ok"}` with status 200

#### Scenario: Readiness endpoint validates project access

- **WHEN** a client sends `GET /api/v1/ready`
- **AND** the `.ito/` directory exists and is readable
- **THEN** the backend returns `{"status": "ready"}` with status 200

#### Scenario: Readiness endpoint reports not ready

- **WHEN** a client sends `GET /api/v1/ready`
- **AND** the `.ito/` directory does not exist or is not readable
- **THEN** the backend returns `{"status": "not_ready", "reason": "..."}` with status 503

### Requirement: Backend API authenticates requests with bearer tokens

The backend SHALL require authentication via bearer tokens for all API endpoints except health/readiness.

#### Scenario: Valid bearer token grants access

- **WHEN** a client sends a request with header `Authorization: Bearer <valid-token>`
- **THEN** the request is processed normally

#### Scenario: Missing or invalid token is rejected

- **WHEN** a client sends a request without an `Authorization` header or with an invalid token
- **AND** the endpoint requires authentication
- **THEN** the backend returns 401 Unauthorized

#### Scenario: Health and readiness endpoints bypass authentication

- **WHEN** a client sends `GET /api/v1/health` or `GET /api/v1/ready`
- **THEN** the request is processed without requiring authentication

### Requirement: Backend API returns structured error responses

The backend SHALL return structured JSON error responses for all error conditions.

#### Scenario: Error response format

- **WHEN** any API endpoint encounters an error
- **THEN** the response body is `{"error": "<message>", "code": "<error_code>"}`
- **AND** the HTTP status code matches the error category (400 for client errors, 404 for not found, 500 for server errors)

### Requirement: Backend starts via CLI subcommand

The backend SHALL be startable via `ito serve-api` CLI subcommand.

#### Scenario: Start backend with default settings

- **WHEN** a user runs `ito serve-api`
- **AND** the current directory contains an `.ito/` directory
- **THEN** the backend starts listening on `127.0.0.1:9010`
- **AND** outputs the listening address and a generated auth token to stderr

#### Scenario: Start backend with custom bind address and port

- **WHEN** a user runs `ito serve-api --bind 0.0.0.0 --port 8080`
- **THEN** the backend starts listening on `0.0.0.0:8080`

#### Scenario: Start backend with explicit auth token

- **WHEN** a user runs `ito serve-api --token my-secret`
- **THEN** the backend uses `my-secret` as the authentication token
- **AND** does not generate a random token

#### Scenario: Fail if no ito directory found

- **WHEN** a user runs `ito serve-api`
- **AND** no `.ito/` directory is found in the current or parent directories
- **THEN** the command exits with an error indicating no Ito project was found
