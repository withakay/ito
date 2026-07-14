# Change Coordination Branch

## Purpose

This spec defines the current behavior and requirements for change coordination branch.

## Requirements

### Requirement: Instruction generation does not require remote coordination sync

Instruction generation MUST remain usable offline. The system MUST NOT require a remote coordination-branch fetch to print apply instructions.

#### Scenario: Apply instruction generation skips coordination sync by default

- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system does not fetch the coordination branch from `origin` by default

### Requirement: Remote coordination sync is available as an opt-in preflight

The system SHALL allow users to opt in to a coordination-branch fetch preflight before apply instructions are printed.

#### Scenario: Coordination sync preflight is enabled

- **GIVEN** coordination-branch sync preflight is enabled
- **WHEN** an agent runs `ito agent instruction apply --change <id>`
- **THEN** the system attempts to fetch the coordination branch from `origin` before printing instructions

### Requirement: Coordination branch behavior is independently feature-gated

Coordination-branch fetch, reservation, push, synchronization, dedicated-worktree lifecycle, symlink or junction wiring, repair, and coordination-only validation rules SHALL be compiled only when the coordination-branch Cargo feature is enabled. This feature MUST NOT require or enable backend support.

- **Requirement ID**: change-coordination-branch:independent-feature-gate

#### Scenario: Default build omits coordination implementation

- **WHEN** Cargo builds `ito-cli` with its default features
- **THEN** coordination synchronization and worktree implementation modules are not compiled into the binary
- **AND** ordinary proposal and iteration commands operate against the default main-compatible storage workflow

#### Scenario: Experimental coordination build excludes backend

- **WHEN** Cargo builds `ito-cli` with only the coordination-branch experimental feature
- **THEN** coordination synchronization and worktree behavior is available
- **AND** backend client, server, and remote repository behavior remains compiled out

### Requirement: Compiled-out coordination requests fail without fallback

When parsed configuration or an invoked operation requests coordination worktree or branch behavior from a binary built without the coordination-branch feature, Ito MUST return a typed feature-unavailable error. Ito MUST NOT silently reinterpret worktree-backed state as embedded or main-compatible state.

- **Requirement ID**: change-coordination-branch:compiled-out-error

#### Scenario: Legacy worktree configuration reaches a default binary

- **GIVEN** legacy configuration enables coordination or selects `changes.coordination_branch.storage: worktree`
- **AND** the active Ito binary was built without coordination-branch support
- **WHEN** a command would read or mutate coordinated state
- **THEN** Ito returns a typed feature-unavailable error identifying `coordination-branch`
- **AND** the error directs the user to `ito agent instruction migrate-to-main`
- **AND** Ito does not write through legacy coordination symlinks or fall back to embedded storage

#### Scenario: Recovery instructions remain exempt

- **GIVEN** legacy coordination configuration is present
- **AND** coordination support is compiled out
- **WHEN** a user requests help, configuration diagnostics, or `ito agent instruction migrate-to-main`
- **THEN** the command remains available for diagnosis or recovery
- **AND** does not attempt coordination sync as a precondition
