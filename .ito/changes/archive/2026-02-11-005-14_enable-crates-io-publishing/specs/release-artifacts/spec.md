## MODIFIED Requirements

### Requirement: GitHub Releases include cross-platform binaries

The project SHALL publish GitHub Releases that include prebuilt `ito` binaries for supported OS/architecture targets. All release workflow jobs SHALL run on the `withakay-selfhost` runner group unless a specific OS/architecture matrix entry requires a different runner.

#### Scenario: Release is created from a version tag

- **WHEN** a maintainer pushes a tag matching `vX.Y.Z`
- **THEN** CI builds `ito` binaries for each supported target
- **AND** CI uploads the binaries as assets to the GitHub Release for that tag

#### Scenario: Release workflow uses self-hosted runners

- **WHEN** the release workflow executes
- **THEN** all jobs that do not require a specific OS runner (e.g., meta, check_assets, validate_version, upload_assets) SHALL use `runs-on: group: withakay-selfhost`

#### Scenario: Release workflow triggers are clean

- **WHEN** the release workflow is triggered
- **THEN** it SHALL NOT reference non-existent workflows (e.g., "Release Please")
- **AND** it SHALL be triggered by `release` events, `workflow_dispatch`, and optionally by completion of the `Release-plz` workflow

## ADDED Requirements

### Requirement: Release pipeline publishes to crates.io

The release pipeline SHALL publish workspace crates to crates.io as part of the release process, in addition to creating GitHub releases and git tags.

#### Scenario: crates.io publish occurs during release

- **WHEN** release-plz runs the `release` command
- **THEN** it SHALL publish eligible crates to crates.io
- **AND** it SHALL create git tags and GitHub releases as before
