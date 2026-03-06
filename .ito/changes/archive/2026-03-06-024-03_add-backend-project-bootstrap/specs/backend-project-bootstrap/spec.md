## ADDED Requirements

### Requirement: Backend exposes a health and version endpoint

The backend SHALL expose a non-authenticated health endpoint suitable for client connectivity checks.

#### Scenario: Health endpoint responds

- **WHEN** a client requests the backend health endpoint
- **THEN** the backend returns a success response
- **AND** the response includes the API version identifier

### Requirement: Backend exposes token introspection for bootstrap

The backend SHALL provide an authenticated introspection endpoint that returns the project identity bound to the presented token.

#### Scenario: Valid token returns project identity

- **GIVEN** a client presents a valid bearer token
- **WHEN** the client calls the token introspection endpoint
- **THEN** the backend returns the authoritative `project_id` for that token

#### Scenario: Invalid token is rejected

- **GIVEN** a client presents an invalid bearer token
- **WHEN** the client calls the token introspection endpoint
- **THEN** the backend returns `401 Unauthorized`
