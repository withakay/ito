<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Sync validation verifies exact coordination wiring

When coordination storage mode is `worktree`, the system SHALL treat `.ito/` wiring as healthy for sync only when each coordination entry resolves to the expected path inside the resolved coordination worktree. When invalid wiring can be repaired safely, sync or worktree initialization SHALL create or repair the expected symlinks before proceeding.

- **Requirement ID**: coordination-worktree:exact-sync-wiring

#### Scenario: Expected target paths are accepted

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` each resolve to the matching directory inside the resolved coordination worktree
- **WHEN** the system validates the coordination setup for sync
- **THEN** the wiring is considered healthy

#### Scenario: Existing symlink to the wrong worktree target is rejected

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/specs` is a symlink
- **BUT** it resolves to a path outside the expected coordination worktree location
- **WHEN** the system validates the coordination setup for sync
- **THEN** the wiring is treated as invalid drift
- **AND** the reported error includes both the actual target and the expected target

#### Scenario: Real directories are treated as duplicate local state

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/modules` exists as a real directory instead of a coordination-worktree link
- **WHEN** the system validates the coordination setup for sync
- **THEN** the wiring is treated as invalid duplicate local state
- **AND** the reported error identifies the real directory path and instructs the user to repair the worktree wiring before syncing

#### Scenario: Missing coordination symlink is created during sync

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/changes` is missing in the current worktree
- **AND** the expected coordination worktree path for `changes` exists
- **WHEN** the system syncs coordination state or initializes a worktree
- **THEN** the system creates `.ito/changes` as a symlink to the expected coordination worktree path
- **AND** continues without requiring the user to run a separate manual repair step

#### Scenario: Empty generated directory is replaced during repair

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/specs` exists as an empty real directory created by template initialization
- **AND** the expected coordination worktree path for `specs` exists
- **WHEN** the system syncs coordination state or initializes a worktree with repair enabled
- **THEN** the system replaces the empty directory with the expected symlink
- **AND** reports the repair action in the command output

#### Scenario: Non-empty duplicate directory is not overwritten

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/modules` exists as a non-empty real directory that is not the expected symlink
- **WHEN** the system syncs coordination state or initializes a worktree
- **THEN** the system does not delete or overwrite the directory automatically
- **AND** the reported error includes the expected symlink target and a safe manual remediation path
<!-- ITO:END -->
