## ADDED Requirements

### Requirement: Core does not depend on adapter frameworks

`ito-core` MUST NOT depend on adapter/framework crates.

At minimum, `ito-core` MUST NOT depend on `clap`, `crossterm`, or `axum`.

#### Scenario: Core Cargo.toml contains no adapter deps

- **WHEN** inspecting `ito-rs/crates/ito-core/Cargo.toml`
- **THEN** it MUST NOT include `clap`
- **AND** it MUST NOT include `crossterm`
- **AND** it MUST NOT include `axum`
