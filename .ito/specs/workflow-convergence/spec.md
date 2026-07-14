<!-- ITO:START -->
# Workflow Convergence

## Purpose

This spec defines the current behavior and requirements for workflow convergence.

## Requirements

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

#### Scenario: Discovery depth gate chooses the least sufficient path

- **WHEN** Ito evaluates whether to enter domain discovery
- **THEN** it distinguishes routine direct work, lightweight terminology discovery, bounded-context discovery, and rigorous domain-grill mode
- **AND** it preserves the direct path for routine bounded work while auto-recommending rigorous grilling for high-impact ambiguity or explicit user opt-in

### Requirement: Domain-discovery review gate

Ito SHALL extend review guidance so reviewers can check whether proposals that used domain discovery preserved the agreed language, bounded-context framing, and event-storming conclusions.

- **Requirement ID**: `workflow-convergence:domain-discovery-review-gate`

#### Scenario: Review checks discovery handoff alignment

- **WHEN** a proposal includes a domain-discovery handoff summary
- **THEN** review guidance asks reviewers to compare proposal, spec, design, task language, evidence checked, and proposed documentation updates against the handoff
- **AND** it asks reviewers to flag missing context relationships, missing evidence, conflicting documentation updates, or unaddressed domain questions before implementation begins

### Requirement: Domain documentation remains change-scoped until approved

Ito SHALL ensure domain-documentation updates produced by discovery follow the same change-driven approval boundary as proposal, spec, design, and task artifacts.

- **Requirement ID**: `workflow-convergence:domain-docs-change-scope`

#### Scenario: Discovery proposes documentation updates before implementation

- **WHEN** discovery resolves a durable term, bounded context, or ADR-worthy decision during proposal work
- **THEN** Ito records the proposed documentation update in the active change package or change worktree
- **AND** canonical project documentation is not treated as accepted truth until the change is reviewed and approved

#### Scenario: Approved documentation updates are promoted through workflow guidance

- **WHEN** an approved change contains proposed `CONTEXT.md`, `CONTEXT-MAP.md`, or ADR updates
- **THEN** apply/archive/finish guidance includes the documentation promotion step
- **AND** rejected or abandoned changes do not update canonical domain docs
<!-- ITO:END -->
