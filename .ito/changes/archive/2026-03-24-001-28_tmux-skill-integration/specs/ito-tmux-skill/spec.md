<!-- ITO:START -->
## ADDED Requirements

### Requirement: Tmux skill is distributed with Ito

The system SHALL embed the `tmux` skill (SKILL.md and companion scripts) in the `ito-templates` asset tree so that `ito init` and `ito update` install it alongside all other Ito-managed skills.

#### Scenario: Tmux skill installed on init

- **WHEN** a user runs `ito init` in a project
- **THEN** the `tmux` skill directory is written to the configured skills output path (e.g., `.opencode/skills/tmux/`)
- **AND** the directory contains `SKILL.md` and a `scripts/` subdirectory with helper scripts

#### Scenario: Tmux skill refreshed on update

- **WHEN** a user runs `ito update`
- **THEN** the `tmux` skill files are refreshed to the latest embedded version
- **AND** existing skill content is overwritten with the embedded asset

#### Scenario: Skill frontmatter identifies upstream

- **WHEN** the installed `SKILL.md` is read
- **THEN** the frontmatter SHALL contain a `name` field set to `tmux`
- **AND** a `description` field describing its purpose
- **AND** a `metadata.upstream` field referencing the original source

### Requirement: Tmux skill includes helper scripts

The installed tmux skill SHALL include companion Bash helper scripts that agents can reference in their instructions.

#### Scenario: wait-for-text helper is present

- **WHEN** the tmux skill is installed
- **THEN** `scripts/wait-for-text.sh` SHALL be present and executable
- **AND** the script SHALL poll a tmux pane for a regex pattern with a configurable timeout

#### Scenario: find-sessions helper is present

- **WHEN** the tmux skill is installed
- **THEN** `scripts/find-sessions.sh` SHALL be present and executable
- **AND** the script SHALL enumerate active tmux sessions on a given socket path
<!-- ITO:END -->
