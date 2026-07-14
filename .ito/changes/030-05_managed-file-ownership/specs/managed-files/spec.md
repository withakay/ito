## ADDED Requirements

### Requirement: Managed File Status

Ito SHALL expose managed-file ownership metadata.

#### Scenario: Inspect managed files

- **WHEN** a user runs `ito managed status --json`
- **THEN** Ito SHALL report known managed files, marker-managed files, generated files, user-owned files, and unknown files
- **AND** each managed entry SHALL include the owning template or installer when known.

### Requirement: Managed Diff

Ito SHALL preview managed-file updates without writing files.

#### Scenario: Preview managed changes

- **WHEN** a user runs `ito managed diff --json`
- **THEN** Ito SHALL report files and managed blocks that would change if the update ran
- **AND** Ito SHALL not modify the worktree.

### Requirement: Update Dry Run

Ito update SHALL support a JSON dry run.

#### Scenario: Dry-run update

- **WHEN** a user runs `ito update --dry-run --json`
- **THEN** Ito SHALL report planned creates, updates, skips, conflicts, and overwrites
- **AND** Ito SHALL not modify the worktree.

### Requirement: Durable Guidance Locations

Ito SHALL report where project-specific durable guidance belongs.

#### Scenario: Generated prompt file

- **WHEN** a managed status entry describes a generated prompt or skill file
- **THEN** Ito SHALL include recommended durable override paths or guidance files when available.
