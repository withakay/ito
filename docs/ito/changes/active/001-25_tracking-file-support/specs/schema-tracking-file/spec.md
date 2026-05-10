## ADDED Requirements

### Requirement: Resolve tracking file path from schema apply.tracks

When a change selects a schema, Ito MUST resolve the change's tracking file path from the schema's `apply.tracks` value.

If the schema does not declare `apply.tracks`, Ito MUST default the tracking file name to `tasks.md`.

#### Scenario: Schema provides apply.tracks

- **GIVEN** a change selects schema `<schema-name>`
- **AND** the resolved schema declares `apply.tracks: todo.md`
- **WHEN** Ito resolves the change's tracking file path
- **THEN** the resolved tracking file is `.ito/changes/<change-id>/todo.md`

#### Scenario: Schema omits apply.tracks

- **GIVEN** a change selects schema `<schema-name>`
- **AND** the resolved schema does not declare `apply.tracks`
- **WHEN** Ito resolves the change's tracking file path
- **THEN** the resolved tracking file is `.ito/changes/<change-id>/tasks.md`

### Requirement: Reject unsafe tracking file paths

Ito MUST reject `apply.tracks` values that are not safe filenames relative to the change directory.

At minimum, Ito MUST reject values containing path traversal (`..`) or path separators.

#### Scenario: Reject path traversal

- **GIVEN** a schema declares `apply.tracks: ../tasks.md`
- **WHEN** Ito resolves the change's tracking file path
- **THEN** resolution fails with an error indicating the tracking file path is invalid

#### Scenario: Reject path separators

- **GIVEN** a schema declares `apply.tracks: progress/todo.md`
- **WHEN** Ito resolves the change's tracking file path
- **THEN** resolution fails with an error indicating the tracking file path is invalid
