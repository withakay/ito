## ADDED Requirements

### Requirement: Audit storage is routed by persistence mode

Ito SHALL route audit event writes and reads to a storage backend appropriate for the current operating mode instead of always using a tracked JSONL file on the current branch.

#### Scenario: Local mode uses non-branch-polluting storage

- **WHEN** Ito runs without backend mode enabled
- **THEN** audit writes SHALL be directed to an internal Ito branch/repository when available
- **AND** Ito SHALL NOT append tracked audit JSONL events onto the current working branch

#### Scenario: Backend mode uses server-side audit storage

- **WHEN** backend mode is enabled
- **THEN** Ito SHALL write audit events only to backend-managed project storage
- **AND** Ito SHALL NOT require a tracked local audit JSONL file on the working branch

### Requirement: Audit commands read from routed storage

Audit read/validate/reconcile workflows SHALL consume the routed audit storage backend rather than assuming `.ito/.state/audit/events.jsonl` on the current branch.

#### Scenario: Validation works in backend mode without local tracked JSONL

- **GIVEN** backend mode is enabled
- **AND** no tracked `.ito/.state/audit/events.jsonl` exists on the working branch
- **WHEN** the user runs `ito audit validate`
- **THEN** Ito SHALL validate against backend-managed audit storage

#### Scenario: Validation works in local mode with internal audit branch storage

- **GIVEN** backend mode is disabled
- **AND** audit history is stored on an internal Ito branch/repository
- **WHEN** the user runs `ito audit reconcile` or `ito audit validate`
- **THEN** Ito SHALL read the audit stream from that internal storage location

### Requirement: Audit storage falls back without polluting working branches

If the preferred local internal audit repository/branch cannot be used, Ito MUST avoid falling back to a tracked JSONL file on the current branch.

#### Scenario: Internal branch unavailable

- **GIVEN** backend mode is disabled
- **AND** the internal audit branch/repository cannot be opened or written
- **WHEN** Ito records an audit event
- **THEN** Ito SHALL use an untracked local fallback store
- **AND** normal commands SHALL continue to run
- **AND** Ito SHALL surface a warning that durable internal audit storage is unavailable
