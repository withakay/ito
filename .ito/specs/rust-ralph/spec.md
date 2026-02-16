## MODIFIED Requirements

### Requirement: State is written under `.ito/.state/ralph/<change>`

Rust MUST write loop state and history in the same location and structure as TypeScript. When Ralph resolves a worktree for the targeted change, state files SHALL be written relative to the worktree's `.ito` directory, not the invoking process's `.ito` directory.

#### Scenario: State files exist

- **GIVEN** a completed loop run
- **WHEN** the user inspects `.ito/.state/ralph/<change-id>/`
- **THEN** the expected state and history files exist

#### Scenario: State written in worktree when resolved

- **GIVEN** Ralph resolves a worktree at `/project/ito-worktrees/002-16_foo/`
- **WHEN** a loop iteration completes
- **THEN** state files SHALL be written under `/project/ito-worktrees/002-16_foo/.ito/.state/ralph/002-16_foo/`
