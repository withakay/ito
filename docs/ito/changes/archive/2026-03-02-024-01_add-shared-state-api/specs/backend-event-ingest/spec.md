## ADDED Requirements

### Requirement: Backend accepts project-scoped event ingestion

The backend SHALL provide an endpoint for authenticated clients to append structured state/audit events for a project.

#### Scenario: Valid event batch is accepted

- **WHEN** a client submits a valid event batch for an authorized project
- **THEN** the backend appends those events to project event storage
- **AND** returns an accepted response

### Requirement: Event ingestion is idempotent

The backend MUST support idempotency keys for event ingestion requests to prevent duplicate event records on retry.

#### Scenario: Retry with same idempotency key does not duplicate events

- **GIVEN** an event ingestion request succeeds with idempotency key `evt-key-1`
- **WHEN** the client retries the same request with idempotency key `evt-key-1`
- **THEN** the backend returns success
- **AND** does not append duplicate events

### Requirement: Event payloads are validated

The backend MUST validate required event fields before accepting ingestion.

#### Scenario: Invalid event payload is rejected

- **WHEN** a client submits an event missing required fields
- **THEN** the backend rejects the request with validation errors
- **AND** no events from that invalid payload are appended
