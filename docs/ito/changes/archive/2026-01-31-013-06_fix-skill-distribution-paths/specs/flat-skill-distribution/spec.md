## Purpose

Skills from `ito-skills/skills/` MUST be distributed with a flat directory structure directly under the harness skills folder, with a `ito-` prefix on each skill name. This ensures compliance with the agentskills.io specification which prohibits nested subfolders under the skills directory.

## ADDED Requirements

### Requirement: Skills use flat directory structure

The distribution system SHALL place skills directly under the harness skills directory without any intermediate subfolder. Skills MUST NOT be placed in a `ito-skills/` or any other subfolder.

#### Scenario: OpenCode skill path structure

- **WHEN** `ito dist install` is run for OpenCode
- **THEN** skills are placed at `~/.config/opencode/skills/ito-<skill-name>/` (not `~/.config/opencode/skills/ito-skills/<skill-name>/`)

#### Scenario: No subfolder nesting

- **WHEN** the distribution creates skill directories
- **THEN** there SHALL be no intermediate directories between the `skills/` folder and the individual skill folders

### Requirement: Skills have ito- prefix

All ito-skills SHALL be prefixed with `ito-` to namespace them and avoid conflicts with user skills.

#### Scenario: Skill name transformation

- **WHEN** a skill named `brainstorming` is distributed
- **THEN** it is placed in a folder named `ito-brainstorming`

#### Scenario: All skills are prefixed

- **WHEN** `ito dist install` completes
- **THEN** every skill from `ito-skills/skills/` has the `ito-` prefix applied

### Requirement: Skills are copied not symlinked

Skills MUST be copied to the target location. Symlinks are explicitly forbidden.

#### Scenario: File copy operation

- **WHEN** skills are distributed
- **THEN** the files are actual copies, not symbolic links

#### Scenario: No symlink references in docs

- **WHEN** the distribution documentation is read
- **THEN** there are no instructions for creating symlinks

### Requirement: Embedded templates use correct paths

The embedded template assets in ito-rs MUST use the flat, prefixed path structure.

#### Scenario: Template path format

- **WHEN** embedded templates reference skill paths
- **THEN** the path format is `.opencode/skills/ito-<skill-name>/` (not `.opencode/skills/ito-skills/<skill-name>/`)
