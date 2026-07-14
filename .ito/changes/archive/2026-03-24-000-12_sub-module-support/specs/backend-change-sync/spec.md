<!-- ITO:START -->
## ADDED Requirements

### Requirement: Backend change sync preserves sub-module ID component during sync

Change sync operations SHALL preserve and correctly propagate the sub-module component of `NNN.SS-NN_name` IDs when syncing change artifacts between local filesystem state and backend stores.

#### Scenario: Sync push with sub-module change ID succeeds

- **GIVEN** a local change with ID `024.01-03_add-jwt` exists
- **WHEN** sync pushes the change to the backend
- **THEN** the backend stores the change under key `024.01-03_add-jwt`
- **AND** the sub-module component `01` is not dropped or mangled during serialization

#### Scenario: Sync pull with sub-module change ID writes correct local path

- **GIVEN** the backend has a change with ID `024.01-03_add-jwt`
- **WHEN** sync pulls the change to local filesystem
- **THEN** the change is written to `.ito/changes/024.01-03_add-jwt/`
- **AND** the dot in the directory name is preserved exactly

#### Scenario: Module extraction during sync identifies correct parent module

- **WHEN** sync processes a change with ID `024.01-03_add-jwt`
- **THEN** it correctly identifies the parent module as `024` (not `024.01`)
- **AND** associates the change with module `024`'s backend scope
<!-- ITO:END -->
