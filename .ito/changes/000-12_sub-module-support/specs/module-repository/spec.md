<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: ModuleRepository supports runtime-selected implementations

`ModuleRepository` SHALL support both filesystem-backed and remote-backed implementations, with callers resolving module and sub-module data through the selected implementation for the current persistence mode.

#### Scenario: Remote mode lists modules through selected repository

- **GIVEN** remote persistence mode is active
- **WHEN** a caller requests modules through `ModuleRepository`
- **THEN** the repository returns module summaries from the selected remote-backed implementation
- **AND** each summary includes any nested sub-module summaries available for that module

#### Scenario: Remote mode resolves a module without local markdown

- **GIVEN** remote persistence mode is active
- **AND** local `.ito/modules/` markdown is absent
- **WHEN** a caller loads a module by ID or name through `ModuleRepository`
- **THEN** the repository returns the module from the selected remote-backed implementation
- **AND** the returned module includes any nested `sub_modules`

#### Scenario: Get module by ID includes nested sub-modules

- **GIVEN** module `024` has sub-modules `024.01` and `024.02`
- **WHEN** calling `module_repo.get("024")`
- **THEN** the returned `Module` includes `sub_modules` populated with those canonical sub-module IDs

#### Scenario: Remote mode resolves a sub-module without local markdown

- **GIVEN** remote persistence mode is active
- **AND** local `.ito/modules/` markdown is absent
- **WHEN** a caller loads sub-module `024.01` through `ModuleRepository`
- **THEN** the repository returns the sub-module from the selected remote-backed implementation

## ADDED Requirements

### Requirement: ModuleRepository provides sub-module access

`ModuleRepository` SHALL provide methods to list and get sub-modules.

#### Scenario: List sub-modules for a parent module

- **GIVEN** module `024_ito-backend` has sub-module directories under `.ito/modules/024_ito-backend/sub/`
- **WHEN** calling `module_repo.list_sub_modules("024")`
- **THEN** it returns a `Vec<SubModuleSummary>` with id, name, and change count for each

#### Scenario: Get a specific sub-module by canonical ID

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

#### Scenario: Remote-backed implementation lists sub-modules

- **GIVEN** remote persistence mode is active
- **AND** the selected remote-backed implementation stores sub-module data for module `024`
- **WHEN** calling `module_repo.list_sub_modules("024")`
- **THEN** it returns those sub-modules without requiring local `.ito/modules/` markdown
<!-- ITO:END -->
