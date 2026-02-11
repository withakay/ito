## ADDED Requirements

### Requirement: CI/CD workflows use self-hosted runner group

All GitHub Actions workflows in the project SHALL use the `withakay-selfhost` runner group for jobs that do not require a specific operating system runner.

#### Scenario: CI workflow uses self-hosted runners

- **GIVEN** the `ci.yml` workflow
- **WHEN** jobs that currently use `ubuntu-latest` execute
- **THEN** they SHALL use `runs-on: group: withakay-selfhost` instead

#### Scenario: Release-plz workflow uses self-hosted runners

- **GIVEN** the `release-plz.yml` workflow
- **WHEN** the release and PR jobs execute
- **THEN** they SHALL use `runs-on: group: withakay-selfhost`

#### Scenario: Homebrew update workflow uses self-hosted runners

- **GIVEN** the `update-homebrew.yml` workflow
- **WHEN** the update-formula job executes
- **THEN** it SHALL use `runs-on: group: withakay-selfhost`

#### Scenario: Polish release notes workflow uses self-hosted runners

- **GIVEN** the `polish-release-notes.yml` workflow
- **WHEN** the polish job executes
- **THEN** it SHALL use `runs-on: group: withakay-selfhost`

#### Scenario: Claude code review workflow uses self-hosted runners

- **GIVEN** the `claude-code-review.yml` workflow
- **WHEN** the review job executes
- **THEN** it SHALL use `runs-on: group: withakay-selfhost`

#### Scenario: OS-specific matrix jobs retain appropriate runners

- **GIVEN** workflow jobs that require specific OS runners (e.g., macOS builds, Windows builds)
- **WHEN** those jobs execute
- **THEN** they SHALL continue using the appropriate OS-specific runner (e.g., `macos-14`, `windows-latest`)
- **AND** Linux matrix entries MAY use the self-hosted runner group if the runners support the required environment

### Requirement: Vestigial Release Please references are removed

All references to the non-existent "Release Please" workflow SHALL be removed from CI configuration and build tooling.

#### Scenario: release.yml workflow_run trigger is updated

- **GIVEN** the `release.yml` workflow
- **WHEN** examining its triggers
- **THEN** it SHALL NOT contain a `workflow_run` trigger referencing "Release Please"

#### Scenario: Makefile release target is updated

- **GIVEN** the `Makefile`
- **WHEN** examining the `release` target
- **THEN** it SHALL NOT reference `release-please.yml`
