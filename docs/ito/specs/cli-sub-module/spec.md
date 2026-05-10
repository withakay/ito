<!-- ITO:START -->
## ADDED Requirements

### Requirement: Sub-modules can be created via CLI

The CLI SHALL support creating a sub-module under an existing parent module.

#### Scenario: Create sub-module with name and parent module

- **WHEN** user executes `ito create sub-module <name> --module <module-id>`
- **THEN** a new sub-module is created under the specified parent module
- **AND** the sub-module number is auto-allocated (next available two-digit number under that parent)
- **AND** a `module.md` is written at `.ito/modules/NNN_<parent>/sub/SS_<name>/module.md`
- **AND** the command prints the canonical sub-module ID (for example `024.01`) together with the sub-module name

#### Scenario: Create sub-module with optional description

- **WHEN** user executes `ito create sub-module <name> --module <id> --description <text>`
- **THEN** the sub-module `module.md` includes the provided description

#### Scenario: Parent module must exist

- **WHEN** user executes `ito create sub-module <name> --module <id>` and the parent module does not exist
- **THEN** the command exits with an error indicating the parent module was not found

#### Scenario: Duplicate sub-module name under the same parent is rejected

- **WHEN** user executes `ito create sub-module auth --module 024`
- **AND** sub-module `024.01_auth` or any other `*_auth` sub-module already exists under parent module `024`
- **THEN** the command exits with an error indicating the sub-module name is already in use for that parent module

#### Scenario: Create sub-module is rejected in remote persistence mode

- **GIVEN** remote persistence mode is active
- **WHEN** user executes `ito create sub-module auth --module 024`
- **THEN** the command exits with an actionable error indicating sub-module creation currently requires local filesystem mode

### Requirement: Sub-modules appear nested in module listings

The `ito list --modules` command SHALL display sub-modules nested under their parent module.

#### Scenario: Listing shows nested sub-modules

- **WHEN** user executes `ito list --modules`
- **AND** module `024_ito-backend` has sub-modules `024.01` and `024.02`
- **THEN** the output shows `024_ito-backend` with indented sub-module entries beneath it
- **AND** each sub-module entry shows its canonical ID, name, and change count

#### Scenario: Modules without sub-modules display unchanged

- **WHEN** user executes `ito list --modules`
- **AND** a module has no sub-modules
- **THEN** it displays exactly as before (no indented sub-entries)

### Requirement: Sub-modules can be shown via CLI

The CLI SHALL support inspecting a sub-module by its canonical ID.

#### Scenario: Show sub-module by ID

- **WHEN** user executes `ito show sub-module <NNN.SS>` or `ito show sub-module <NNN.SS_name>`
- **THEN** the command displays the sub-module metadata (id, name, description) and lists associated changes

#### Scenario: Show sub-module with unknown ID

- **WHEN** user executes `ito show sub-module 999.99`
- **THEN** the command exits with a clear error: sub-module not found

### Requirement: Sub-module listing and show commands use runtime-selected ModuleRepository

When remote persistence mode is active, read-only sub-module CLI commands SHALL resolve data through the selected `ModuleRepository` implementation.

#### Scenario: List modules with sub-modules in remote mode

- **GIVEN** remote persistence mode is active
- **WHEN** user executes `ito list --modules`
- **THEN** Ito renders parent modules and nested sub-modules from the selected remote-backed `ModuleRepository`
- **AND** the command does not require local `.ito/modules/` markdown to exist

#### Scenario: Show sub-module in remote mode

- **GIVEN** remote persistence mode is active
- **WHEN** user executes `ito show sub-module 024.01`
- **THEN** Ito renders the sub-module from the selected remote-backed `ModuleRepository`
<!-- ITO:END -->
