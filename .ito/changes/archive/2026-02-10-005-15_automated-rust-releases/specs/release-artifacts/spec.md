## MODIFIED Requirements

### Requirement: GitHub Releases include cross-platform binaries

The project SHALL publish GitHub Releases that include prebuilt `ito` binaries for supported OS/architecture targets.

#### Scenario: Release is created from a version tag

- **WHEN** a tag matching `vX.Y.Z` is created in the repository
- **THEN** CI builds `ito` binaries for each supported target
- **AND** CI uploads the binaries as assets to the GitHub Release for that tag

#### Scenario: Release artifacts contain the expected executable name

- **WHEN** CI packages release artifacts for `ito-cli`
- **THEN** the packaged artifact contains an executable named `ito` (or `ito.exe` on Windows)
