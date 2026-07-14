## Purpose

Provide a canonical set of `.ito/` path builders in `ito-core` so other crates do not duplicate path construction.

## ADDED Requirements

### Requirement: Canonical path builder for `.ito/` root

The system SHALL provide a reusable API that returns the `.ito/` root for a workspace.

#### Scenario: Compute ito root

- **GIVEN** a workspace root directory
- **WHEN** requesting the ito root
- **THEN** the API returns `<root>/.ito`

### Requirement: Canonical path builders for key directories

The system SHALL provide reusable APIs for commonly used directories.

#### Scenario: Compute changes and modules directories

- **GIVEN** a ito root
- **WHEN** requesting changes and modules directories
- **THEN** the API returns `<ito>/changes` and `<ito>/modules`

### Requirement: Call sites avoid string-based path formatting

Call sites SHALL avoid `format!("{}/...", path.display())` for constructing filesystem paths.

#### Scenario: Spec path construction

- **GIVEN** a spec id
- **WHEN** constructing the spec file path
- **THEN** code uses `PathBuf::join` (or equivalent) rather than string formatting
