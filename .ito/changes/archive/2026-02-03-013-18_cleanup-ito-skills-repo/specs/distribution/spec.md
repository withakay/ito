# Distribution Specification

## MODIFIED Requirements

### Requirement: Development Local Source Mode

The system SHALL support copying skill files from a local `./ito-skills/` directory for development.

#### Scenario: Local source detection
- **GIVEN** a `./ito-skills/` directory exists in the repo root containing `skills/<name>/SKILL.md`
- **WHEN** skill installation is requested
- **THEN** the system SHALL copy files from the local directory (not fetch from GitHub)

#### Scenario: No symlinks
- **GIVEN** local source mode is active
- **WHEN** files are installed
- **THEN** the system SHALL copy files (not create symlinks)
