<!-- ITO:START -->
## ADDED Requirements

### Requirement: Domain-discovery entrypoint

Ito SHALL extend the canonical instruction-and-skill workflow with a domain-discovery lane that can run before proposal scaffolding for broad, ambiguous, or cross-context work.

- **Requirement ID**: `workflow-convergence:domain-discovery-entrypoint`

#### Scenario: Discovery lane precedes proposal lane

- **WHEN** a user request is still exploratory or the workflow detects substantial domain ambiguity
- **THEN** Ito routes the user toward domain discovery before proposal creation
- **AND** the proposal workflow consumes the discovery outputs rather than bypassing them

#### Scenario: Clear cross-context work still gets boundary discovery

- **WHEN** a request is clear enough to describe but spans multiple bounded contexts or ownership boundaries
- **THEN** Ito routes the user through at least bounded-context discovery before proposal creation
- **AND** the workflow records affected contexts, ownership, relationships, and translation boundaries before finalizing proposal scope

#### Scenario: Routine work keeps the direct path

- **WHEN** a request is already bounded, low-risk, and clear
- **THEN** Ito may continue directly into proposal creation or direct implementation
- **AND** the discovery lane remains optional rather than mandatory ceremony

#### Scenario: Technique fit is decided before artifact selection

- **WHEN** a request enters the domain-discovery lane
- **THEN** Ito asks whether ubiquitous language definition, bounded context mapping, event storming, or a smaller subset is appropriate
- **AND** the selected techniques inform the proposal artifacts without requiring a different schema solely because event storming was considered

### Requirement: Domain-discovery review gate

Ito SHALL extend review guidance so reviewers can check whether proposals that used domain discovery preserved the agreed language, bounded-context framing, and event-storming conclusions.

- **Requirement ID**: `workflow-convergence:domain-discovery-review-gate`

#### Scenario: Review checks discovery handoff alignment

- **WHEN** a proposal includes a domain-discovery handoff summary
- **THEN** review guidance asks reviewers to compare proposal, spec, design, and task language against the handoff
- **AND** it asks reviewers to flag missing context relationships or unaddressed domain questions before implementation begins

### Requirement: Domain documentation remains change-scoped until approved

Ito SHALL ensure domain-documentation updates produced by discovery follow the same change-driven approval boundary as proposal, spec, design, and task artifacts.

- **Requirement ID**: `workflow-convergence:domain-docs-change-scope`

#### Scenario: Discovery proposes documentation updates before implementation

- **WHEN** discovery resolves a durable term, bounded context, or ADR-worthy decision during proposal work
- **THEN** Ito records the proposed documentation update in the active change package or change worktree
- **AND** canonical project documentation is not treated as accepted truth until the change is reviewed and approved
<!-- ITO:END -->
