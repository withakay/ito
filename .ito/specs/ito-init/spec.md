## MODIFIED Requirements

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

#### Scenario: Worktree wizard runs before template installation

- **GIVEN** the user runs `ito init` interactively
- **WHEN** the worktree wizard completes
- **THEN** the worktree configuration SHALL be resolved and available before `install_default_templates()` is called
- **AND** the resolved config SHALL be passed to the template installer for rendering AGENTS.md and skills
