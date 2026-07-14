<!-- ITO:START -->
## ADDED Requirements

### Requirement: Repair current worktree links

In coordination storage mode `worktree`, the system SHALL provide a supported repair path that rewires the current worktree's `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` entries to the resolved coordination worktree when they are missing, stale, or real directories.

- **Requirement ID**: `coordination-worktree:repair-current-worktree-links`

#### Scenario: Missing links are created

- **GIVEN** coordination storage mode is `worktree`
- **AND** the current worktree is missing one or more expected `.ito/*` coordination entries
- **WHEN** the repair path runs
- **THEN** the missing entries are created as links to the resolved coordination worktree targets

#### Scenario: Real directories are migrated and replaced

- **GIVEN** coordination storage mode is `worktree`
- **AND** `.ito/modules` or another coordination path exists as a real directory in the current worktree
- **WHEN** the repair path runs
- **THEN** any content is migrated to the resolved coordination worktree target as needed
- **AND** the real directory is replaced by the correct coordination link

#### Scenario: Healthy links are left unchanged

- **GIVEN** coordination storage mode is `worktree`
- **AND** all expected `.ito/*` coordination entries already resolve to the correct targets
- **WHEN** the repair path runs
- **THEN** it completes without modifying the healthy links
<!-- ITO:END -->
