<!-- ITO:START -->
## ADDED Requirements

### Requirement: Deterministic Init/Update Merge Policy

The system SHALL apply a deterministic, test-covered merge/overwrite policy when installing templates via `ito init` and `ito update`.

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
<!-- ITO:END -->
