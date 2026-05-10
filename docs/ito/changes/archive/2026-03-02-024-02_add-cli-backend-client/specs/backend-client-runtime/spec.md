## ADDED Requirements

### Requirement: Backend client runtime is configuration-gated

Ito SHALL initialize a backend API client only when backend mode is enabled in resolved configuration.

#### Scenario: Backend mode enabled initializes client

- **GIVEN** `backend.enabled=true` and required backend settings are present
- **WHEN** Ito starts a backend-aware command
- **THEN** Ito initializes a backend client using configured base URL and project scope

#### Scenario: Backend mode disabled skips client

- **GIVEN** `backend.enabled=false`
- **WHEN** Ito starts a command
- **THEN** Ito does not initialize a backend client
- **AND** command behavior continues through filesystem pathways

### Requirement: Backend requests use bounded retries

Backend client requests MUST use bounded timeout and retry behavior for transient failures.

#### Scenario: Transient failure retries with same idempotency key

- **WHEN** a retriable network error occurs during an idempotent backend operation
- **THEN** Ito retries the request up to configured limits
- **AND** retries reuse the same idempotency key

#### Scenario: Non-retriable error fails fast

- **WHEN** a non-retriable backend error response is returned
- **THEN** Ito surfaces the error without additional retries
