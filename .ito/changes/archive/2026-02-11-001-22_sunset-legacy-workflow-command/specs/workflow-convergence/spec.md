## ADDED Requirements

### Requirement: Unified workflow model

Ito SHALL provide one canonical workflow model centered on change artifacts, agent instructions, and skills, rather than parallel orchestration systems.

#### Scenario: Canonical workflow entry points

- **WHEN** users look for workflow guidance in CLI output or project documentation
- **THEN** the recommended entry points SHALL be `ito agent instruction <artifact>` and the corresponding Ito skills
- **AND** the guidance SHALL avoid presenting retired standalone workflow orchestration as an equal alternative

#### Scenario: Concept migration from legacy workflows

- **WHEN** legacy workflow concepts (research, execute, review structure) are retained
- **THEN** they SHALL be mapped into proposal/apply/review instruction artifacts and skills
- **AND** the mapping SHALL preserve actionable sequencing and review checkpoints where appropriate

#### Scenario: Legacy workflow namespace remains no-op

- **WHEN** users invoke `ito workflow` commands after convergence
- **THEN** the commands SHALL behave as no-ops
- **AND** workflow execution behavior SHALL remain in instruction artifacts and skills
