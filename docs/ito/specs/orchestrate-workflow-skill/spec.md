<!-- ITO:START -->
## ADDED Requirements

### Requirement: Workflow skill loaded by convention

The system SHALL treat the presence of a skill named `ito-orchestrator-workflow` in the project's skill directory as the authoritative workflow description for the orchestrator. The orchestrator SHALL load this skill automatically without any explicit configuration reference.

- **Requirement ID**: orchestrate-workflow-skill:convention-load

#### Scenario: Skill present — loaded automatically

- **WHEN** `ito-orchestrator-workflow` exists in the project's skill directory
- **THEN** the orchestrator instruction document includes its content as workflow guidance
- **AND** no configuration entry is required to activate it

#### Scenario: Skill absent — orchestrator proceeds with base guidance only

- **WHEN** `ito-orchestrator-workflow` does not exist in the project's skill directory
- **THEN** the orchestrator renders without workflow skill content and continues normally
- **AND** no error or warning is emitted

### Requirement: Workflow skill is living documentation

The generated `ito-orchestrator-workflow` skill SHALL be a human-readable markdown document that the user can evolve over time. It SHALL reference other skills by name, describe gate preferences, agent role guidance, and project-specific conventions. It SHALL NOT be a binary or locked configuration file.

- **Requirement ID**: orchestrate-workflow-skill:living-doc

#### Scenario: User edits workflow skill

- **WHEN** a user edits `ito-orchestrator-workflow/SKILL.md` to add a new gate preference
- **THEN** the next orchestrator run reflects that preference via the loaded skill content
- **AND** no CLI command or config reload is required

#### Scenario: Workflow skill references other skills by name

- **WHEN** the workflow skill markdown contains a reference to `rust-style`
- **THEN** the orchestrator instruction renderer includes the reference as advisory guidance for the orchestrator agent to load that skill
<!-- ITO:END -->
