# Cli Update

## Purpose

This spec defines the current behavior and requirements for cli update.

## Requirements

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

### Requirement: Update prunes retired managed skill surfaces safely
`ito update` and `ito init --upgrade` SHALL compare installed Ito-managed skill and command paths with the canonical lifecycle inventory and remove obsolete managed assets only when ownership and user-content checks permit safe deletion.

#### Scenario: Managed-only retired asset is removed
- **GIVEN** a retired skill or command contains only Ito-managed content at a known legacy path
- **WHEN** update cleanup runs
- **THEN** the obsolete file tree is removed
- **AND** empty Ito-managed parent directories are pruned

#### Scenario: Broken managed symlink is removed
- **GIVEN** a known retired asset path is a broken symlink
- **WHEN** update cleanup runs
- **THEN** symlink-aware metadata identifies and removes the obsolete link

#### Scenario: User content is preserved
- **GIVEN** a retired managed Markdown asset contains content outside its Ito-managed block
- **WHEN** update cleanup runs
- **THEN** user content is not deleted
- **AND** the command reports the path and retained lifecycle replacement

#### Scenario: Repeated update is stable
- **GIVEN** obsolete managed assets have been removed and the seven retained skills are current
- **WHEN** update runs again
- **THEN** no managed skill or command file changes

#### Scenario: Cleanup audits every selected harness
- **GIVEN** retired managed surfaces exist in one or more configured harness roots
- **WHEN** update cleanup runs
- **THEN** it audits every selected harness before writing retained assets
- **AND** applies the same ownership proof to each harness

#### Scenario: Cleanup reports every decision
- **WHEN** update removes a proven managed surface or preserves an ambiguous surface
- **THEN** it reports the path and lifecycle replacement
- **AND** deliberate removals such as tmux report that no Ito replacement exists

#### Scenario: Explicit update invocation is the deletion gate
- **WHEN** the user runs `ito update`, `ito init --upgrade`, or a forceful refresh
- **THEN** cleanup may remove only byte- or shell-fingerprint-proven Ito assets
- **AND** never requires `--force` to preserve ambiguous or user-owned content
<!-- ITO:END -->
