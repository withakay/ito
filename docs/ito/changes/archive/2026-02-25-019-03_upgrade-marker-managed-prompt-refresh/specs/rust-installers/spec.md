## MODIFIED Requirements

### Requirement: Deterministic Init/Update Merge Policy

The system SHALL apply a deterministic, test-covered merge/overwrite policy when installing templates via `ito init --update`, `ito init --upgrade`, and `ito update`.

#### Scenario: Update preserves user-owned files

- **GIVEN** a project has user edits in explicitly user-owned files (e.g., `.ito/project.md`, `.ito/config.json`)
- **WHEN** `ito update` is executed
- **THEN** the installer SHALL preserve the user edits

#### Scenario: Update refreshes Ito-managed adapter assets

- **GIVEN** a project has Ito-managed harness assets installed under `.opencode/`, `.claude/`, `.github/`, or `.codex/`
- **WHEN** `ito update` is executed
- **THEN** the installer SHALL refresh those assets to match the embedded templates

#### Scenario: Marker-managed files are merged

- **GIVEN** a file contains Ito markers
- **WHEN** `ito update` is executed
- **THEN** the installer SHALL update the managed block content
- **AND** preserve user content outside the managed block

#### Scenario: Upgrade refreshes prompt/template managed blocks only

- **GIVEN** a prompt/template file contains `<!-- ITO:START -->` and `<!-- ITO:END -->` markers
- **WHEN** `ito init --upgrade` is executed
- **THEN** only content between those markers SHALL be replaced from embedded templates
- **AND** all content outside those markers SHALL be preserved exactly

#### Scenario: Missing markers fail safe during upgrade

- **GIVEN** a prompt/template file is expected to be marker-managed but no longer contains valid Ito markers
- **WHEN** `ito init --upgrade` is executed
- **THEN** the installer SHALL leave the file unchanged
- **AND** SHALL emit actionable guidance describing how to restore markers or manually reconcile the file
