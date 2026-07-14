## ADDED Requirements

### Requirement: Validation defines trace-ready changes

When a change opts into requirement traceability, `ito validate <change-id>` MUST require every delta requirement in that change to declare a requirement id before computed coverage is considered available.

#### Scenario: Missing requirement id fails traced validation

- **GIVEN** a change where at least one delta requirement declares a requirement id
- **AND** another delta requirement in the same change declares no requirement id
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation fails with an actionable error identifying the requirement that is missing a requirement id

### Requirement: Validation fails on invalid requirement references

When a change provides traceability metadata, `ito validate <change-id>` MUST fail if the change contains duplicate requirement ids or task references that do not resolve within that change.

#### Scenario: Unknown task requirement reference fails validation

- **GIVEN** a change task declares a requirement reference that no delta requirement declares
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation fails with an actionable error identifying the unresolved reference and task

### Requirement: Validation reports uncovered requirements

When a change is trace-ready, `ito validate <change-id>` MUST report declared requirement ids that are not covered by any non-shelved enhanced task.

#### Scenario: Non-strict validation warns on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id>` without `--strict`
- **THEN** validation reports the uncovered requirement as a warning

#### Scenario: Strict validation errors on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id> --strict`
- **THEN** validation reports the uncovered requirement as an error

### Requirement: Validation reports unavailable computed traceability

When a change declares requirement traceability metadata but its active tracking file does not support enhanced task trace references, `ito validate <change-id>` MUST report that computed requirement coverage is unavailable instead of reporting every declared requirement as uncovered.

#### Scenario: Checkbox tracking reports unavailable coverage

- **GIVEN** a change declares requirement ids
- **AND** its active tracking file uses checkbox task encoding rather than enhanced task blocks
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation reports that computed requirement coverage is unavailable for that change
- **AND** it does not treat every declared requirement as uncovered solely because enhanced task trace references are unavailable
