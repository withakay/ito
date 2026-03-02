## ADDED Requirements

### Requirement: Clients forward local events to backend in backend mode

When backend mode is enabled, Ito clients SHALL be able to forward locally produced events to the backend event ingest endpoint.

#### Scenario: Forwarder sends a batch successfully

- **GIVEN** local events exist that have not yet been forwarded
- **WHEN** the forwarder runs
- **THEN** Ito submits an event batch to the backend ingest endpoint
- **AND** records that those events were forwarded

#### Scenario: Forwarder retries transient failures idempotently

- **GIVEN** a transient network failure occurs while submitting an event batch
- **WHEN** Ito retries the submission
- **THEN** the request uses the same idempotency key
- **AND** the backend does not store duplicate events

#### Scenario: Invalid event payload is not forwarded

- **GIVEN** a local event batch fails payload validation
- **WHEN** the forwarder attempts submission
- **THEN** Ito reports the validation failure
- **AND** does not mark the batch as forwarded
