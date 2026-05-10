## ADDED Requirements

### Requirement: Audit events can be mirrored to a dedicated remote branch

The system SHALL support optional mirroring of locally persisted audit events to a dedicated internal git branch.

#### Scenario: Mirror branch defaults are applied

- **WHEN** audit mirroring is enabled without explicit branch override
- **THEN** the system mirrors audit events to `ito/internal/audit`

#### Scenario: Mirror uses dedicated branch independent of change coordination

- **WHEN** both change coordination and audit mirroring are enabled
- **THEN** audit mirroring writes only to the configured audit mirror branch
- **AND** change coordination continues to use its own configured branch

### Requirement: Audit mirroring is best-effort

Audit mirror failures MUST NOT cause core CLI commands to fail.

#### Scenario: Mirror push fails due to remote conflict

- **WHEN** a mirror push encounters non-fast-forward conflict
- **THEN** the command still completes with its normal outcome
- **AND** the system emits a warning with remediation guidance

#### Scenario: Mirror push fails while offline

- **WHEN** remote connectivity is unavailable
- **THEN** audit events remain locally persisted
- **AND** the command still completes with its normal outcome
