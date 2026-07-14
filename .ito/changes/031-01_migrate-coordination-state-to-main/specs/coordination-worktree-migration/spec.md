<!-- ITO:START -->
## ADDED Requirements

### Requirement: Reverse migration is explicit and agent-driven
Ito SHALL support moving from coordination worktree storage back to tracked main storage through an emitted agent instruction. `ito init`, `ito update`, and normal command execution MUST NOT perform that state migration automatically.
- **Requirement ID**: coordination-worktree-migration:reverse-agent-driven-migration
- **Tags**: behavior, stateful

#### Scenario: Upgrade recommends but does not migrate
- **GIVEN** an existing project uses coordination worktree storage
- **WHEN** the user upgrades Ito-managed assets
- **THEN** Ito may recommend `ito agent instruction migrate-to-main`
- **BUT** it does not replace symlinks, copy artifacts, change storage configuration, or delete the coordination worktree

#### Scenario: Agent instruction is requested explicitly
- **GIVEN** legacy coordination state is detected
- **WHEN** a user runs `ito agent instruction migrate-to-main`
- **THEN** Ito emits contextual migration guidance for the current repository
- **AND** does not itself perform the migration

### Requirement: Reverse migration retains rollback evidence
The reverse migration guidance MUST retain the original coordination worktree, branch, commit identity, file inventory, and content hashes until the tracked-main migration has been reviewed and independently validated.
- **Requirement ID**: coordination-worktree-migration:reverse-migration-rollback-evidence
- **Tags**: behavior, stateful

#### Scenario: Migration preparation completes
- **GIVEN** coordinated artifacts have been copied into real tracked directories
- **WHEN** the migration branch is ready for review
- **THEN** the instruction records the source coordination ref and commit
- **AND** the original external coordination state remains unchanged and accessible
<!-- ITO:END -->
