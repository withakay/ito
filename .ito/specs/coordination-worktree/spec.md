## ADDED Requirements

### Requirement: Coordination worktree creation

The system SHALL create a dedicated Git worktree for the coordination branch at a central system location when `storage` is set to `"worktree"`.

- **Requirement ID**: coordination-worktree:worktree-creation

#### Scenario: Worktree created at XDG data path

- **WHEN** `ito init` runs with coordination storage mode `"worktree"`
- **AND** no explicit `worktree_path` is configured
- **THEN** the system creates a worktree at `$XDG_DATA_HOME/ito/<org>/<repo>/`
- **AND** the worktree is checked out to the coordination branch

#### Scenario: Worktree fallback when XDG_DATA_HOME is not set

- **WHEN** `ito init` runs with coordination storage mode `"worktree"`
- **AND** `$XDG_DATA_HOME` is not set
- **THEN** the system creates a worktree at `~/.local/share/ito/<org>/<repo>/`

#### Scenario: Worktree created at explicit path

- **WHEN** `worktree_path` is explicitly set in configuration
- **THEN** the system creates the worktree at that path instead of the XDG default

#### Scenario: Coordination branch fetched from remote if needed

- **WHEN** the coordination branch does not exist locally
- **AND** it exists on `origin`
- **THEN** the system fetches it before creating the worktree

#### Scenario: Coordination branch created if missing everywhere

- **WHEN** the coordination branch exists neither locally nor on `origin`
- **THEN** the system creates an orphan branch with that name
- **AND** creates the worktree from the new branch

### Requirement: Symlink wiring from project to worktree

The system SHALL replace `.ito/{changes,specs,modules,workflows,audit}` directories with symlinks pointing to the corresponding directories in the central worktree.

- **Requirement ID**: coordination-worktree:symlink-wiring

#### Scenario: Symlinks created for all mutable directories

- **WHEN** the coordination worktree is set up
- **THEN** `.ito/changes` is a symlink to `<worktree>/.ito/changes`
- **AND** `.ito/specs` is a symlink to `<worktree>/.ito/specs`
- **AND** `.ito/modules` is a symlink to `<worktree>/.ito/modules`
- **AND** `.ito/workflows` is a symlink to `<worktree>/.ito/workflows`
- **AND** `.ito/audit` is a symlink to `<worktree>/.ito/audit`

#### Scenario: Platform-appropriate link type on Windows

- **WHEN** the coordination worktree is set up on Windows
- **THEN** the system uses directory junctions or reparse points instead of symlinks
- **AND** the behavior is otherwise identical to Unix symlinks

#### Scenario: Existing content moved to worktree before symlinking

- **WHEN** `.ito/changes` exists as a real directory with content
- **THEN** the system moves its content to the worktree location
- **AND** replaces the directory with a symlink

#### Scenario: Symlinks added to .gitignore

- **WHEN** symlinks are created
- **THEN** the system adds the symlink paths to `.gitignore`
- **AND** does not duplicate entries if they already exist

### Requirement: Auto-commit on write

The system SHALL automatically commit changes to the coordination branch worktree after every write operation that modifies tracked Ito artifacts.

- **Requirement ID**: coordination-worktree:auto-commit

#### Scenario: Change creation triggers auto-commit

- **WHEN** a new change proposal is created via `ito create change`
- **AND** coordination storage mode is `"worktree"`
- **THEN** the new files are committed to the coordination branch automatically

#### Scenario: Task status update triggers auto-commit

- **WHEN** a task status is updated via `ito tasks complete`
- **AND** coordination storage mode is `"worktree"`
- **THEN** the modified `tasks.md` is committed to the coordination branch

#### Scenario: Auto-commit message is descriptive

- **WHEN** an auto-commit occurs
- **THEN** the commit message includes the operation type and affected artifact (e.g., `"ito: create change 025-08_foo"`)

### Requirement: Worktree teardown

The system SHALL provide a mechanism to remove the coordination worktree and restore local directories.

- **Requirement ID**: coordination-worktree:teardown

#### Scenario: Teardown removes worktree and symlinks

- **WHEN** the user tears down the coordination worktree
- **THEN** the symlinks are replaced with real directories containing the current content
- **AND** the Git worktree is removed
- **AND** the central directory is optionally deleted

### Requirement: Worktree health check

The system SHALL detect when the coordination worktree is missing or in a broken state and provide actionable guidance.

- **Requirement ID**: coordination-worktree:health-check

#### Scenario: Missing worktree detected on command

- **WHEN** a user runs any `ito` command that accesses changes, specs, or modules
- **AND** coordination storage mode is `"worktree"`
- **AND** the worktree directory does not exist
- **THEN** the system prints an error with the path and suggests `ito init` to restore it

#### Scenario: Broken symlink detected

- **WHEN** `.ito/changes` is a symlink but its target does not exist
- **THEN** the system prints an error identifying the broken symlink and its expected target
