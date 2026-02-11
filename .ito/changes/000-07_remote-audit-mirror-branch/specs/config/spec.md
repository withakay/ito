## ADDED Requirements

### Requirement: Configure audit remote mirroring

The configuration system SHALL support nested keys for audit remote mirroring behavior.

Supported keys SHALL include:

- `audit.mirror.enabled`
- `audit.mirror.branch`

#### Scenario: Audit mirror is enabled by configuration

- **WHEN** the user sets `audit.mirror.enabled=true`
- **THEN** the system attempts audit mirroring after local audit event writes

#### Scenario: Audit mirror branch can be overridden

- **WHEN** the user sets `audit.mirror.branch` to a valid branch name
- **THEN** the system mirrors audit events to the configured branch

#### Scenario: Invalid audit mirror branch is rejected

- **WHEN** the user sets `audit.mirror.branch` to an invalid branch name
- **THEN** configuration validation fails with a clear error
