## REMOVED Requirements

### Requirement: Bun lockfile management

This requirement is removed; the system SHALL NOT use `bun.lock`.
**Reason**: No Bun-managed dependencies remain.
**Migration**: Rust dependencies are managed via Cargo.lock.

#### Scenario: Cargo.lock is the lockfile

- **WHEN** a developer installs/builds Ito
- **THEN** dependency resolution SHALL be based on Cargo tooling

### Requirement: Migration from pnpm lockfile

This requirement is removed; the system SHALL NOT require pnpm/bun lockfile migration.
**Reason**: No pnpm/bun lockfile migration is needed.
**Migration**: None.

#### Scenario: No JS lockfile migration

- **WHEN** a developer checks out the repo
- **THEN** there is no requirement to migrate pnpm/bun lockfiles

### Requirement: Dependency lifecycle scripts

This requirement is removed; the system SHALL NOT rely on npm lifecycle scripts.
**Reason**: No npm lifecycle scripts remain.
**Migration**: None.

#### Scenario: No postinstall/prepare scripts

- **WHEN** a developer sets up the repo
- **THEN** setup SHALL NOT rely on Node lifecycle scripts

### Requirement: Package installation

This requirement is removed; the system SHALL NOT require Node package installation.
**Reason**: Node package installation is removed.
**Migration**: Use Cargo for Rust dependencies.

#### Scenario: No bun install

- **WHEN** a developer follows repo setup instructions
- **THEN** they SHALL NOT need to run `bun install`

### Requirement: Cross-platform compatibility

This requirement is removed; the system SHALL support cross-platform operations via Rust tooling.
**Reason**: The relevant compatibility surface is Rust builds.
**Migration**: Covered by Rust CI matrix.

#### Scenario: Cargo builds on supported platforms

- **WHEN** CI runs on Linux, macOS, and Windows
- **THEN** Rust build and tests SHALL succeed
