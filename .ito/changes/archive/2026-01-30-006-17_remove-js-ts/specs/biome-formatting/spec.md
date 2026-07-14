## REMOVED Requirements

### Requirement: Formatting command exists

This requirement is removed; formatting SHALL be performed by Rust tooling.
**Reason**: Biome tooling is removed.
**Migration**: Use `cargo fmt` via `make lint`.

#### Scenario: Rust formatting replaces Biome

- **WHEN** a developer formats code
- **THEN** formatting SHALL be performed by Rust tooling

### Requirement: Formatting can be checked in CI

This requirement is removed; CI MUST check formatting via `cargo fmt --check`.
**Reason**: Biome formatting checks are removed.
**Migration**: CI uses `cargo fmt --check`.

#### Scenario: CI checks rustfmt

- **WHEN** CI runs formatting checks
- **THEN** it SHALL run `cargo fmt --check`
