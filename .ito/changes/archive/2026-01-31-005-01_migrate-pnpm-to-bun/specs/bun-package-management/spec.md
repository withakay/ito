## Purpose

Define the requirements for using Bun as the primary package manager for installing, managing, and locking dependencies.

## ADDED Requirements

### Requirement: Bun lockfile management

The system SHALL use `bun.lock` as the canonical lockfile for dependency resolution.

#### Scenario: Fresh install generates lockfile

- **WHEN** a developer runs `bun install` in a clean checkout without `bun.lock`
- **THEN** Bun SHALL generate `bun.lock` with resolved dependency versions

#### Scenario: Frozen lockfile in CI

- **WHEN** CI runs `bun ci` (or `bun install --frozen-lockfile`)
- **THEN** Bun SHALL fail if dependencies don't match `bun.lock` exactly

### Requirement: Migration from pnpm lockfile

The system SHALL support automatic migration from `pnpm-lock.yaml` to `bun.lock`.

#### Scenario: First install migrates pnpm lockfile

- **WHEN** a developer runs `bun install` with `pnpm-lock.yaml` present but no `bun.lock`
- **THEN** Bun SHALL read `pnpm-lock.yaml` and generate equivalent `bun.lock`
- **AND** `pnpm-lock.yaml` SHALL remain unchanged for manual verification

### Requirement: Dependency lifecycle scripts

The system SHALL execute trusted dependency lifecycle scripts using Bun's security model.

#### Scenario: Project lifecycle scripts execute

- **WHEN** `bun install` runs
- **THEN** project-level `postinstall` and `prepare` scripts SHALL execute

#### Scenario: Trusted dependency scripts execute

- **WHEN** `bun install` runs
- **THEN** lifecycle scripts from Bun's internal trusted list (e.g., esbuild) SHALL execute

### Requirement: Package installation

The system SHALL install dependencies using Bun's package manager.

#### Scenario: Install from package.json

- **WHEN** a developer runs `bun install`
- **THEN** Bun SHALL install all dependencies listed in `package.json`
- **AND** Bun SHALL create or update `bun.lock`

#### Scenario: Add new dependency

- **WHEN** a developer runs `bun add <package>`
- **THEN** Bun SHALL add the package to `package.json` and `bun.lock`
- **AND** Bun SHALL install the package to `node_modules/`

### Requirement: Cross-platform compatibility

The system SHALL support package operations on Linux, macOS, and Windows.

#### Scenario: Install on all platforms

- **WHEN** `bun install` runs on Linux, macOS, or Windows
- **THEN** dependencies SHALL install successfully on all platforms
- **AND** lockfile SHALL be identical across platforms
