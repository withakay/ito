# Cli Skills Specification

## Purpose

Define the `cli-skills` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Skills are managed via init/update (not CLI)

The system SHALL NOT expose skills management as part of the supported CLI UX.

#### Scenario: Skills are refreshed by init/update

- **WHEN** user runs `ito init` or `ito update`
- **THEN** the system installs/refreshes the core skill set for the configured harnesses

#### Scenario: Skills commands remain callable but hidden

- **WHEN** user executes `ito skills <subcommand>`
- **THEN** the command executes successfully (for compatibility)
- **AND** prints a deprecation warning pointing to `ito init` and/or `ito update`
- **AND** the command is hidden from help and omitted from shell completions
