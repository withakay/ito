<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ensure wires coordination links

When `worktrees.enabled` is `true` and coordination storage mode is `worktree`, `ito worktree ensure --change <id>` SHALL leave the resulting worktree fully Ito-ready by wiring `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` to the resolved coordination worktree before reporting success.

- **Requirement ID**: `worktree-lifecycle:ensure-wires-coordination-links`

#### Scenario: New worktree is Ito-ready immediately

- **GIVEN** worktrees are enabled and coordination storage mode is `worktree`
- **WHEN** `ito worktree ensure --change <id>` creates a new worktree
- **THEN** the returned worktree contains the expected `.ito/*` coordination symlinks
- **AND** the command does not require a follow-up `ito init --update` before shared change, spec, module, workflow, and audit state are visible

#### Scenario: Existing partially initialized worktree is repaired

- **GIVEN** worktrees are enabled and coordination storage mode is `worktree`
- **AND** the expected worktree directory already exists with a valid Git worktree but missing or stale `.ito/*` coordination links
- **WHEN** `ito worktree ensure --change <id>` runs
- **THEN** the command repairs the missing or stale coordination links before reporting success
- **AND** the repair is idempotent when the links are already correct
<!-- ITO:END -->
