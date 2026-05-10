## ADDED Requirements

### Requirement: Backend requires bearer token authentication

The backend MUST require a bearer token for all project-scoped state endpoints.

#### Scenario: Missing token is unauthorized

- **WHEN** a client calls a protected endpoint without an `Authorization: Bearer` token
- **THEN** the backend returns `401 Unauthorized`

#### Scenario: Invalid token is unauthorized

- **WHEN** a client calls a protected endpoint with an invalid token
- **THEN** the backend returns `401 Unauthorized`

### Requirement: Tokens are scoped to a project

The backend MUST validate that the presented token is authorized for the target project scope.

#### Scenario: Token project mismatch is forbidden

- **GIVEN** a token scoped to project `proj_a`
- **WHEN** the client calls an endpoint under project `proj_b`
- **THEN** the backend returns `403 Forbidden`
