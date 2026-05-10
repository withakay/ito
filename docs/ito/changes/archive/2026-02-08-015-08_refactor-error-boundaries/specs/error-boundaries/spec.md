## ADDED Requirements

### Requirement: Domain errors are framework-agnostic

`ito-domain` MUST define domain error types that are framework-agnostic.

Domain error types MUST implement `std::error::Error` and `Display`.

`ito-domain` MUST NOT depend on diagnostic or UI frameworks (for example: `miette`, `clap`, `crossterm`, `axum`).

#### Scenario: Domain has no diagnostic dependencies

- **WHEN** inspecting `ito-rs/crates/ito-domain/Cargo.toml`
- **THEN** it MUST NOT include `miette`

#### Scenario: Domain has no adapter dependencies

- **WHEN** inspecting `ito-rs/crates/ito-domain/Cargo.toml`
- **THEN** it MUST NOT include `clap`, `crossterm`, or `axum`

### Requirement: Core translates infrastructure failures

`ito-core` SHALL translate infrastructure failures (filesystem, schema parsing, process execution) into structured use-case errors with actionable context (operation + relevant path/identifier).

#### Scenario: Missing file is reported with context

- **GIVEN** a use-case needs to read an on-disk artifact file
- **WHEN** the file is missing
- **THEN** `ito-core` returns an error that identifies the operation and the missing path
