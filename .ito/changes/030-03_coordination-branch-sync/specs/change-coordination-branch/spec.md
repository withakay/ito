## ADDED Requirements

### Requirement: Explicit Coordination Sync

Ito SHALL expose an explicit command for synchronizing internal coordination state.

#### Scenario: Sync remote-ahead coordination branch

- **WHEN** the coordination branch exists remotely and is ahead of the local writer
- **THEN** `ito coordination sync --json` SHALL fetch the remote branch, merge deterministic metadata, and report success when no semantic conflict remains.

#### Scenario: Missing coordination branch

- **WHEN** the coordination branch does not exist remotely
- **THEN** sync SHALL initialize it according to configuration
- **AND** the JSON response SHALL report that initialization occurred.

### Requirement: Automatic Write Recovery

Ito coordination writes SHALL recover from non-fast-forward push failures without requiring LLM intervention when deterministic recovery is safe.

#### Scenario: Create change push rejected

- **WHEN** `ito create change` attempts to push coordination metadata and receives a non-fast-forward rejection
- **THEN** Ito SHALL fetch, merge coordination metadata, and retry the push with a bounded retry count.

### Requirement: Coordination Diagnostics

Ito SHALL provide a machine-readable doctor command for coordination state.

#### Scenario: Diagnose branch state

- **WHEN** a user runs `ito coordination doctor --json`
- **THEN** Ito SHALL report configured branch name, local availability, remote availability, ahead/behind state, last sync attempt, and any retryable or non-retryable conflict.

### Requirement: Coordination Locks

Ito SHALL support lightweight coordination locks for active change metadata writes.

#### Scenario: Lock a change

- **WHEN** a process runs `ito coordination lock <change-id> --json`
- **THEN** Ito SHALL record a lock entry with owner, timestamp, and expiration
- **AND** concurrent writers SHALL receive structured conflict information when the lock is active.
