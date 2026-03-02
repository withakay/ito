## ADDED Requirements

### Requirement: Backend publishes a versioned OpenAPI contract

The backend SHALL expose a versioned OpenAPI specification that documents all state-management endpoints and request/response schemas.

#### Scenario: OpenAPI document is available

- **WHEN** a client requests the backend OpenAPI endpoint
- **THEN** the backend returns an OpenAPI document for API version `v1`
- **AND** the document includes schemas for projects, changes, leases, artifacts, and events

### Requirement: Backend state endpoints are project-scoped

All backend endpoints that read or mutate Ito state SHALL be scoped to a project identifier.

#### Scenario: Project-scoped endpoint resolution

- **WHEN** a client requests a change resource for project `proj_abc`
- **THEN** the backend resolves state only within `proj_abc`
- **AND** state from other projects is not returned

#### Scenario: Unknown project is rejected

- **WHEN** a client requests a project-scoped endpoint with an unknown project identifier
- **THEN** the backend returns a not-found error for that project scope

### Requirement: Backend exposes change lifecycle state including archive

The backend SHALL represent change lifecycle state and SHALL allow a change to be marked archived.

#### Scenario: Change indicates archived status

- **GIVEN** a change has been archived
- **WHEN** a client requests the change resource
- **THEN** the backend includes a field indicating the change is archived

#### Scenario: Archive operation marks change as archived

- **GIVEN** a change is not archived
- **WHEN** a client requests to archive that change
- **THEN** the backend marks the change archived
- **AND** subsequent reads show the archived status
