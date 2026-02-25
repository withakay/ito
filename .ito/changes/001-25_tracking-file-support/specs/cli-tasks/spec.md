## ADDED Requirements

### Requirement: Tasks CLI operates on the resolved tracking file

When a change selects a schema, `ito tasks` commands MUST read and update the tracking file resolved from that schema's `apply.tracks` (or `tasks.md` by default).

#### Scenario: Tasks status reads schema tracking file

- **GIVEN** a change selects schema `<schema-name>`
- **AND** the schema resolves `apply.tracks` to `todo.md`
- **AND** `.ito/changes/<change-id>/todo.md` is a valid `ito.tasks-tracking.v1` file
- **WHEN** executing `ito tasks status <change-id>`
- **THEN** the command reads tasks from `.ito/changes/<change-id>/todo.md`
- **AND** the command does not require `.ito/changes/<change-id>/tasks.md`

### Requirement: Tasks CLI rejects incompatible tracking formats

If a change's schema tracks progress in a format other than `ito.tasks-tracking.v1`, `ito tasks` MUST exit with a helpful error explaining that the schema uses a different tracking format.

#### Scenario: Schema tracking format is not Ito tasks-tracking

- **GIVEN** a change selects schema `<schema-name>`
- **AND** the schema's tracking validator is not `ito.tasks-tracking.v1`
- **WHEN** executing `ito tasks status <change-id>`
- **THEN** the command exits with a non-zero exit code
- **AND** the output explains that `ito tasks` requires `ito.tasks-tracking.v1`
- **AND** the output suggests using schema-specific workflow commands or documentation
