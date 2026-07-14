## ADDED Requirements

### Requirement: release-plz config lives at the git repository root

The `release-plz.toml` configuration file MUST be located at the git repository root (not inside the `ito-rs/` subdirectory) so that release-plz can discover the `.git` directory when it clones the repo into a temporary directory.

#### Scenario: Config references subdirectory workspace manifest

- **GIVEN** `release-plz.toml` is at the git repo root
- **WHEN** release-plz reads the configuration
- **THEN** it SHALL find the workspace via `manifest_path = "ito-rs/Cargo.toml"` in the `[workspace]` section

#### Scenario: release-plz can open the git repository in CI

- **GIVEN** the release-plz GitHub Action clones the repo to a temp directory
- **WHEN** release-plz attempts to open the git repository
- **THEN** it SHALL succeed because `release-plz.toml` is co-located with `.git`

#### Scenario: GitHub Action config and manifest_path inputs match

- **GIVEN** `release-plz.toml` is at the repo root with `manifest_path = "ito-rs/Cargo.toml"`
- **WHEN** the `release-plz.yml` workflow invokes the action
- **THEN** the action inputs SHALL either omit `manifest_path` and `config` (auto-discovery) or reference the repo-root paths correctly

### Requirement: release-plz creates release PRs on push to main

The release-plz `release-pr` command MUST successfully create or update a release PR when new commits are pushed to `main`.

#### Scenario: Release PR is created after conventional commits

- **GIVEN** new commits following conventional commit format are pushed to `main`
- **WHEN** the `release-plz release-pr` command runs in CI
- **THEN** it SHALL create a PR with version bumps and changelog updates
- **AND** the workflow run SHALL succeed (exit code 0)

#### Scenario: Release PR is updated on subsequent pushes

- **GIVEN** a release PR already exists
- **WHEN** additional commits are pushed to `main`
- **THEN** release-plz SHALL update the existing PR with new version bumps and changelog entries

### Requirement: release-plz creates releases and tags when release PR is merged

The release-plz `release` command MUST create git tags and GitHub releases when a release PR is merged.

#### Scenario: Git tag and GitHub release are created

- **GIVEN** a release PR created by release-plz is merged to `main`
- **WHEN** the push-to-main event triggers the release-plz workflow
- **THEN** release-plz SHALL create a git tag matching `v{{ version }}`
- **AND** it SHALL create a GitHub release with the changelog content

### Requirement: Workspace crates are published to crates.io

The release pipeline SHALL publish all public library crates in the Ito workspace to crates.io when a new version is released.

#### Scenario: Library crates are published in dependency order

- **WHEN** release-plz creates a release
- **THEN** it SHALL publish crates to crates.io in dependency order: `ito-common` → `ito-config` → `ito-domain` → `ito-templates` → `ito-logging` → `ito-core` → `ito-cli`

#### Scenario: Test-support crate is excluded from publishing

- **GIVEN** the `ito-test-support` crate has `publish = false` in its `Cargo.toml`
- **WHEN** release-plz evaluates crates for publishing
- **THEN** it SHALL skip `ito-test-support`

#### Scenario: Web crate is excluded from publishing

- **GIVEN** the `ito-web` crate has `publish = false` in its `Cargo.toml`
- **WHEN** release-plz evaluates crates for publishing
- **THEN** it SHALL skip `ito-web`

### Requirement: All published crates have valid crates.io metadata

Every crate published to crates.io SHALL have the required metadata fields: `name`, `version`, `description`, `license`, and `repository`.

#### Scenario: Metadata validation before publish

- **GIVEN** a crate is marked for publishing
- **WHEN** `cargo publish --dry-run` is executed
- **THEN** it SHALL succeed without metadata errors

### Requirement: Crate package names avoid registry conflicts

Each published crate's package name MUST be unique on crates.io and not conflict with existing unrelated packages.

#### Scenario: ito-cli name conflict resolution

- **GIVEN** an unrelated `ito-cli` package already exists on crates.io
- **WHEN** publishing the Ito CLI crate
- **THEN** the crate MUST use a non-conflicting package name (e.g., `ito` or an alternative) or the existing name must be secured

### Requirement: CARGO_REGISTRY_TOKEN is configured for publishing

The CI release workflow MUST use a valid `CARGO_REGISTRY_TOKEN` secret for crates.io authentication.

#### Scenario: Token is available in release-plz workflow

- **GIVEN** the `release-plz.yml` workflow runs
- **WHEN** the `release` command executes with publishing enabled
- **THEN** the `CARGO_REGISTRY_TOKEN` environment variable SHALL contain a valid crates.io API token

### Requirement: release-plz configuration enables crates.io publishing

The `release-plz.toml` SHALL be configured to publish crates to crates.io instead of operating in git-only mode.

#### Scenario: Workspace publish is enabled

- **GIVEN** the `release-plz.toml` workspace section
- **WHEN** release-plz evaluates the configuration
- **THEN** `publish` SHALL NOT be `false` and `git_only` SHALL NOT be `true`

#### Scenario: Per-package publish control

- **GIVEN** crates that should not be published (e.g., `ito-test-support`, `ito-web`)
- **WHEN** release-plz evaluates per-package configuration
- **THEN** those packages SHALL have `publish = false` in their `Cargo.toml` or `release = false` in `release-plz.toml`
