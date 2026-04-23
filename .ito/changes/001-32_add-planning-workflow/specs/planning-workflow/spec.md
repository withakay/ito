<!-- ITO:START -->
## ADDED Requirements

### Requirement: Pre-proposal planning prompt

The system SHALL provide an `ito-plan` planning prompt that guides exploratory planning before proposal creation. The prompt MUST ask clarifying questions, frame the output as a precursor to one or more change proposals, and direct the resulting planning artifact into `.ito/planning/`.

- **Requirement ID**: `planning-workflow:pre-proposal-planning`

#### Scenario: Agent enters planning mode

- **WHEN** a user invokes `/ito-plan` with an idea or rough feature request
- **THEN** the system loads the dedicated planning guidance
- **AND** asks clarifying questions that help shape scope, goals, constraints, and likely proposal boundaries
- **AND** treats the session as planning work rather than immediate proposal scaffolding
- **AND** instructs the agent to create or update a markdown plan under `.ito/planning/`

### Requirement: Planning and research artifact locations

The system SHALL distinguish planning synthesis from research evidence. Planning documents MUST live under `.ito/planning/`, while supporting research documents MUST live under `.ito/research/`.

- **Requirement ID**: `planning-workflow:planning-and-research-locations`

#### Scenario: Planning identifies deeper investigation work

- **WHEN** the planning workflow uncovers questions that require deeper technical or product investigation
- **THEN** the system directs the agent to place those findings under `.ito/research/`
- **AND** instructs the agent to reference relevant research outputs from the plan in `.ito/planning/`
- **AND** preserves the distinction between exploratory evidence and proposal-oriented synthesis
<!-- ITO:END -->
