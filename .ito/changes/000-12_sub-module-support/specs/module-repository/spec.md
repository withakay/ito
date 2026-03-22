<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: ModuleRepository provides centralized module access

A `ModuleRepository` interface SHALL exist in `ito-domain` that provides methods for loading and querying module data, including sub-modules.

`ito-core` SHALL provide a filesystem-backed implementation of this interface for production use.

#### Scenario: Get a module by ID

- **GIVEN** a module with ID "005" and name "dev-tooling" exists
- **WHEN** calling `module_repo.get("005")`
- **THEN** it returns a `Module` object with id, name, description, and `sub_modules` (empty vec if none)

#### Scenario: List all modules

- **WHEN** calling `module_repo.list()`
- **THEN** it returns a `Vec<ModuleSummary>` with all modules
- **AND** each summary includes id, name, change count, and a `sub_modules: Vec<SubModuleSummary>` (empty if none)

#### Scenario: List modules with changes

- **WHEN** calling `module_repo.list_with_changes()`
- **THEN** it returns modules along with their associated changes
- **AND** each module entry also includes its sub-modules and their associated changes

## ADDED Requirements

### Requirement: ModuleRepository provides sub-module access

`ModuleRepository` SHALL provide methods to list and get sub-modules.

#### Scenario: List sub-modules for a parent module

- **GIVEN** module `024_ito-backend` has sub-module directories under `.ito/modules/024_ito-backend/sub/`
- **WHEN** calling `module_repo.list_sub_modules("024")`
- **THEN** it returns a `Vec<SubModuleSummary>` with id, name, description, and change count for each

#### Scenario: Get a specific sub-module by composite ID

- **GIVEN** sub-module `024.01_auth` exists
- **WHEN** calling `module_repo.get_sub_module("024.01")`
- **THEN** it returns a `SubModule` with id `"024.01"`, parent_module_id `"024"`, sub_id `"01"`, name `"auth"`, and optional description

#### Scenario: Get sub-module that does not exist returns not-found error

- **WHEN** calling `module_repo.get_sub_module("024.99")`
- **AND** that sub-module does not exist
- **THEN** it returns a not-found error (not a panic)

#### Scenario: Filesystem implementation reads sub-module metadata from sub/ directory

- **GIVEN** a sub-module directory exists at `.ito/modules/024_ito-backend/sub/01_auth/`
- **WHEN** the filesystem module repository lists or gets this sub-module
- **THEN** it reads `module.md` from that path to obtain name and description
<!-- ITO:END -->
