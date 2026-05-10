## MODIFIED Requirements

### Requirement: Audit events use a dedicated internal branch for local durable storage

The system SHALL store local durable audit history on a dedicated internal git branch rather than on user-facing working branches.

#### Scenario: Internal branch defaults are applied

- **WHEN** local durable audit storage is enabled without explicit branch override
- **THEN** the system stores audit events on `ito/internal/audit`

#### Scenario: Internal audit branch is independent of change coordination

- **WHEN** both change coordination and local audit storage are enabled
- **THEN** audit history writes only to the configured internal audit branch
- **AND** change coordination continues to use its own configured branch

### Requirement: Internal audit branch failures are best-effort

Failures writing or syncing the internal audit branch MUST NOT cause core CLI commands to fail.

#### Scenario: Internal branch update fails due to git conflict

- **WHEN** an internal audit branch write encounters a non-fast-forward or similar git conflict
- **THEN** the command still completes with its normal outcome
- **AND** the system emits a warning with remediation guidance

#### Scenario: Internal branch storage is unavailable

- **WHEN** local internal audit storage is unavailable
- **THEN** audit events are routed to the configured fallback local store
- **AND** the command still completes with its normal outcome
