## ADDED Requirements

### Requirement: Schema-selected tracking file is validated

When validating a change, the system MUST resolve the change's tracking file path from the selected schema and validate that file.

Tracking file resolution MUST follow this precedence:

1) `schema.yaml` `apply.tracks` if present
2) otherwise default to `tasks.md`

#### Scenario: apply.tracks overrides tasks.md for validation

- **GIVEN** a schema declares `apply.tracks: todo.md`
- **WHEN** executing `ito validate <change>`
- **THEN** the tracking-file validation reads and validates `todo.md`
- **AND** the tracking-file validation does not validate `tasks.md`

#### Scenario: Missing apply.tracks falls back to tasks.md

- **GIVEN** a schema does not declare `apply.tracks`
- **WHEN** executing `ito validate <change>`
- **THEN** the tracking-file validation reads and validates `tasks.md`

### Requirement: Tracking file path configuration is safe

The system MUST reject any `apply.tracks` value that attempts path traversal or includes a path separator.

#### Scenario: Path traversal is rejected

- **GIVEN** a schema declares `apply.tracks: ../tasks.md`
- **WHEN** executing `ito validate <change>`
- **THEN** validation fails with an actionable error

#### Scenario: Path separators are rejected

- **GIVEN** a schema declares `apply.tracks: dir/tasks.md`
- **WHEN** executing `ito validate <change>`
- **THEN** validation fails with an actionable error
