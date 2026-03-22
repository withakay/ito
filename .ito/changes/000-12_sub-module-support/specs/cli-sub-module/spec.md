<!-- ITO:START -->
## ADDED Requirements

### Requirement: Sub-modules can be created via CLI

The CLI SHALL support creating a sub-module under an existing parent module.

#### Scenario: Create sub-module with name and parent module

- **WHEN** user executes `ito create sub-module <name> --module <module-id>`
- **THEN** a new sub-module is created under the specified parent module
- **AND** the sub-module number is auto-allocated (next available two-digit number under that parent)
- **AND** a `module.md` is written at `.ito/modules/NNN_<parent>/sub/SS_<name>/module.md`
- **AND** the command prints the new sub-module ID (e.g., `Created sub-module '024.01_auth'`)

#### Scenario: Create sub-module with optional description

- **WHEN** user executes `ito create sub-module <name> --module <id> --description <text>`
- **THEN** the sub-module `module.md` includes the provided description

#### Scenario: Parent module must exist

- **WHEN** user executes `ito create sub-module <name> --module <id>` and the parent module does not exist
- **THEN** the command exits with an error indicating the parent module was not found

### Requirement: Sub-modules appear nested in module listings

The `ito list --modules` command SHALL display sub-modules nested under their parent module.

#### Scenario: Listing shows nested sub-modules

- **WHEN** user executes `ito list --modules`
- **AND** module `024_ito-backend` has sub-modules `024.01_auth` and `024.02_sync`
- **THEN** the output shows `024_ito-backend` with indented sub-module entries beneath it
- **AND** each sub-module entry shows its ID, name, and change count

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

### Requirement: Changes can be created under a sub-module

`ito create change` SHALL accept an optional `--sub-module <id>` flag.

#### Scenario: Create change under sub-module

- **WHEN** user executes `ito create change <name> --sub-module 024.01`
- **THEN** the change is allocated using the `NNN.SS-NN_name` format (e.g., `024.01-01_name`)
- **AND** the change is added to the sub-module's `module.md` checklist
- **AND** the parent module's `module.md` checklist is NOT modified

#### Scenario: --sub-module and --module are mutually exclusive

- **WHEN** user provides both `--module` and `--sub-module` to `ito create change`
- **THEN** the command exits with an error explaining they are mutually exclusive
<!-- ITO:END -->
