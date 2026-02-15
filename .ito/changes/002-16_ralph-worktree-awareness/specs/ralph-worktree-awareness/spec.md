## ADDED Requirements

### Requirement: Resolve worktree for targeted change

When Ralph targets a specific change (via `--change`), the runner SHALL attempt to resolve an existing git worktree whose branch name matches the change ID. If a matching worktree is found, Ralph SHALL use that worktree's root as the effective working directory for the harness, git operations, and validation commands.

#### Scenario: Matching worktree exists

- **WHEN** `ito ralph --change 002-16_ralph-worktree-awareness` is invoked
- **AND** `git worktree list --porcelain` shows a worktree on branch `002-16_ralph-worktree-awareness`
- **THEN** Ralph SHALL resolve the effective working directory to that worktree's path
- **AND** the harness SHALL execute in the worktree directory
- **AND** `git add -A` and `git commit` SHALL execute in the worktree directory
- **AND** validation commands SHALL execute in the worktree directory

#### Scenario: No matching worktree exists

- **WHEN** `ito ralph --change 005-01_some-change` is invoked
- **AND** no worktree exists on branch `005-01_some-change`
- **THEN** Ralph SHALL fall back to the process's current working directory
- **AND** behaviour SHALL be identical to the pre-change baseline

#### Scenario: Worktrees not enabled in config

- **WHEN** worktree support is not enabled in the project configuration
- **THEN** Ralph SHALL skip worktree resolution entirely
- **AND** behaviour SHALL be identical to the pre-change baseline

#### Scenario: No change targeted (unscoped run)

- **WHEN** `ito ralph --file prompt.md` is invoked without `--change`
- **THEN** Ralph SHALL skip worktree resolution
- **AND** the effective working directory SHALL be the process's current working directory

### Requirement: Worktree detection uses git porcelain output

Ralph SHALL detect worktrees by parsing the output of `git worktree list --porcelain`. The branch field from the porcelain output SHALL be compared against the change ID to find a match.

#### Scenario: Branch name matches change ID

- **WHEN** the porcelain output contains a worktree with `branch refs/heads/002-16_ralph-worktree-awareness`
- **THEN** Ralph SHALL treat this as a matching worktree for change `002-16_ralph-worktree-awareness`

#### Scenario: Bare repo worktree is excluded

- **WHEN** the porcelain output contains a bare worktree entry
- **THEN** Ralph SHALL NOT consider it as a candidate match

### Requirement: Verbose logging of resolved working directory

When `--verbose` is enabled, Ralph SHALL log the resolved effective working directory so users can confirm Ralph is operating in the correct location.

#### Scenario: Worktree resolved with verbose

- **WHEN** `--verbose` is enabled
- **AND** a matching worktree is found
- **THEN** Ralph SHALL print a message indicating the resolved worktree path

#### Scenario: Fallback with verbose

- **WHEN** `--verbose` is enabled
- **AND** no matching worktree is found
- **THEN** Ralph SHALL print a message indicating it is using the current working directory

### Requirement: Ralph does not create worktrees

Ralph SHALL NOT create git worktrees. Worktree creation remains the responsibility of the user, instruction templates, or other Ito commands. Ralph only detects and uses existing worktrees.

#### Scenario: Missing worktree is not created

- **WHEN** a change has no existing worktree
- **THEN** Ralph SHALL NOT run `git worktree add`
- **AND** Ralph SHALL fall back to the current working directory
