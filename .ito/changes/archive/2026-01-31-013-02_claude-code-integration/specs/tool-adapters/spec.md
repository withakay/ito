## ADDED Requirements

### Requirement: Claude Code Skill Integration

The system SHALL provide Claude Code skill files that integrate Ito workflows into the Claude Code agent environment.

#### Scenario: Skill file delegates to CLI
- **GIVEN** the skill file `.claude/skills/ito-workflow.md` is installed
- **WHEN** an agent loads the skill
- **THEN** the skill SHALL point to `ito agent instruction <artifact>` for workflow bodies
- **AND** the skill SHALL NOT embed long policy text

#### Scenario: Project templates preferred over hooks
- **GIVEN** a Ito-enabled project
- **WHEN** Claude Code starts a session
- **THEN** the system SHALL prefer project templates (`AGENTS.md`/`CLAUDE.md`) over hooks for workflow injection

### Requirement: Optional SessionStart Hook Shim

The system SHALL provide an optional minimal `SessionStart` hook shim for cases where project files are not loaded.

#### Scenario: Hook shim is minimal
- **GIVEN** the optional hook shim is installed
- **WHEN** a session starts without project files loaded
- **THEN** the shim SHALL only print a pointer to an instruction artifact
- **AND** the shim SHALL NOT embed workflow content
