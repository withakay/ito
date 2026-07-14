# Rust Packaging Transition Specification

## Purpose

Define the `rust-packaging-transition` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Transition plan preserves `ito` command name

The transition plan MUST keep the user-facing `ito` command stable and MUST define `ito-rs` as the supported implementation for the `ito` command.

The legacy TypeScript/Bun implementation MUST be treated as deprecated and MUST NOT be installed or distributed in a way that claims the `ito` command by default.

#### Scenario: npm-installed `ito` continues to work (Rust default)

- **GIVEN** a user who previously installed `@withakay/ito`
- **WHEN** they upgrade to a version where `ito` resolves to the Rust implementation
- **THEN** `ito --help` and `ito --version` behave identically at the CLI-contract level
- **AND** the output clearly identifies `ito-rs` as the supported implementation

### Requirement: Platform artifacts and verification are defined

The plan MUST define build artifacts per platform and how they are verified, and it MUST distinguish supported `ito-rs` artifacts from any deprecated TypeScript/Bun artifacts.

#### Scenario: Release checklist is explicit

- **GIVEN** the packaging documentation
- **WHEN** a release engineer follows the checklist
- **THEN** it includes commands to build `ito-rs` artifacts for supported platforms
- **AND** it includes checksum/integrity verification
- **AND** it documents any legacy TypeScript/Bun artifacts as deprecated and non-default (if shipped)
