<!-- ITO:START -->
## ADDED Requirements

### Requirement: Configure coordination sync interval

The config command SHALL allow setting and retrieving `changes.coordination_branch.sync_interval_seconds` as the coordination-worktree sync interval in seconds.

- **Requirement ID**: `cli-config:coordination-sync-interval`

#### Scenario: Set the coordination sync interval

- **WHEN** the user executes `ito config set changes.coordination_branch.sync_interval_seconds 300`
- **THEN** Ito stores `300` as the coordination sync interval

#### Scenario: Reject invalid sync interval values

- **WHEN** the user executes `ito config set changes.coordination_branch.sync_interval_seconds 0`
- **THEN** the command fails with a validation error because the sync interval must be a positive integer number of seconds

### Requirement: Configure archive main integration mode

The config command SHALL allow setting and retrieving `changes.archive.main_integration_mode` as the default policy for integrating archived changes into `main`.

- **Requirement ID**: `cli-config:archive-main-integration-mode`

#### Scenario: Set archive integration mode to direct merge

- **WHEN** the user executes `ito config set changes.archive.main_integration_mode direct_merge`
- **THEN** Ito stores `direct_merge` as the archive integration mode

#### Scenario: Set archive integration mode to pull request auto merge

- **WHEN** the user executes `ito config set changes.archive.main_integration_mode pull_request_auto_merge`
- **THEN** Ito stores `pull_request_auto_merge` as the archive integration mode

#### Scenario: Reject unsupported archive integration modes

- **WHEN** the user executes `ito config set changes.archive.main_integration_mode <value>`
- **AND** `<value>` is not one of `direct_merge`, `pull_request`, `pull_request_auto_merge`, or `coordination_only`
- **THEN** the command fails with a validation error
<!-- ITO:END -->
