## ADDED Requirements

### Requirement: Worktrees Instruction Artifact

The CLI SHALL support a `worktrees` artifact in `ito agent instruction` that outputs config-driven worktree workflow guidance.

#### Scenario: Worktrees artifact prints resolved config and commands

- **WHEN** the user runs `ito agent instruction worktrees`
- **THEN** the output includes a summary of resolved `worktrees.*` configuration
- **AND** the output includes strategy-specific worktree creation guidance
- **AND** the output includes the config file precedence and which files were loaded

#### Scenario: Workflow is an alias for worktrees

- **WHEN** the user runs `ito agent instruction workflow`
- **THEN** the output is equivalent to `ito agent instruction worktrees`

#### Scenario: Worktrees artifact supports JSON output

- **WHEN** the user runs `ito agent instruction worktrees --json`
- **THEN** the command outputs a JSON object with `artifactId` and `instruction`
