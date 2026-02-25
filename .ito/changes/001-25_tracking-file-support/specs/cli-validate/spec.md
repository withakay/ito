## ADDED Requirements

### Requirement: Tracking file validation is schema-driven

When schema validation is configured to validate a tracking file derived from `apply.tracks`, `ito validate <change-id>` MUST validate the tracking file at that path, not a hard-coded filename.

#### Scenario: Tracking file uses apply.tracks path

- **GIVEN** the resolved schema's `apply.tracks` is `todo.md`
- **AND** `validation.yaml` declares tracking validation sourced from `apply.tracks`
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation MUST validate `.ito/changes/<change-id>/todo.md`
- **AND** validation MUST NOT require `.ito/changes/<change-id>/tasks.md`

### Requirement: Empty tasks-tracking file produces a warning

If the resolved tracking file is validated as `ito.tasks-tracking.v1` but contains zero recognizable tasks, validation MUST produce a warning issue by default.

In strict mode, validation MUST treat this condition as an error.

#### Scenario: Empty tasks-tracking file warns

- **GIVEN** the resolved tracking file is `.ito/changes/<change-id>/todo.md`
- **AND** tracking validation uses `ito.tasks-tracking.v1`
- **AND** the tracking file contains zero recognizable tasks
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation produces a warning issue indicating no tasks were found

#### Scenario: Empty tasks-tracking file fails in strict mode

- **GIVEN** the resolved tracking file contains zero recognizable tasks
- **WHEN** executing `ito validate <change-id> --strict`
- **THEN** validation fails with an error indicating no tasks were found
