## MODIFIED Requirements

### Requirement: Update Is Idempotent and Non-Destructive

The system SHALL make `ito update` idempotent and non-destructive for user-owned files **and for user content outside the managed block of any installed Ito asset**, regardless of whether that asset was installed by the project-template installer or by the harness manifest installer.

- **Requirement ID**: cli-update:idempotent-non-destructive

#### Scenario: Repeated update is stable

- **GIVEN** a project has already been updated
- **WHEN** `ito update` is executed again
- **THEN** the resulting installed files SHALL be unchanged

#### Scenario: Update does not require force

- **GIVEN** a project contains pre-existing files
- **WHEN** `ito update` is executed
- **THEN** the update SHALL complete without requiring `--force`
- **AND** SHALL only change files that are Ito-managed or marker-managed
- **AND** SHALL NOT modify content sitting outside the `<!-- ITO:START -->` / `<!-- ITO:END -->` managed block of any installed asset

## ADDED Requirements

### Requirement: Harness Manifest Installs Are Marker-Scoped

When the harness manifest installer writes an asset that contains an Ito-managed block (`<!-- ITO:START -->` / `<!-- ITO:END -->`), it SHALL update only the managed block on update, exactly the same way the project-template installer (`write_one`) does. Content sitting outside the managed block SHALL be preserved across updates.

- **Requirement ID**: cli-update:harness-manifest-marker-scoped

#### Scenario: User edits to harness skill survive update

- **GIVEN** a project has installed a harness skill, e.g. `.opencode/skills/ito-feature/SKILL.md`
- **AND** the user has appended notes after the `<!-- ITO:END -->` marker
- **WHEN** `ito update` is executed
- **THEN** the managed block content SHALL be refreshed to match the current Ito templates
- **AND** the user's notes after the end marker SHALL be preserved byte-for-byte

#### Scenario: User edits to harness command survive update

- **GIVEN** a project has installed a harness command, e.g. `.opencode/commands/ito-loop.md`
- **AND** the user has appended a "Project notes" section below the `<!-- ITO:END -->` marker
- **WHEN** `ito update` is executed
- **THEN** the managed block SHALL be refreshed
- **AND** the appended section SHALL be preserved byte-for-byte

#### Scenario: Update refreshes the version stamp inside the managed block

- **GIVEN** an existing harness skill carries an older `<!--ITO:VERSION:0.9.0-->` stamp inside its managed block
- **WHEN** `ito update` is executed against a newer Ito CLI
- **THEN** the version stamp SHALL be updated to the current CLI version
- **AND** the rest of the managed block SHALL match the current template

#### Scenario: Non-markdown manifest assets remain wholesale-overwritten

- **GIVEN** a manifest entry is a non-markdown asset (e.g. a `.sh` helper or `.js` adapter glue)
- **WHEN** `ito update` is executed
- **THEN** the manifest installer MAY continue to write the asset wholesale
- **AND** the marker-scoped guarantee SHALL NOT apply to that asset

#### Scenario: Force overrides marker scoping

- **GIVEN** the user runs `ito init --force` against an existing project
- **WHEN** the harness manifest installer writes a managed-marker asset
- **THEN** the asset SHALL be rewritten wholesale (matching `--force` semantics elsewhere)

#### Scenario: Idempotent rerun

- **GIVEN** the harness manifest installer has just completed an update
- **WHEN** `ito update` is invoked a second time without intervening changes
- **THEN** no harness asset SHALL be modified
- **AND** the on-disk content SHALL be byte-identical to the post-first-update state
