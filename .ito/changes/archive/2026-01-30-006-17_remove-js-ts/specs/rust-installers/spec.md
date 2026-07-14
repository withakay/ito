## REMOVED Requirements

### Requirement: Non-interactive installers match TypeScript byte-for-byte

This requirement is removed; the Rust implementation SHALL NOT require TypeScript for installer verification.
**Reason**: TypeScript implementation is removed; byte-for-byte comparison against TypeScript is no longer possible or meaningful.
**Migration**: Validate installer outputs via Rust-only tests and deterministic assets.

#### Scenario: No TypeScript byte-for-byte parity requirement

- **WHEN** running installers in non-interactive mode
- **THEN** the project SHALL NOT require executing TypeScript to validate output bytes

## ADDED Requirements

### Requirement: Installer outputs are deterministic and validated in Rust

Installer outputs MUST be deterministic under non-interactive flags and MUST be validated by Rust test coverage.

#### Scenario: Rust tests validate installers

- **WHEN** running `cargo test --workspace`
- **THEN** installer-related tests MUST validate the expected file outputs
