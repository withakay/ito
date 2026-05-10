## ADDED Requirements

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
