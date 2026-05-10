## ADDED Requirements

### Requirement: Schema-aware change validation

When a change declares a schema, `ito validate <change-id>` MUST resolve that schema and apply schema-defined validation rules when they are available.

#### Scenario: Change uses schema validation.yaml

- **GIVEN** `.ito/changes/<change-id>/.ito.yaml` selects schema `<schema-name>`
- **AND** the resolved schema directory contains `validation.yaml`
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation MUST use the schema validation rules from `validation.yaml`
- **AND** validation output MUST report the resolved schema name

### Requirement: Manual validation signal when schema has no validation spec

If a schema does not provide `validation.yaml`, `ito validate <change-id>` MUST NOT assume Ito delta spec semantics and MUST emit an explicit issue indicating the schema requires manual validation.

#### Scenario: Change schema has no validation.yaml

- **GIVEN** `.ito/changes/<change-id>/.ito.yaml` selects schema `<schema-name>`
- **AND** the resolved schema directory does not contain `validation.yaml`
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation MUST emit an informational issue indicating manual validation is required
- **AND** validation MUST NOT fail solely because no Ito delta specs are present

### Requirement: Tracking file validation is schema-driven

When schema validation is configured to validate a tracking file derived from `apply.tracks`, `ito validate <change-id>` MUST validate the tracking file at that path, not a hard-coded filename.

#### Scenario: Tracking file uses apply.tracks path

- **GIVEN** the resolved schema's `apply.tracks` is `todo.md`
- **AND** `validation.yaml` declares tracking validation sourced from `apply.tracks`
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation MUST validate `.ito/changes/<change-id>/todo.md`
- **AND** validation MUST NOT require `.ito/changes/<change-id>/tasks.md`
