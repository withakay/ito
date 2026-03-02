## ADDED Requirements

### Requirement: Backend allocates the next available unlocked change

The backend SHALL provide an atomic allocation operation that selects and leases one eligible unlocked change for a requesting agent.

#### Scenario: Allocation returns a leased change

- **GIVEN** at least one eligible change exists without an active lease
- **WHEN** an agent requests "next available change"
- **THEN** the backend returns one change
- **AND** the returned change is leased to the requesting agent in the same operation

#### Scenario: Allocation returns no work when all changes are leased or unavailable

- **GIVEN** no eligible unlocked changes exist
- **WHEN** an agent requests "next available change"
- **THEN** the backend returns a no-work result without creating a lease

### Requirement: Allocation excludes archived changes

The backend MUST NOT allocate archived changes.

#### Scenario: Archived changes are not allocated

- **GIVEN** a change is archived
- **WHEN** an agent requests "next available change"
- **THEN** the archived change is not returned as an allocation result

### Requirement: Allocation supports idempotent retries

The backend MUST support idempotent allocation requests so client retries do not allocate multiple changes for the same attempt.

#### Scenario: Retry with same idempotency key returns same allocation

- **GIVEN** an allocation request succeeds with idempotency key `k1`
- **WHEN** the client retries allocation with the same key `k1`
- **THEN** the backend returns the original allocation result
- **AND** no additional change lease is created
