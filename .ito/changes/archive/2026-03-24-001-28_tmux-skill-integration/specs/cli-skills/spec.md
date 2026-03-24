<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Skills are managed via init/update (not CLI)

The system SHALL NOT expose skills management as part of the supported CLI UX. Skills installed by `ito init` / `ito update` MAY include both SKILL.md-only skills and skills with bundled asset subdirectories (e.g., `scripts/`); both forms SHALL be written to the output path preserving their directory structure.

#### Scenario: Skills are refreshed by init/update

- **WHEN** user runs `ito init` or `ito update`
- **THEN** the system installs/refreshes the core skill set for the configured harnesses
- **AND** any skill that bundles asset files (e.g., `scripts/`) has those files written alongside SKILL.md

#### Scenario: Skills commands remain callable but hidden

- **WHEN** user executes `ito skills <subcommand>`
- **THEN** the command executes successfully (for compatibility)
- **AND** prints a deprecation warning pointing to `ito init` and/or `ito update`
- **AND** the command is hidden from help and omitted from shell completions

#### Scenario: Skill with bundled scripts installs completely

- **WHEN** a skill asset directory contains both `SKILL.md` and a `scripts/` subdirectory
- **THEN** all files under `scripts/` are written to the output skill directory
- **AND** script files are written with executable permissions
<!-- ITO:END -->
