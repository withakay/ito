# Spec Delta: rust-workspace

## Purpose

Create the Rust `ito-rs/` workspace and baseline tooling required for the port.

## ADDED Requirements

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
