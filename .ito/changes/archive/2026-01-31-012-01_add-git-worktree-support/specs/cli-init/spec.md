## ADDED Requirements

### Requirement: Worktree workspace layout (opt-in)

`ito init` SHALL support an opt-in mode that prepares a Git worktree-based workspace layout under the repository root.

#### Scenario: Initialize in worktree mode
- **WHEN** the user runs `ito init` with worktree mode enabled
- **THEN** Ito prepares a workspace layout that includes a default-branch worktree at `./main`
- **AND** the layout is created without modifying tracked project files beyond normal Ito initialization outputs

#### Scenario: Worktree mode is idempotent
- **GIVEN** the repository already has a `./main` worktree created by Ito
- **WHEN** the user runs `ito init` again with worktree mode enabled
- **THEN** Ito does not create duplicate worktrees
- **AND** Ito reports that the workspace layout is already configured
