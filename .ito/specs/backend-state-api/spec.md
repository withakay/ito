## ADDED Requirements

### Requirement: REST transport uses safe reads and explicit mutation verbs

When the remote persistence implementation uses HTTP, the backend API SHALL use read-only `GET` endpoints for retrieval and SHALL use explicit non-`GET` verbs for state-changing operations.

#### Scenario: Read endpoint does not mutate state

- **WHEN** a client performs a `GET` request against a backend read endpoint
- **THEN** the backend returns the requested representation
- **AND** the backend does not mutate project state as a side effect of servicing that `GET`

#### Scenario: Mutation endpoint does not use GET

- **WHEN** the backend exposes an operation that creates, updates, archives, claims, releases, or deletes state
- **THEN** that operation is exposed through `POST`, `PUT`, `PATCH`, or `DELETE` as appropriate
- **AND** it is not exposed as a state-changing `GET`

### Requirement: Retryable REST mutations are idempotent

When the remote persistence implementation uses HTTP, mutation endpoints that may be retried by clients MUST be safe to retry through inherent verb semantics or an explicit idempotency mechanism.

#### Scenario: Idempotent mutation retry does not duplicate side effects

- **GIVEN** a client retries the same mutation request due to a transient failure
- **WHEN** the backend receives that retried request
- **THEN** the backend applies the mutation at most once
- **AND** returns a response that allows the client to treat the retry as safe

#### Scenario: Repository client can rely on safe retries

- **GIVEN** a remote-backed repository adapter is configured with bounded retry behavior
- **WHEN** it retries a mutation request after a transient transport failure
- **THEN** the backend contract preserves correctness by providing idempotent mutation semantics
