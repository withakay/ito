<!-- ITO:START -->
## ADDED Requirements

### Requirement: Backend project stores persist sub-module metadata as module state

Backend-managed project stores SHALL persist sub-module metadata as part of module state so remote-backed `ModuleRepository` implementations can list and resolve sub-modules without local markdown.

#### Scenario: Filesystem-backed project store round-trips sub-module metadata

- **GIVEN** backend-managed project state stores module `024` with sub-module `024.01`
- **WHEN** the filesystem-backed project store serves module reads through `ModuleRepository`
- **THEN** the returned module includes sub-module metadata sufficient for `ito list --modules` and `ito show sub-module`

#### Scenario: SQLite-backed project store round-trips sub-module metadata

- **GIVEN** equivalent backend-managed project state exists in SQLite storage
- **WHEN** the SQLite-backed project store serves module reads through `ModuleRepository`
- **THEN** the returned module includes the same sub-module metadata as the filesystem-backed project store
<!-- ITO:END -->
