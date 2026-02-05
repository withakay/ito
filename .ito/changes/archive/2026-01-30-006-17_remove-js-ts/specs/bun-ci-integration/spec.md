## REMOVED Requirements

### Requirement: Bun installation in CI

This requirement is removed; CI SHALL NOT install Bun.
**Reason**: CI no longer uses Bun.
**Migration**: Install Rust toolchain in CI.

#### Scenario: CI does not install Bun

- **WHEN** CI runs
- **THEN** it SHALL NOT install Bun

### Requirement: Frozen lockfile enforcement

This requirement is removed; CI SHALL NOT enforce a Bun lockfile.
**Reason**: Bun lockfile is removed.
**Migration**: Use Cargo.lock correctness.

#### Scenario: Cargo.lock is respected

- **WHEN** CI builds/tests
- **THEN** Cargo SHALL use `Cargo.lock` as applicable

### Requirement: Build commands in CI

This requirement is removed; CI MUST build using Rust tooling.
**Reason**: Build is Rust-only.
**Migration**: Run `make build`.

#### Scenario: CI builds Rust

- **WHEN** CI runs build
- **THEN** it SHALL run `make build`

### Requirement: Test commands in CI

This requirement is removed; CI MUST run tests using Rust tooling.
**Reason**: Tests are Rust-only.
**Migration**: Run `make test`.

#### Scenario: CI runs Rust tests

- **WHEN** CI runs tests
- **THEN** it SHALL run `make test`

### Requirement: Type checking in CI

This requirement is removed; CI SHALL NOT run TypeScript type checking.
**Reason**: TypeScript is removed.
**Migration**: Rust compilation + clippy gates.

#### Scenario: No TypeScript type check

- **WHEN** CI runs
- **THEN** it SHALL NOT run `tsc --noEmit`

### Requirement: Release workflow integration

This requirement is removed; release automation SHALL NOT require Bun/changesets.
**Reason**: npm publishing is removed.
**Migration**: Use Rust-native release artifacts.

#### Scenario: No changesets publish

- **WHEN** release automation runs
- **THEN** it SHALL NOT require Bun/changesets

### Requirement: Cross-platform CI matrix

This requirement is removed; CI MUST validate Rust checks across supported platforms.
**Reason**: Still required, but for Rust.
**Migration**: Keep the CI matrix for cargo.

#### Scenario: Rust CI matrix

- **WHEN** CI runs on Linux, macOS, and Windows
- **THEN** Rust checks SHALL pass on all platforms
