<!-- ITO:START -->
## ADDED Requirements

### Requirement: Cleanup skill definition

The system SHALL provide an `ito-cleanup` skill installable via `ito init` that instructs agents to run `ito agent instruction cleanup` and follow the returned instructions.

- **Requirement ID**: cleanup-skill:skill-definition

#### Scenario: Skill is installed by ito init

- **WHEN** `ito init` is run with any harness tool configured
- **THEN** the `ito-cleanup` skill SHALL be installed to the appropriate skill directory for each configured harness (e.g., `.opencode/skills/ito-cleanup/SKILL.md`, `.claude/skills/ito-cleanup/SKILL.md`)

#### Scenario: Skill instructs agent to run cleanup

- **WHEN** an agent loads the `ito-cleanup` skill
- **THEN** the skill SHALL instruct the agent to run `ito agent instruction cleanup`
- **AND** the skill SHALL instruct the agent to follow the returned instructions step-by-step
- **AND** the skill SHALL instruct the agent to present findings to the user before taking destructive actions

### Requirement: Skill triggers on cleanup-related requests

The skill description SHALL match when users ask about cleaning up, migrating, or removing legacy Ito files.

- **Requirement ID**: cleanup-skill:trigger-matching

#### Scenario: Skill matches cleanup intent

- **WHEN** a user asks to "clean up old ito files" or "remove legacy skills" or "migrate from old ito version"
- **THEN** the skill's description SHALL be specific enough for the agent to select it as the appropriate skill to load
<!-- ITO:END -->
