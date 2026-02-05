## ADDED Requirements

### Requirement: Release PR is created and updated automatically

The system MUST run `release-please` in CI to open or update a release PR targeting the `main` branch when release-worthy changes exist.

#### Scenario: Release-worthy changes exist

- **WHEN** commits land on `main` that match the configured release-please strategy
- **THEN** CI opens or updates a single release PR that includes version/changelog updates

### Requirement: Merging the release PR creates a tag that triggers release build

When the `release-please` release PR is merged, the system MUST create a `vX.Y.Z` git tag and a corresponding GitHub Release so that the existing tag-triggered release pipeline runs.

#### Scenario: Release PR merged

- **WHEN** a maintainer merges the `release-please` release PR
- **THEN** a `vX.Y.Z` tag is created on the merge commit
- **THEN** the tag-triggered release workflow runs using that tag

### Requirement: Release version matches tag version

The version used by the release artifacts MUST match the `vX.Y.Z` tag version.

#### Scenario: Tag validation

- **WHEN** the tag-triggered release workflow runs for tag `vX.Y.Z`
- **THEN** the workflow verifies the Rust crate version used for publishing equals `X.Y.Z`
