## ADDED Requirements

### Requirement: CLI can explicitly claim and release changes in backend mode

Ito SHALL provide backend-aware CLI operations to claim a change lease and release it when work is finished.

The command names SHALL be:

- `ito tasks claim <change-id>`
- `ito tasks release <change-id>`

#### Scenario: Claim acquires lease for unlocked change

- **GIVEN** backend mode is enabled
- **AND** target change has no active lease
- **WHEN** the user runs `ito tasks claim <change-id>` for that change
- **THEN** Ito requests lease acquisition from the backend
- **AND** reports the change as claimed by the current client identity

#### Scenario: Claim fails when lease already exists

- **GIVEN** backend mode is enabled
- **AND** target change has an active lease owned by another client
- **WHEN** the user runs `ito tasks claim <change-id>`
- **THEN** Ito reports a conflict and does not override the existing lease

#### Scenario: Release unlocks claimed change

- **GIVEN** backend mode is enabled and current client holds the lease
- **WHEN** the user runs `ito tasks release <change-id>`
- **THEN** Ito requests lease release from the backend
- **AND** reports the change as available for allocation

### Requirement: CLI can allocate next available change from backend

Ito SHALL provide a backend-aware allocation operation that returns the next available unlocked change and claims it atomically.

The allocation command SHALL be `ito tasks allocate`.

#### Scenario: Allocation returns one claimed change

- **GIVEN** backend mode is enabled and at least one eligible change is unlocked
- **WHEN** the user runs `ito tasks allocate`
- **THEN** Ito receives a single allocated change from the backend
- **AND** the returned change is already leased to the requester

#### Scenario: Allocation reports no work available

- **GIVEN** backend mode is enabled and no eligible unlocked changes exist
- **WHEN** the user runs `ito tasks allocate`
- **THEN** Ito reports that no allocatable work is currently available
