<!-- ITO:START -->
## ADDED Requirements

### Requirement: Backend project stores persist sub-module metadata as module state

Backend-managed project stores SHALL persist sub-module metadata as part of module state so remote-backed `ModuleRepository` implementations can list and resolve sub-modules without local markdown.

#### Scenario: Filesystem-backed project store round-trips sub-module metadata

- **GIVEN** backend-managed project state stores module `024` with sub-module `024.01`
- **WHEN** the filesystem-backed project store serves module reads through `ModuleRepository`
- **THEN** the returned module includes sub-module metadata sufficient for `ito list --modules` and `ito show sub-module`

#### Scenario: SQLite-backed project store returns modules with empty sub-module list (PoC)

- **GIVEN** equivalent backend-managed project state exists in SQLite storage
- **WHEN** the SQLite-backed project store serves module reads through `ModuleRepository`
- **THEN** the returned module list is populated but `sub_modules` is empty (cross-referencing sub-module data is not yet implemented in the SQLite PoC store)

> **Note:** Full sub-module metadata round-tripping in the SQLite-backed project store is deferred. The current implementation returns `sub_modules: Vec::new()` for all modules. Sub-module data is available through the filesystem-backed store and the remote HTTP-backed `ModuleRepository`.
<!-- ITO:END -->
