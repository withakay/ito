## MODIFIED Requirements

### Requirement: ito-domain crate dependencies

The `ito-domain` crate SHALL depend on `ito-common` and `ito-schemas` only. It SHALL NOT depend on `ito-core`, `ito-config`, or CLI crates.

#### Scenario: Crate depends on ito-common and ito-schemas
- **WHEN** examining `ito-domain/Cargo.toml`
- **THEN** the only `ito-*` dependencies are `ito-common` and `ito-schemas`

## ADDED Requirements

### Requirement: Discovery module in ito-domain

The `ito-domain` crate SHALL provide a `discovery` module for listing ito artifacts (changes, modules, specs) from the filesystem.

#### Scenario: List changes in ito directory
- **WHEN** calling `discovery::list_changes(fs, ito_path)`
- **THEN** returns list of change IDs found in `{ito_path}/changes/`

#### Scenario: List modules in ito directory
- **WHEN** calling `discovery::list_modules(fs, ito_path)`
- **THEN** returns list of module IDs found in `{ito_path}/modules/`

#### Scenario: List specs in ito directory
- **WHEN** calling `discovery::list_specs(fs, ito_path)`
- **THEN** returns list of spec names found in `{ito_path}/specs/`

#### Scenario: Discovery uses FileSystem trait
- **WHEN** discovery functions are called
- **THEN** they accept a generic `F: FileSystem` parameter for testability
