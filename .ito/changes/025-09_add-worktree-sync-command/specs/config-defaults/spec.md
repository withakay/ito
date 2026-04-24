<!-- ITO:START -->
## ADDED Requirements

### Requirement: Coordination sync interval default

The system SHALL provide a default `changes.coordination_branch.sync_interval_seconds` value of `120` when no explicit coordination sync interval is configured.

- **Requirement ID**: `config-defaults:coordination-sync-interval-default`

#### Scenario: Default sync interval is applied

- **WHEN** Ito loads project configuration for `ito sync`
- **AND** `changes.coordination_branch.sync_interval_seconds` is not set
- **THEN** the effective coordination sync interval is `120` seconds

#### Scenario: Default sync interval is exported in schema output

- **WHEN** Ito generates the JSON schema for configuration
- **THEN** the schema includes the default value `120` for `changes.coordination_branch.sync_interval_seconds`

### Requirement: Archive main integration mode default

The system SHALL provide a default `changes.archive.main_integration_mode` value of `pull_request` when no explicit archive integration mode is configured.

- **Requirement ID**: `config-defaults:archive-main-integration-mode-default`

#### Scenario: Default archive integration mode is applied

- **WHEN** Ito loads project configuration for worktree-mode archive guidance
- **AND** `changes.archive.main_integration_mode` is not set
- **THEN** the effective archive integration mode is `pull_request`

#### Scenario: Default archive integration mode is exported in schema output

- **WHEN** Ito generates the JSON schema for configuration
- **THEN** the schema includes the default value `pull_request` for `changes.archive.main_integration_mode`
<!-- ITO:END -->
