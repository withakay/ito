## ADDED Requirements

### Requirement: CLI can pull backend artifact bundles into local change files

Ito SHALL provide a synchronization operation that pulls a change artifact bundle from backend state into local markdown files.

The pull command SHALL be `ito tasks sync pull <change-id>`.

#### Scenario: Pull writes artifact files locally

- **GIVEN** backend mode is enabled and change artifacts exist on the backend
- **WHEN** the user runs `ito tasks sync pull <change-id>`
- **THEN** Ito writes proposal, tasks, design (if present), and spec delta files into the local change directory
- **AND** Ito stores backend revision metadata needed for the next push

### Requirement: Sync operations write local backups outside the repo

When performing backend pull or push operations, Ito SHALL write a timestamped local backup snapshot of the affected change artifacts to a per-user directory outside the repo.

#### Scenario: Pull creates a backup snapshot

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks sync pull <change-id>`
- **THEN** Ito writes a backup snapshot of the pulled artifacts under `backend.backup_dir`

#### Scenario: Push creates a backup snapshot before attempting upload

- **GIVEN** backend mode is enabled
- **WHEN** the user runs `ito tasks sync push <change-id>`
- **THEN** Ito writes a backup snapshot of the local artifacts under `backend.backup_dir` before uploading

### Requirement: CLI can push local artifact updates with revision checks

Ito SHALL push local artifact updates to the backend using optimistic concurrency.

The push command SHALL be `ito tasks sync push <change-id>`.

#### Scenario: Push succeeds with current revisions

- **GIVEN** local artifacts are based on current backend revisions
- **WHEN** the user runs `ito tasks sync push <change-id>`
- **THEN** Ito sends artifact updates to the backend
- **AND** backend revisions are advanced

#### Scenario: Push reports conflict on stale revision

- **GIVEN** local artifacts are based on stale backend revisions
- **WHEN** the user runs `ito tasks sync push <change-id>`
- **THEN** Ito reports a revision conflict
- **AND** the output instructs the user to pull latest artifacts before retrying push
