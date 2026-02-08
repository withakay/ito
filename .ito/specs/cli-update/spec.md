## MODIFIED Requirements

### Requirement: Update refreshes harness wrappers without duplicating instruction bodies

`ito update` SHALL refresh the managed blocks of harness prompt/command files so they remain thin wrappers that delegate to `ito agent instruction <artifact>` rather than embedding large duplicated instruction bodies.

`ito update` SHALL include an interactive worktree setup wizard when worktree configuration has not yet been set, guiding users through the same setup flow as `ito init`.

#### Scenario: Refreshing OpenCode wrapper keeps delegation pattern

- **GIVEN** `.opencode/commands/` contains Ito command files
- **WHEN** a user runs `ito update`
- **THEN** each file's managed block SHALL be refreshed to delegate to `ito agent instruction <artifact>`

#### Scenario: Worktree setup prompt on first upgrade
- **GIVEN** the user has not configured `worktrees.strategy` in their config
- **WHEN** the user runs `ito update` in interactive mode
- **THEN** the CLI asks whether to enable worktrees for this project
- **AND** if the user answers yes, the CLI asks which strategy to use, presenting `checkout_subdir` (recommended), `checkout_siblings`, and `bare_control_siblings` as options
- **AND** the CLI asks which integration mode to prefer, presenting `commit_pr` (recommended) and `merge_parent` as options
- **AND** the CLI persists the answers to the project or global config file
- **AND** the CLI prints the config file path and the keys that were written

#### Scenario: Worktree setup prompt skipped when already configured
- **GIVEN** the user has already configured `worktrees.strategy` in their config
- **WHEN** the user runs `ito update`
- **THEN** the worktree setup wizard is not shown
- **AND** existing worktree config is preserved

#### Scenario: Non-interactive update skips worktree prompts
- **WHEN** the user runs `ito update --no-interactive`
- **THEN** the worktree setup wizard is skipped
- **AND** worktree config is not modified

#### Scenario: User declines worktree enablement during update
- **WHEN** the user runs `ito update` in interactive mode
- **AND** the worktree setup wizard is shown
- **AND** the user answers "no" to the worktree enablement question
- **THEN** `worktrees.enabled` is set to `false` in config
- **AND** no further worktree questions are asked
- **AND** the CLI prints the config file path for future reference
