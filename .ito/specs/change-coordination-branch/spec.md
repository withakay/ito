## MODIFIED Requirements

### Requirement: Coordination branch can be provisioned before first change creation

The system SHALL provide a workflow to provision the coordination branch on `origin` before change-creation operations attempt reservation pushes. When coordination storage mode is `"worktree"`, provisioning SHALL also create a dedicated worktree at a central system location and wire symlinks from the project's `.ito/` directory.

- **Requirement ID**: change-coordination-branch:provisioning

#### Scenario: Init provisioning honors configured branch name

- **GIVEN** `changes.coordination_branch.name` is set to a custom branch
- **WHEN** the user runs `ito init --setup-coordination-branch`
- **THEN** provisioning targets the configured branch name
- **AND** no hardcoded fallback branch name is used for remote setup

#### Scenario: Worktree provisioning when storage is worktree

- **GIVEN** `changes.coordination_branch.storage` is `"worktree"`
- **WHEN** the user runs `ito init`
- **THEN** a dedicated worktree is created at the resolved central path
- **AND** `.ito/{changes,specs,modules,workflows,audit}` are replaced with symlinks to the worktree

#### Scenario: Embedded provisioning unchanged

- **GIVEN** `changes.coordination_branch.storage` is `"embedded"`
- **WHEN** the user runs `ito init`
- **THEN** the system behaves as it does today with no worktree or symlinks created

## ADDED Requirements

### Requirement: Coordination branch storage mode selection

The system SHALL support two storage modes for coordination branch content: `"worktree"` (dedicated worktree at central location) and `"embedded"` (current behavior, artifacts in-tree). The default for new projects (`ito init`) SHALL be `"worktree"`.

- **Requirement ID**: change-coordination-branch:storage-mode-selection

#### Scenario: New project defaults to worktree storage

- **WHEN** `ito init` runs on a project with no existing coordination branch config
- **THEN** `changes.coordination_branch.storage` is set to `"worktree"`

#### Scenario: Backend mode overrides storage mode

- **WHEN** `backend.enabled` is `true`
- **THEN** the coordination worktree is not created
- **AND** backend persistence takes precedence
