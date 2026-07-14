## MODIFIED Requirements

### Requirement: Backend ingests audit event batches

The backend SHALL expose an authenticated endpoint to ingest batches of audit events.

The endpoint MUST be project-scoped:

`POST /api/v1/projects/{org}/{repo}/events`

#### Scenario: Ingest events appends to project audit log

- **GIVEN** project `{org}/{repo}` is allowed
- **WHEN** a client sends `POST /api/v1/projects/{org}/{repo}/events` with an event batch
- **THEN** the backend appends the events to the audit log for `{org}/{repo}`
- **AND** the backend returns the number of accepted events and duplicates

#### Scenario: Idempotency key prevents duplicate appends

- **GIVEN** a client sends a batch with idempotency key `k1`
- **WHEN** the client retries the same batch with the same idempotency key `k1`
- **THEN** the backend returns duplicates for already ingested events
- **AND** does not append the events a second time
