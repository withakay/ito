## ADDED Requirements

### Requirement: Validation fails on invalid requirement references

When a change provides traceability metadata, `ito validate <change-id>` MUST fail if the change contains duplicate requirement ids or task references that do not resolve within that change.

#### Scenario: Unknown task requirement reference fails validation

- **GIVEN** a change task declares a requirement reference that no delta requirement declares
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation fails with an actionable error identifying the unresolved reference and task

### Requirement: Validation reports uncovered requirements

When a change provides traceability metadata, `ito validate <change-id>` MUST report declared requirement ids that are not covered by any non-shelved enhanced task.

#### Scenario: Non-strict validation warns on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id>` without `--strict`
- **THEN** validation reports the uncovered requirement as a warning

#### Scenario: Strict validation errors on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id> --strict`
- **THEN** validation reports the uncovered requirement as an error
