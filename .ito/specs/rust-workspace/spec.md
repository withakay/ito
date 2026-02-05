# rust-workspace Specification

## Purpose

Define the Rust `ito-rs/` Cargo workspace layout and baseline quality gates for the port.

## Requirements

### Requirement: Workspace exists at `ito-rs/` and passes baseline checks

The repository MUST contain a Cargo workspace rooted at `ito-rs/` and it MUST be formatted, lint-clean, and testable.

#### Scenario: Workspace is present and builds

- **WHEN** running workspace checks in `ito-rs/`
- **THEN** `cargo test --workspace` MUST pass
- **AND** `cargo fmt --check` MUST pass
- **AND** `cargo clippy --workspace -- -D warnings` MUST pass

### Requirement: Planned crate directories exist

The workspace MUST include crate directories for the planned port layers.

#### Scenario: Crate directories exist

- **WHEN** inspecting `ito-rs/crates/`
- **THEN** `ito-cli` MUST exist
- **AND** `ito-core` MUST exist
- **AND** `ito-fs` MUST exist
- **AND** `ito-templates` MUST exist
- **AND** `ito-schemas` MUST exist
- **AND** `ito-workflow` MUST exist
- **AND** `ito-harness` MUST exist
- **AND** `ito-test-support` MUST exist

### Requirement: Coverage command is documented

The workspace MUST document a command to measure coverage across the workspace.

#### Scenario: Coverage documentation exists

- **WHEN** reading `ito-rs/README.md`
- **THEN** it MUST include a coverage command (for example, `cargo llvm-cov --workspace`)

### Requirement: Cargo workspace exists with defined crate structure

The repository MUST include a Cargo workspace at `ito-rs/` with the agreed crate structure.

#### Scenario: Workspace layout exists

- **WHEN** a developer lists `ito-rs/`
- **THEN** it contains a workspace `Cargo.toml` and `crates/`
- **AND** the crates include `ito-cli`, `ito-core`, `ito-fs`, `ito-templates`, `ito-test-support`

### Requirement: Baseline quality tooling is runnable

The workspace MUST support formatting, clippy linting, tests, and coverage measurement.

#### Scenario: Tooling commands succeed

- **WHEN** a developer runs formatting, clippy, and tests
- **THEN** `cargo fmt --check`, `cargo clippy --workspace`, and `cargo test --workspace` succeed

#### Scenario: Coverage command is documented

- **WHEN** a developer reads `ito-rs/README.md`
- **THEN** it documents running `cargo llvm-cov --workspace`
