## ADDED Requirements

### Requirement: Init sets up coordination worktree for new projects

`ito init` SHALL set up the coordination branch worktree and symlinks by default for new projects when backend mode is not enabled.

- **Requirement ID**: cli-init:coordination-worktree-setup

#### Scenario: New project gets worktree and symlinks

- **WHEN** `ito init` runs on a project with no `.ito/` directory
- **AND** `backend.enabled` is not `true`
- **THEN** the coordination branch is created or fetched
- **AND** a worktree is created at the resolved central path
- **AND** symlinks are created for changes, specs, modules, workflows, and audit
- **AND** `.gitignore` is updated with symlink entries

#### Scenario: Init skip worktree with --no-coordination-worktree

- **WHEN** `ito init --no-coordination-worktree` runs
- **THEN** coordination storage is set to `"embedded"`
- **AND** no worktree or symlinks are created

## MODIFIED Requirements

### Requirement: Init prompts for tmux preference

Interactive `ito init` SHALL ask the user whether they use tmux and write the result to `tools.tmux.enabled` in the project config file, regardless of the answer.

- **Requirement ID**: cli-init:tmux-preference

#### Scenario: User answers yes to tmux prompt

- **WHEN** the user runs `ito init` interactively
- **AND** answers `Yes` to "Do you use tmux?"
- **THEN** Ito writes `tools.tmux.enabled = true` to the project config

#### Scenario: User answers no to tmux prompt

- **WHEN** the user runs `ito init` interactively
- **AND** answers `No` to "Do you use tmux?"
- **THEN** Ito writes `tools.tmux.enabled = false` to the project config

#### Scenario: Tmux prompt text is stable

- **WHEN** `ito init` runs interactively
- **THEN** the tmux preference prompt text is exactly: `Do you use tmux?`
- **AND** provides choices `Yes` and `No`

### Requirement: Non-interactive tmux preference flag

`ito init` SHALL support `--no-tmux` to set `tools.tmux.enabled = false` without an interactive prompt.

- **Requirement ID**: cli-init:no-tmux-flag

#### Scenario: --no-tmux suppresses prompt and writes false

- **WHEN** the user runs `ito init --no-tmux`
- **THEN** Ito skips the tmux preference prompt
- **AND** writes `tools.tmux.enabled = false` to the project config

#### Scenario: Default without --no-tmux is true

- **WHEN** the user runs `ito init` non-interactively without `--no-tmux`
- **THEN** Ito writes `tools.tmux.enabled = true` to the project config
