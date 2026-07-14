## ADDED Requirements

### Requirement: NPM install is optional and produces a working binary

The project SHALL provide an npm-based installation method that results in a working native `ito` binary, but this method MUST remain optional.

#### Scenario: Users can install via npm

- **WHEN** a user runs the documented npm install command on a supported platform
- **THEN** `ito --version` runs successfully

### Requirement: Package versions match the released Ito version

Published npm package versions MUST align with the corresponding Ito release version.

#### Scenario: Version mismatch is prevented

- **WHEN** publishing npm packages for a release
- **THEN** CI fails if the npm package version does not match the release version

### Requirement: Platform packages map to supported targets

The npm distribution SHALL publish platform-specific packages that correspond to the supported OS/arch release targets.

#### Scenario: Platform selection is deterministic

- **WHEN** a user installs on a supported platform
- **THEN** npm resolves the correct platform-specific package for that OS/arch
