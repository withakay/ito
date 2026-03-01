## ADDED Requirements

### Requirement: Change ownership uses expiring leases

The backend SHALL provide change-level ownership leases with expiration timestamps so only one active lease may exist per change.

#### Scenario: Lease acquisition succeeds when change is unlocked

- **GIVEN** change `024-01_add-shared-state-api` has no active lease
- **WHEN** an agent requests lease acquisition
- **THEN** the backend creates an active lease for that change
- **AND** the response includes lease owner identity and lease expiration timestamp

#### Scenario: Lease acquisition fails when another active lease exists

- **GIVEN** change `024-01_add-shared-state-api` has an active lease owned by another agent
- **WHEN** a second agent requests lease acquisition
- **THEN** the backend rejects the request with a conflict response

### Requirement: Lease heartbeat renews ownership

The backend SHALL support lease heartbeat updates that extend lease expiration for the current lease owner.

#### Scenario: Owner heartbeat extends expiration

- **GIVEN** an agent holds an active lease
- **WHEN** the agent sends a heartbeat before expiration
- **THEN** the backend extends the lease expiration time

#### Scenario: Non-owner heartbeat is rejected

- **GIVEN** an active lease owned by agent `A`
- **WHEN** agent `B` sends a heartbeat for that lease
- **THEN** the backend rejects the heartbeat as unauthorized for the lease

### Requirement: Lease release and expiry unlock changes

The backend SHALL unlock a change when the owner releases the lease or when the lease expires without heartbeat renewal.

#### Scenario: Owner release unlocks change

- **GIVEN** an active lease owned by an agent
- **WHEN** the owner requests lease release
- **THEN** the backend marks the lease inactive
- **AND** the change becomes available for new lease acquisition

#### Scenario: Expired lease is treated as unlocked

- **GIVEN** a lease with expiration timestamp in the past
- **WHEN** a new agent requests lease acquisition
- **THEN** the backend treats the previous lease as expired
- **AND** grants a new active lease if no newer lease exists

### Requirement: Archived changes cannot be leased

The backend MUST reject lease acquisition and heartbeat operations for archived changes.

#### Scenario: Lease acquisition is rejected for archived change

- **GIVEN** a change is archived
- **WHEN** an agent requests lease acquisition
- **THEN** the backend rejects the request

#### Scenario: Heartbeat is rejected for archived change

- **GIVEN** a change is archived
- **WHEN** an agent sends a lease heartbeat
- **THEN** the backend rejects the request
