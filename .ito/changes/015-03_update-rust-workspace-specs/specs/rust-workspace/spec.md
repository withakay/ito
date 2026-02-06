## MODIFIED Requirements

### Requirement: Planned crate directories exist

The workspace MUST include crate directories for the supported Rust workspace crates.

#### Scenario: Crate directories exist

- **WHEN** inspecting `ito-rs/crates/`
- **THEN** `ito-cli` MUST exist
- **AND** `ito-common` MUST exist
- **AND** `ito-config` MUST exist
- **AND** `ito-core` MUST exist
- **AND** `ito-domain` MUST exist
- **AND** `ito-harness` MUST exist
- **AND** `ito-logging` MUST exist
- **AND** `ito-models` MUST exist
- **AND** `ito-schemas` MUST exist
- **AND** `ito-templates` MUST exist
- **AND** `ito-test-support` MUST exist
- **AND** `ito-web` MUST exist

### Requirement: Cargo workspace exists with defined crate structure

The repository MUST include a Cargo workspace at `ito-rs/` with the agreed crate structure.

#### Scenario: Workspace layout exists

- **WHEN** a developer lists `ito-rs/`
- **THEN** it contains a workspace `Cargo.toml` and `crates/`
- **AND** the crates include `ito-cli`, `ito-core`, `ito-domain`, `ito-templates`, `ito-schemas`, `ito-harness`, `ito-test-support`, `ito-web`
