<!-- ITO:START -->
## ADDED Requirements

### Requirement: Sync validation verifies exact coordination wiring

When coordination storage mode is `worktree`, the system SHALL treat `.ito/` wiring as healthy for sync only when each coordination entry resolves to the expected path inside the resolved coordination worktree.

- **Requirement ID**: `coordination-worktree:exact-sync-wiring`

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
<!-- ITO:END -->
