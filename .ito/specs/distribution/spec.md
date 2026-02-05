# Distribution Specification

## Purpose

Define the `distribution` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: GitHub Fetch for Adapter Files

The system SHALL support fetching adapter files from GitHub for released Ito versions.

#### Scenario: Fetch with version tag
- **GIVEN** a released Ito version with tag `v0.20.0`
- **WHEN** fetching adapter files
- **THEN** the system SHALL use URL `https://raw.githubusercontent.com/withakay/ito/v0.20.0/ito-skills/<path>`

#### Scenario: Fallback to main branch
- **GIVEN** a version tag that does not exist on GitHub
- **WHEN** fetching adapter files
- **THEN** the system SHALL fall back to `https://raw.githubusercontent.com/withakay/ito/main/ito-skills/<path>`

### Requirement: Per-User Cache

The system SHALL cache fetched adapter files per-user to avoid repeated downloads.

#### Scenario: Cache location
- **GIVEN** adapter files are fetched
- **WHEN** stored in cache
- **THEN** they SHALL be stored at `~/.config/ito/cache/ito-skills/<version>/<path>`

#### Scenario: Cache reuse
- **GIVEN** adapter files exist in cache for the current version
- **WHEN** installation is requested
- **THEN** the system SHALL use cached files without re-downloading

### Requirement: Tool-Specific Installation via ito init

The `ito init` command SHALL support installing tool-specific adapters.

#### Scenario: Install with tools flag
- **GIVEN** the user runs `ito init --tools opencode,claude,codex`
- **WHEN** the command executes
- **THEN** it SHALL fetch and install adapter files for the specified tools

#### Scenario: Default tool selection
- **GIVEN** the user runs `ito init` without `--tools` flag
- **WHEN** the command executes
- **THEN** it SHALL prompt for tool selection or use a sensible default

### Requirement: Adapter Update via ito update

The `ito update` command SHALL refresh adapter files for the current Ito version.

#### Scenario: Update refreshes adapters
- **GIVEN** adapter files are installed
- **WHEN** the user runs `ito update`
- **THEN** it SHALL refresh managed adapter files to match the current Ito version

### Requirement: Development Local Source Mode

The system SHALL support copying adapter files from a local `./ito-skills/` directory for development.

#### Scenario: Local source detection
- **GIVEN** a `./ito-skills/` directory exists in the repo root
- **WHEN** adapter installation is requested
- **THEN** the system SHALL copy files from the local directory (not fetch from GitHub)

#### Scenario: No symlinks
- **GIVEN** local source mode is active
- **WHEN** files are installed
- **THEN** the system SHALL copy files (not create symlinks)
