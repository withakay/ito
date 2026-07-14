## MODIFIED Requirements

### Requirement: Configure worktree workspace defaults

The config command SHALL allow setting and retrieving nested configuration keys related to worktree workspace behavior.

Supported keys SHALL include:

- `worktrees.enabled`
- `worktrees.strategy`
- `worktrees.layout.base_dir`
- `worktrees.layout.dir_name`
- `worktrees.apply.enabled`
- `worktrees.apply.integration_mode`
- `worktrees.apply.copy_from_main`
- `worktrees.apply.setup_commands`
- `worktrees.default_branch`

#### Scenario: Set default branch for worktrees

- **WHEN** the user executes `ito config set worktrees.default_branch <value>`
- **THEN** Ito stores the value in global configuration

#### Scenario: Set local file copy patterns

- **WHEN** the user executes `ito config set worktrees.apply.copy_from_main <json-array>`
- **THEN** Ito stores the list in global configuration
- **AND** the list is used when generating worktree-aware apply instructions

#### Scenario: Set integration mode

- **WHEN** the user executes `ito config set worktrees.apply.integration_mode <value>`
- **THEN** Ito stores the value in global configuration
- **AND** `<value>` MUST be either `commit_pr` or `merge_parent`

#### Scenario: Set setup command list

- **WHEN** the user executes `ito config set worktrees.apply.setup_commands <json-array>`
- **THEN** Ito stores the ordered command list in global configuration

#### Scenario: Set worktree enablement

- **WHEN** the user executes `ito config set worktrees.enabled <boolean>`
- **THEN** Ito stores the boolean in global configuration
- **AND** when set to `false`, worktree-specific behavior is disabled across all commands

#### Scenario: Set apply enablement

- **WHEN** the user executes `ito config set worktrees.apply.enabled <boolean>`
- **THEN** Ito stores the boolean in global configuration

#### Scenario: Set layout strategy

- **WHEN** the user executes `ito config set worktrees.strategy <value>`
- **THEN** Ito stores the value in global configuration
- **AND** `<value>` MUST be one of `bare_control_siblings`, `checkout_subdir`, or `checkout_siblings`

#### Scenario: Reject unsupported strategy values

- **WHEN** the user executes `ito config set worktrees.strategy <value>`
- **AND** `<value>` is not one of the supported strategies
- **THEN** the command fails with a validation error

#### Scenario: Set layout base directory

- **WHEN** the user executes `ito config set worktrees.layout.base_dir <value>`
- **THEN** Ito stores the value in global configuration

#### Scenario: Set worktree directory name

- **WHEN** the user executes `ito config set worktrees.layout.dir_name <value>`
- **THEN** Ito stores the value in global configuration
- **AND** the value is used in place of the default `ito-worktrees` when resolving worktree directory paths
