## MODIFIED Requirements

### Requirement: ito-core contains business logic only

The `ito-core` crate SHALL contain only business logic (workflow, archive, validate, installers, create, list, show, ralph). Configuration, utilities, and discovery SHALL be extracted to other crates.

#### Scenario: Core does not export config modules
- **WHEN** examining `ito-core` public API
- **THEN** there are no `config`, `ito_dir`, or `output` modules

#### Scenario: Core does not export utility modules
- **WHEN** examining `ito-core` public API
- **THEN** there are no `io`, `paths`, `id`, or `match_` modules

#### Scenario: Core does not export discovery
- **WHEN** examining `ito-core` public API
- **THEN** there is no `discovery` module

### Requirement: ito-core dependencies

The `ito-core` crate SHALL depend on `ito-config`, `ito-domain`, `ito-common`, `ito-templates`, and `ito-harness`. It SHALL NOT depend on CLI crates.

#### Scenario: Core depends on config and domain
- **WHEN** examining `ito-core/Cargo.toml`
- **THEN** dependencies include `ito-config` and `ito-domain`

#### Scenario: Core does not depend on CLI
- **WHEN** examining `ito-core/Cargo.toml`
- **THEN** there is no dependency on `ito-cli`

## ADDED Requirements

### Requirement: Marker-based file updates inlined from ito-fs

The `ito-core` crate SHALL provide marker-based file update functionality (previously in `ito-fs`) for installer operations.

#### Scenario: Update file between markers
- **WHEN** calling marker update function with content and markers
- **THEN** content between markers is replaced, preserving content outside markers

#### Scenario: ito-fs crate removed
- **WHEN** examining workspace members
- **THEN** `ito-fs` is not listed (functionality inlined into core)
