<!-- ITO:START -->
## ADDED Requirements

### Requirement: Repair coordination links in existing worktree

When `ito init --update` runs inside an existing Git worktree and coordination storage mode is `worktree`, the command SHALL repair missing or stale `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` coordination links for the current worktree.

- **Requirement ID**: `cli-init:repair-coordination-links-in-existing-worktree`

#### Scenario: Existing change worktree is repaired

- **GIVEN** the current directory is an existing Git worktree for the project
- **AND** coordination storage mode is `worktree`
- **AND** one or more expected `.ito/*` coordination links are missing or stale
- **WHEN** `ito init --update` runs
- **THEN** the command repairs those links for the current worktree
- **AND** shared module, change, spec, workflow, and audit state become visible afterward

#### Scenario: Embedded storage mode skips repair

- **GIVEN** coordination storage mode is `embedded`
- **WHEN** `ito init --update` runs in a worktree
- **THEN** no coordination-link repair is attempted
<!-- ITO:END -->
