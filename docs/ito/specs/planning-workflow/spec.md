<!-- ITO:START -->
# Planning Workflow

## Purpose

This spec defines the current behavior and requirements for planning workflow.

## Requirements

### Requirement: Pre-proposal planning prompt
The system SHALL provide pre-proposal planning guidance through the retained `ito-proposal` lifecycle skill and authoritative proposal instruction artifacts. The guidance MUST ask clarifying questions, frame the output as a precursor to one or more change proposals, and direct any durable planning artifact into `.ito/planning/` without requiring an `ito-plan` skill.

#### Scenario: Agent enters pre-proposal planning
- **WHEN** a user brings an idea or rough feature request to `ito-proposal`
- **THEN** the retained skill uses the planning section of its authoritative guidance
- **AND** asks questions that shape scope, goals, constraints, and likely proposal boundaries
- **AND** treats the session as planning work rather than immediate proposal scaffolding

#### Scenario: Durable planning output is retained
- **WHEN** exploratory planning produces reusable context
- **THEN** the guidance directs the agent to create or update a topic-specific document under `.ito/planning/`
- **AND** no standalone planning skill or prompt wrapper is required

### Requirement: Planning and research artifact locations

The system SHALL distinguish planning synthesis from research evidence. Planning documents MUST live under `.ito/planning/`, while supporting research documents MUST live under `.ito/research/`.

- **Requirement ID**: `planning-workflow:planning-and-research-locations`

#### Scenario: Planning identifies deeper investigation work

- **WHEN** the planning workflow uncovers questions that require deeper technical or product investigation
- **THEN** the system directs the agent to place those findings under `.ito/research/`
- **AND** instructs the agent to reference relevant research outputs from the plan in `.ito/planning/`
- **AND** preserves the distinction between exploratory evidence and proposal-oriented synthesis
<!-- ITO:END -->
