# Spec: release-automation

## Purpose

Define the `release-automation` capability and its current-truth behavior. This spec captures requirements and scenarios (for example: Release PR is created and maintained automatically).

## Requirements

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

### Requirement: Release automation supports a root workspace with nested crate directories

The release automation MUST work with a root-level Cargo workspace where member crates are organized under subdirectories (e.g., `ito-rs/crates/`).

#### Scenario: Workflows reference the root workspace with nested members

- **GIVEN** the workspace manifest is `Cargo.toml` at the repository root and member crates live under `ito-rs/crates/`
- **WHEN** release automation runs in CI
- **THEN** workflows reference the root workspace (implicitly by running at repo root, or explicitly via `manifest_path` / `--manifest-path`)
- **AND** no step assumes crates are located at the repository root

### Requirement: Default and experimental feature sets have separate verification lanes

Repository automation SHALL verify the default shipping feature set independently from an explicit all-features experimental lane. Make targets and CI jobs MUST make the selected lane visible in their names and commands.

- **Requirement ID**: release-automation:split-feature-verification

#### Scenario: Default lane verifies the shipped build

- **WHEN** the default build, test, lint, documentation, or coverage lane runs
- **THEN** it uses the same default feature selection as the distributed `ito-cli` binary
- **AND** includes regression evidence that the backend crate is absent from the normal dependency graph
- **AND** verifies the default Ralph, loop, and migration-instruction surface

#### Scenario: Experimental lane verifies all features

- **WHEN** the experimental verification lane runs
- **THEN** it explicitly enables all Cargo features and workspace members needed by backend and coordination support
- **AND** runs their tests and lints without changing the default shipping feature set

### Requirement: Release artifacts contain only default CLI features

Cargo-dist, GitHub Release, installer, and Homebrew artifacts for `ito-cli` SHALL build the default CLI feature set and MUST NOT include backend or coordination-branch implementation code. Release automation SHALL retain explicit evidence of the selected feature set.

- **Requirement ID**: release-automation:default-artifact-features

#### Scenario: Cargo-dist builds the standard binary

- **WHEN** a version tag triggers cargo-dist packaging
- **THEN** cargo-dist selects `ito-cli` with its standard default features
- **AND** the packaged binary includes web and iteration behavior
- **AND** excludes backend and coordination-branch behavior

#### Scenario: Experimental backend remains buildable outside standard artifacts

- **WHEN** a developer or experimental CI job explicitly enables backend support
- **THEN** Cargo resolves a version-compatible published or workspace `ito-backend` crate
- **AND** standard GitHub Release and Homebrew artifacts remain unchanged

### Requirement: Shared default dependencies are reported accurately

Build and release evidence MUST distinguish feature-gated implementation code from dependencies that remain required by default functionality. The change MUST NOT claim that `rusqlite`, `sha2`, or `hex` disappear from the default dependency graph unless separate evidence proves that result.

- **Requirement ID**: release-automation:accurate-dependency-evidence

#### Scenario: Dependency evidence is reviewed

- **WHEN** implementation records before-and-after Cargo dependency evidence
- **THEN** it reports whether backend and coordination implementation code is compiled
- **AND** separately reports shared crates that remain for validation, task analysis, or front-matter behavior
