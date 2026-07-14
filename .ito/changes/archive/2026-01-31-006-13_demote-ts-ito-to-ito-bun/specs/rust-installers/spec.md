## ADDED Requirements

### Requirement: `ito-rs` is installed as `ito` by default

Installers MUST ensure the default `ito` command resolves to the Rust implementation.

If the legacy TypeScript/Bun implementation is installed for legacy purposes, it MUST use a distinct command/name and MUST be labeled deprecated.

#### Scenario: Default CLI resolves to Rust

- **WHEN** a user installs Ito using the documented installer path
- **THEN** running `ito --version` indicates the Rust implementation
- **AND** the installation does not place a TypeScript/Bun `ito` ahead of Rust on PATH

### Requirement: Legacy TypeScript `ito` is removed from global cache

Installers MUST remove or disable any cached legacy TypeScript `ito` that would shadow the Rust `ito` command.

#### Scenario: Cached legacy CLI does not shadow Rust

- **GIVEN** a machine with a cached legacy TypeScript `ito` in the global cache
- **WHEN** the Rust `ito` installation or upgrade is performed
- **THEN** `ito` resolves to the Rust implementation
- **AND** the legacy cache entry is removed or renamed so it cannot shadow `ito`

## REMOVED Requirements

### Requirement: Non-interactive installers match TypeScript byte-for-byte

This requirement is removed; installer verification MUST NOT require executing the TypeScript/Bun implementation.

#### Scenario: Rust installers do not depend on TypeScript

- **WHEN** a developer runs `ito init` in non-interactive mode
- **THEN** installer outputs MUST be validated using Rust-owned templates and/or Rust golden tests
- **AND** the validation process SHALL NOT execute TypeScript/Bun code

**Reason**: The TypeScript/Bun implementation is deprecated and is no longer the canonical source for installer outputs.
**Migration**: Treat Rust `ito init` outputs as canonical and validate outputs via templates and/or golden tests instead of comparing to the TypeScript implementation.
