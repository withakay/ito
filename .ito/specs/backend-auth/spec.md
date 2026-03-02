## MODIFIED Requirements

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

The backend MUST support two token tiers:

- **Admin tokens**: authorize access to any project namespace on the backend instance
- **Derived project tokens**: authorize access to exactly one `{org}/{repo}` namespace

Derived project tokens MUST be computed as:

- `project_key = "{org}/{repo}"`
- `token = HMAC-SHA256(token_seed, project_key)`

#### Scenario: Admin token authorizes any project

- **GIVEN** a request presents a valid admin token
- **WHEN** the client calls an endpoint under any `{org}/{repo}`
- **THEN** the backend authorizes the request

#### Scenario: Project token authorizes only its project

- **GIVEN** a request presents a derived token for `{org}/{repo}`
- **WHEN** the client calls an endpoint under the same `{org}/{repo}`
- **THEN** the backend authorizes the request

#### Scenario: Token project mismatch is forbidden

- **GIVEN** a derived token for project `{org_a}/{repo_a}`
- **WHEN** the client calls an endpoint under `{org_b}/{repo_b}`
- **THEN** the backend returns `403 Forbidden`

#### Scenario: Health and readiness endpoints bypass authentication

- **WHEN** a client sends `GET /api/v1/health` or `GET /api/v1/ready`
- **THEN** the request is processed without requiring authentication
