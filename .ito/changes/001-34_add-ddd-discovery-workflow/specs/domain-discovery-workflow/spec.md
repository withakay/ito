<!-- ITO:START -->
## ADDED Requirements

### Requirement: DDD discovery bundle

The system SHALL provide a DDD-oriented discovery bundle for ambiguous, architectural, or cross-context work before proposal scaffolding. The bundle MUST extract ubiquitous language, bounded contexts, commands, domain events, policies, and proposal-relevant open questions.

- **Requirement ID**: `domain-discovery-workflow:ddd-discovery-bundle`

#### Scenario: Planning lane enters domain discovery mode

- **WHEN** a user starts planning for a broad or ambiguous change
- **THEN** the workflow asks discovery questions about domain terms, responsibilities, actors, commands, events, policies, and constraints
- **AND** it records the resulting model in planning artifacts rather than jumping straight to proposal prose

### Requirement: Proposal handoff summary

The discovery workflow SHALL produce a proposal-ready handoff summary that carries forward the canonical terms, affected bounded contexts, candidate capabilities, commands, domain events, policies, and unresolved questions.

- **Requirement ID**: `domain-discovery-workflow:proposal-handoff-summary`

#### Scenario: Discovery outputs feed proposal creation

- **WHEN** a discovery session is ready to become a change proposal
- **THEN** the workflow emits a compact handoff summary for proposal scaffolding
- **AND** the summary names the canonical vocabulary and affected contexts
- **AND** the proposal author does not need to rediscover those concepts from scratch

### Requirement: Context map distinguishes module and capability

The discovery workflow SHALL treat bounded contexts as domain-model boundaries distinct from Ito modules and capabilities.

- **Requirement ID**: `domain-discovery-workflow:context-map-distinguishes-module-and-capability`

#### Scenario: Cross-context work does not collapse concepts

- **WHEN** a request spans more than one bounded context
- **THEN** the workflow records the affected contexts and their relationship
- **AND** it does not treat a module id or capability name as equivalent to a bounded context unless explicitly justified
<!-- ITO:END -->
