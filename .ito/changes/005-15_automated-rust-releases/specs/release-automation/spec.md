## ADDED Requirements

### Requirement: Release PR is created and maintained automatically

The system SHALL create and update a "release PR" that contains version bumps and changelog updates for the Rust workspace.

#### Scenario: Release PR is opened on main

- **GIVEN** commits include changes in release-relevant Rust crate directories
- **WHEN** commits are pushed to the `main` branch
- **THEN** CI creates or updates a release PR
- **AND** the release PR includes the required version and changelog changes

### Requirement: Non-Rust-only changes do not force a version bump

The system MUST avoid bumping crate versions when changes do not affect release-relevant Rust crates.

#### Scenario: Docs-only changes do not bump versions

- **GIVEN** a set of commits that only change non-Rust files (e.g., docs, CI configuration)
- **WHEN** release automation runs
- **THEN** crate versions are not bumped
- **AND** no crates.io publish step is attempted for those crates

### Requirement: Merging the release PR produces a version tag and publishes crates

The system SHALL publish configured crates to crates.io and create a git tag `vX.Y.Z` when a release PR is merged.

#### Scenario: Tags and publish occur after merge

- **WHEN** the release PR is merged into `main`
- **THEN** CI publishes crates to crates.io in dependency order
- **AND** CI creates a git tag matching `vX.Y.Z`

### Requirement: The installed CLI binary name is `ito`

The system MUST distribute the Ito CLI such that the installed executable name is `ito` (or `ito.exe` on Windows).

#### Scenario: Release artifacts contain the expected executable name

- **WHEN** CI builds release artifacts for `ito-cli`
- **THEN** the packaged artifact contains an executable named `ito` (or `ito.exe` on Windows)

### Requirement: Version tags trigger artifact packaging and GitHub Release assets

The system SHALL produce cross-platform release artifacts and attach them to the GitHub Release associated with the `vX.Y.Z` tag.

#### Scenario: Artifacts are attached to the release

- **WHEN** a tag matching `vX.Y.Z` is created
- **THEN** CI builds and packages release artifacts for supported targets
- **AND** CI uploads artifacts and checksums to the GitHub Release for that tag

### Requirement: Release automation supports a subdirectory Cargo workspace

The release automation MUST work in a monorepo where Rust crates may live under subdirectories, and MUST not rely on a single hard-coded workspace location.

#### Scenario: Workflows use an explicit manifest path

- **GIVEN** Rust crates live under `ito-rs/` (or equivalent)
- **WHEN** release automation runs in CI
- **THEN** workflows use an explicit `manifest_path` / `--manifest-path` and/or a root workspace configuration
- **AND** no step assumes crates are located at the git root
