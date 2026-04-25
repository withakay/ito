<!-- ITO:START -->
## ADDED Requirements

### Requirement: Validate current change worktree

The CLI SHALL provide a fast validation command that determines whether the current working directory is an acceptable worktree for a supplied change ID when worktrees are enabled.

- **Requirement ID**: cli-config:validate-current-change-worktree

#### Scenario: Validation passes in matching change worktree

- **GIVEN** `worktrees.enabled=true`
- **AND** the current working directory is inside a git worktree whose path or branch contains `012-07_guard-opencode-worktree-path`
- **WHEN** the user runs the validation command for `012-07_guard-opencode-worktree-path`
- **THEN** the command succeeds
- **AND** it emits machine-readable details suitable for hook callers

#### Scenario: Validation fails on main or control checkout

- **GIVEN** `worktrees.enabled=true`
- **AND** the current working directory is the main/control checkout or the configured default worktree path
- **WHEN** the user runs the validation command for `012-07_guard-opencode-worktree-path`
- **THEN** the command fails quickly
- **AND** the output explains that change work must run from a dedicated change worktree
- **AND** the output includes the expected worktree path when it can be resolved

#### Scenario: Validation warns on missing change ID in branch or path

- **GIVEN** `worktrees.enabled=true`
- **AND** the current working directory is not main/control
- **AND** neither the branch name nor the worktree path contains `012-07_guard-opencode-worktree-path`
- **WHEN** the user runs the validation command for `012-07_guard-opencode-worktree-path`
- **THEN** the command reports a mismatch
- **AND** the message explains that the branch or path should include the full change ID

#### Scenario: Validation is disabled when worktrees are disabled

- **GIVEN** `worktrees.enabled=false`
- **WHEN** the user runs the validation command for any change ID
- **THEN** the command succeeds without enforcing worktree path or branch checks
- **AND** the output states that worktree validation is disabled by configuration

#### Scenario: Same-change suffix worktree is accepted

- **GIVEN** `worktrees.enabled=true`
- **AND** the current working directory is inside a worktree named `012-07_guard-opencode-worktree-path-review`
- **WHEN** the user runs the validation command for `012-07_guard-opencode-worktree-path`
- **THEN** the command succeeds because the worktree name starts with the full change ID
<!-- ITO:END -->
