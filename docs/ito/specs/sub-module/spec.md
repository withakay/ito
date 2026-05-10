<!-- ITO:START -->
## ADDED Requirements

### Requirement: SubModule is a first-class named entity within a module

A `SubModule` SHALL be a named, numbered child of a parent module with its own `module.md` metadata file.

Sub-modules are one level deep only. A sub-module cannot contain another sub-module.

#### Scenario: Sub-module has a canonical ID

- **WHEN** a sub-module is created under module `024` with sub-module number `01` and name `auth`
- **THEN** its canonical sub-module ID is `024.01` (parent module ID, dot, two-digit sub-module number)
- **AND** the sub-module name is `auth`

#### Scenario: Sub-module has its own module.md

- **WHEN** a sub-module `024.01_auth` is created
- **THEN** a `module.md` file is written at `.ito/modules/024_ito-backend/sub/01_auth/module.md`
- **AND** the file contains at minimum: id, name, optional description, and a `## Changes` checklist

### Requirement: Sub-module directory layout follows a prescribed path

The filesystem layout for sub-modules SHALL be deterministic and human-readable.

#### Scenario: Sub-module metadata directory path

- **GIVEN** parent module directory `.ito/modules/NNN_<name>/`
- **WHEN** sub-module `SS_<subname>` is created
- **THEN** its metadata directory is `.ito/modules/NNN_<name>/sub/SS_<subname>/`
- **AND** its `module.md` is at `.ito/modules/NNN_<name>/sub/SS_<subname>/module.md`

#### Scenario: Changes still reside in the flat changes directory

- **WHEN** a change is created under sub-module `024.01_auth`
- **THEN** the change directory is `.ito/changes/024.01-NN_<name>/`
- **AND** no change directories are created inside the sub-module metadata directory

### Requirement: SubModule domain model captures sub-module metadata

The domain layer SHALL provide a `SubModule` struct with the fields needed to represent a sub-module.

#### Scenario: SubModule fields

- **WHEN** a sub-module is loaded
- **THEN** the resulting `SubModule` struct contains: `id` (e.g., `"024.01"`), `parent_module_id` (e.g., `"024"`), `sub_id` (e.g., `"01"`), `name` (e.g., `"auth"`), `description: Option<String>`, `change_count: u32`

### Requirement: Parent modules can own direct changes alongside sub-modules

A parent module SHALL be allowed to own module-level changes and sub-modules at the same time.

#### Scenario: Parent module and sub-module each own changes

- **GIVEN** module `024` has module-level change `024-07_health-check`
- **AND** sub-module `024.01` has change `024.01-01_add-jwt`
- **WHEN** module and sub-module metadata are loaded
- **THEN** the module-level change remains associated with parent module `024`
- **AND** the sub-module change remains associated with sub-module `024.01`
- **AND** neither change is reassigned to the other scope
<!-- ITO:END -->
