# Coordination Worktree Migration

## Purpose

This spec defines the current behavior and requirements for coordination worktree migration.

## Requirements

### Requirement: Agent instruction for migration

The system SHALL provide an agent instruction (`ito agent instruction migrate-to-coordination-worktree`) that guides an LLM through migrating an existing project from embedded to worktree storage.

- **Requirement ID**: coordination-worktree-migration:agent-instruction

#### Scenario: Instruction covers full migration steps

- **WHEN** `ito agent instruction migrate-to-coordination-worktree` is invoked
- **THEN** the output includes steps for: creating the coordination branch, creating the worktree, moving content, creating symlinks, updating .gitignore, and updating config

#### Scenario: Instruction warns about in-flight changes

- **WHEN** the instruction is generated
- **THEN** it includes a warning to ensure no in-flight change proposals have uncommitted work before migrating

### Requirement: No automatic migration on upgrade

`ito init --upgrade` SHALL NOT automatically migrate existing projects from embedded to worktree storage.

- **Requirement ID**: coordination-worktree-migration:no-auto-migrate

#### Scenario: Upgrade preserves existing storage mode

- **WHEN** `ito init --upgrade` runs on a project with `storage: "embedded"` (or no storage field)
- **THEN** the storage mode is unchanged
- **AND** no worktree is created
- **AND** no symlinks are created

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
