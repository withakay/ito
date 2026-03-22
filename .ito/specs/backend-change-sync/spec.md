## MODIFIED Requirements

### Requirement: CLI exports backend changes as a zip archive

When backend mode is enabled, Ito SHALL provide backend change transfer commands that export backend change artifacts to a zip archive and import local change artifacts into backend-managed state.

The export command SHALL be `ito backend export`.

The import command SHALL be `ito backend import`.

#### Scenario: Export writes a zip bundle with active and archived changes

- **GIVEN** backend mode is enabled
- **AND** backend contains active and archived changes
- **WHEN** the user runs `ito backend export`
- **THEN** Ito writes a zip archive to the filesystem
- **AND** the archive includes both active and archived change artifacts

#### Scenario: Import writes backend state from local active and archived changes

- **GIVEN** backend mode is enabled
- **AND** local active and archived change artifacts exist
- **WHEN** the user runs `ito backend import`
- **THEN** Ito imports both lifecycle states into backend-managed storage
- **AND** reports imported, skipped, and failed counts

#### Scenario: Transfer commands in local mode are rejected

- **GIVEN** backend mode is disabled
- **WHEN** the user runs `ito backend export` or `ito backend import`
- **THEN** Ito exits with an actionable error indicating backend mode is required

### Requirement: Export uses a canonical archive layout and manifest

Exported zip archives MUST use a stable layout and include a machine-readable manifest.

#### Scenario: Archive includes canonical directories

- **WHEN** Ito creates a backend export archive
- **THEN** the zip contains `changes/active/` and `changes/archived/` roots
- **AND** each exported change appears under exactly one root based on lifecycle state

#### Scenario: Archive includes manifest metadata

- **WHEN** Ito creates a backend export archive
- **THEN** the zip contains `manifest.json`
- **AND** the manifest includes archive format version, export timestamp, and exported change counts

### Requirement: Export includes integrity metadata

Exported archives MUST include integrity metadata for all artifact files.

#### Scenario: Manifest includes per-file checksums

- **WHEN** Ito creates a backend export archive
- **THEN** `manifest.json` includes checksums for each exported artifact file

#### Scenario: Export reports integrity summary

- **WHEN** export completes
- **THEN** Ito prints the archive path and exported counts
- **AND** indicates manifest/integrity generation succeeded
