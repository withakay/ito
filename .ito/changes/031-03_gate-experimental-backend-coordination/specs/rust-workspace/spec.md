<!-- ITO:START -->
## ADDED Requirements

### Requirement: Experimental subsystems are independent opt-in features

The Rust workspace SHALL expose backend support and coordination-branch support as independent additive Cargo features. Neither feature SHALL be enabled by the default `ito-cli` build, and enabling one feature MUST NOT implicitly enable the other.

- **Requirement ID**: rust-workspace:independent-experimental-features

#### Scenario: Default CLI excludes both experimental subsystems

- **WHEN** the workspace builds `ito-cli` with its default features
- **THEN** the build does not enable the backend feature
- **AND** the build does not enable the coordination-branch feature
- **AND** the CLI default feature set contains `web`

#### Scenario: Backend can be enabled independently

- **WHEN** `ito-cli` is built with the backend feature and without the coordination-branch feature
- **THEN** backend client and server integration code is available
- **AND** coordination-branch synchronization and worktree wiring code is not compiled

#### Scenario: Coordination can be enabled independently

- **WHEN** `ito-cli` is built with the coordination-branch feature and without the backend feature
- **THEN** coordination-branch synchronization and worktree wiring code is available
- **AND** backend client and server integration code is not compiled

### Requirement: Workspace defaults select the primary CLI

The root Cargo workspace SHALL declare `ito-cli` as its default member while retaining experimental crates as workspace members for explicit builds, tests, and releases.

- **Requirement ID**: rust-workspace:primary-default-member

#### Scenario: Plain workspace build selects the CLI

- **WHEN** a developer runs `cargo build` from the repository root without `--workspace` or `-p`
- **THEN** Cargo selects the primary `ito-cli` package through `default-members`
- **AND** does not select `ito-backend` as a top-level package

#### Scenario: Explicit workspace build retains experimental coverage

- **WHEN** a developer runs the documented all-features workspace check
- **THEN** the experimental backend crate and feature-gated coordination code are built and tested

### Requirement: Default builds retain the complete iteration surface

Feature gating MUST NOT remove the proposal, apply, review, archive, Ralph, loop, or iteration workflows from the default CLI. The standard migration-to-main agent instruction introduced by `031-01_migrate-coordination-state-to-main` MUST remain compiled and renderable without either experimental feature.

- **Requirement ID**: rust-workspace:default-iteration-surface

#### Scenario: Ralph and loop remain available

- **WHEN** a user inspects the default `ito` binary
- **THEN** the Ralph and loop command surfaces remain available
- **AND** they do not require backend or coordination-branch features merely to start an iteration workflow

#### Scenario: Migration recovery remains available

- **GIVEN** `ito-cli` was built without backend and coordination-branch features
- **WHEN** a user runs `ito agent instruction migrate-to-main`
- **THEN** Ito renders the migration instruction successfully
- **AND** does not link or invoke coordination synchronization code to render it
<!-- ITO:END -->
