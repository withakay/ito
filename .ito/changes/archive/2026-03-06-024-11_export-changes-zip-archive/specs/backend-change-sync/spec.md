## ADDED Requirements

### Requirement: CLI exports backend changes as a zip archive

When backend mode is enabled, Ito SHALL provide a command that exports backend change artifacts to a zip archive.

The command SHALL be `ito backend export`.

#### Scenario: Export writes a zip bundle with active and archived changes

- **GIVEN** backend mode is enabled
- **AND** backend contains active and archived changes
- **WHEN** the user runs `ito backend export`
- **THEN** Ito writes a zip archive to the filesystem
- **AND** the archive includes both active and archived change artifacts

#### Scenario: Export in local mode is rejected

- **GIVEN** backend mode is disabled
- **WHEN** the user runs `ito backend export`
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
