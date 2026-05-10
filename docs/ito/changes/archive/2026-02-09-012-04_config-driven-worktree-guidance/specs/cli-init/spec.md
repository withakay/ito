## MODIFIED Requirements

### Requirement: Worktree workspace layout (opt-in)

`ito init` SHALL support an opt-in mode that prepares a Git worktree-based workspace layout under the repository root.

`ito init` SHALL include an interactive worktree setup wizard that guides users through configuring worktree behavior. The wizard runs during every `ito init` invocation in interactive mode.

#### Scenario: Initialize in worktree mode
- **WHEN** the user runs `ito init` with worktree mode enabled
- **THEN** Ito prepares a workspace layout that includes a default-branch worktree at `./main`
- **AND** the layout is created without modifying tracked project files beyond normal Ito initialization outputs

#### Scenario: Worktree mode is idempotent
- **GIVEN** the repository already has a `./main` worktree created by Ito
- **WHEN** the user runs `ito init` again with worktree mode enabled
- **THEN** Ito does not create duplicate worktrees
- **AND** Ito reports that the workspace layout is already configured

#### Scenario: Interactive worktree setup during init
- **WHEN** the user runs `ito init` in interactive mode
- **THEN** the CLI asks whether to enable worktrees for this project
- **AND** if the user answers yes, the CLI asks which strategy to use, presenting `checkout_subdir` (recommended), `checkout_siblings`, and `bare_control_siblings` as options
- **AND** the CLI asks which integration mode to prefer, presenting `commit_pr` (recommended) and `merge_parent` as options
- **AND** the CLI persists the answers to the per-developer project config overlay at `<itoDir>/config.local.json` by default
- **AND** the CLI prints the config file path and the keys that were written

#### Scenario: Non-interactive init skips worktree prompts
- **WHEN** the user runs `ito init --no-interactive`
- **THEN** the worktree setup wizard is skipped
- **AND** worktree config uses defaults (disabled)

#### Scenario: User declines worktree enablement
- **WHEN** the user runs `ito init` in interactive mode
- **AND** the user answers "no" to the worktree enablement question
- **THEN** `worktrees.enabled` is set to `false` in config
- **AND** no further worktree questions are asked
- **AND** the CLI prints the config file path for future reference
