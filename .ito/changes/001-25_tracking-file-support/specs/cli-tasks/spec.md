## ADDED Requirements

### Requirement: Tasks CLI operates on the schema-selected tracking file

The `ito tasks` CLI MUST resolve the tracking file path from the selected schema and operate on that file.

Tracking file resolution MUST follow this precedence:

1) `schema.yaml` `apply.tracks` if present
2) otherwise default to `tasks.md`

#### Scenario: apply.tracks overrides tasks.md for tasks operations

- **GIVEN** a schema declares `apply.tracks: todo.md`
- **WHEN** executing `ito tasks status <change>`
- **THEN** the command reads task state from `todo.md`
- **AND** it does not read task state from `tasks.md`

#### Scenario: Non-tasks-tracking schema fails with helpful error

- **GIVEN** a schema resolves a tracking file that is not an Ito `tasks-tracking` file
- **WHEN** executing `ito tasks status <change>`
- **THEN** the command exits with an actionable error explaining the schema uses a different tracking format
