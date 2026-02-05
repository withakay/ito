# Release Artifacts Specification

## Purpose

Define the `release-artifacts` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: GitHub Releases include cross-platform binaries

The project SHALL publish GitHub Releases that include prebuilt `ito` binaries for supported OS/architecture targets.

#### Scenario: Release is created from a version tag

- **WHEN** a maintainer pushes a tag matching `vX.Y.Z`
- **THEN** CI builds `ito` binaries for each supported target
- **AND** CI uploads the binaries as assets to the GitHub Release for that tag

### Requirement: Release artifacts include checksums

Each release SHALL publish checksums for every distributed artifact.

#### Scenario: Checksums are attached to the release

- **WHEN** CI publishes release artifacts
- **THEN** it also publishes a checksum file that covers all artifacts

### Requirement: Release version matches the Rust crate version

The release pipeline MUST ensure the Git tag version aligns with the `ito` crate version to avoid mismatched binaries.

#### Scenario: Version mismatch fails the release

- **WHEN** a tag `vX.Y.Z` is pushed
- **AND** the `ito` crate version does not match `X.Y.Z`
- **THEN** the release workflow fails before publishing artifacts
