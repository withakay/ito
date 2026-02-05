## Purpose

Skills from `ito-skills/skills/` MUST be distributed to all three supported AI coding assistant harnesses: OpenCode, Claude, and Codex. Currently skills are only distributed to OpenCode.

## ADDED Requirements

### Requirement: Skills distributed to OpenCode

The distribution system SHALL copy skills to the OpenCode configuration directory at `~/.config/opencode/skills/`.

#### Scenario: OpenCode skill installation

- **WHEN** `ito dist install` is run
- **THEN** all skills from `ito-skills/skills/` are copied to `~/.config/opencode/skills/ito-<skill-name>/`

### Requirement: Skills distributed to Claude

The distribution system SHALL copy skills to the Claude configuration directory at `~/.claude/skills/`.

#### Scenario: Claude skill installation

- **WHEN** `ito dist install` is run
- **THEN** all skills from `ito-skills/skills/` are copied to `~/.claude/skills/ito-<skill-name>/`

### Requirement: Skills distributed to Codex

The distribution system SHALL copy skills to the Codex configuration directory at `~/.codex/skills/`.

#### Scenario: Codex skill installation

- **WHEN** `ito dist install` is run
- **THEN** all skills from `ito-skills/skills/` are copied to `~/.codex/skills/ito-<skill-name>/`

### Requirement: Consistent skill content across harnesses

The same skill content SHALL be distributed to all harnesses. Each harness receives identical skill files.

#### Scenario: Skill file consistency

- **WHEN** the `ito-brainstorming` skill is distributed
- **THEN** the SKILL.md and supporting files are identical in `~/.config/opencode/skills/ito-brainstorming/`, `~/.claude/skills/ito-brainstorming/`, and `~/.codex/skills/ito-brainstorming/`

### Requirement: Harness-specific skill naming conventions

Each harness may have different file naming conventions. The distribution system SHALL use the appropriate naming for each harness.

#### Scenario: OpenCode uses SKILL.md

- **WHEN** skills are distributed to OpenCode
- **THEN** the main skill file is named `SKILL.md` (or the appropriate OpenCode convention)

#### Scenario: Claude uses appropriate naming

- **WHEN** skills are distributed to Claude
- **THEN** the skill files follow Claude's expected naming convention

#### Scenario: Codex uses appropriate naming

- **WHEN** skills are distributed to Codex
- **THEN** the skill files follow Codex's expected naming convention
