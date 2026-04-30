<!-- ITO:START -->
## ADDED Requirements

### Requirement: DDD discovery bundle

The system SHALL provide a DDD-oriented discovery bundle for ambiguous, architectural, or cross-context work before proposal scaffolding. The bundle MUST extract ubiquitous language, bounded contexts, commands, policies, invariants, and proposal-relevant open questions, and it MUST include domain events when event storming is useful for the request.

- **Requirement ID**: `domain-discovery-workflow:ddd-discovery-bundle`

#### Scenario: Planning lane enters domain discovery mode

- **WHEN** a user starts planning for a broad or ambiguous change
- **THEN** the workflow asks discovery questions about domain terms, responsibilities, actors, commands, events, policies, and constraints
- **AND** it records the resulting model in planning artifacts rather than jumping straight to proposal prose

### Requirement: Ubiquitous language glossary

The discovery workflow SHALL produce a glossary that names canonical domain terms, short definitions, rejected aliases, overloaded terms, and unresolved vocabulary questions.

- **Requirement ID**: `domain-discovery-workflow:ubiquitous-language-glossary`

#### Scenario: Vocabulary ambiguity is resolved before proposal drafting

- **WHEN** a user describes the same concept with multiple names during discovery
- **THEN** the workflow asks which term is canonical or marks the vocabulary question as unresolved
- **AND** proposal, spec, and task guidance can reuse the canonical term instead of inventing a new synonym

### Requirement: Bounded context map

The discovery workflow SHALL produce a bounded context map that identifies context names, responsibilities, owned language, upstream/downstream relationships, and translation boundaries when more than one model is involved.

- **Requirement ID**: `domain-discovery-workflow:bounded-context-map`

#### Scenario: Context ownership is explicit

- **WHEN** a change crosses multiple domain models or teams' responsibilities
- **THEN** the workflow records the affected bounded contexts and what each context owns
- **AND** it describes the relationship between those contexts before proposal scope is finalized

### Requirement: Event storming technique fit

The discovery workflow SHALL treat event storming as an optional DDD technique, not a mandatory artifact. It MUST recommend event storming when behavior is temporal, event-heavy, policy-driven, or unclear from static requirements alone.

- **Requirement ID**: `domain-discovery-workflow:event-storming-technique-fit`

#### Scenario: Event storming is used when behavior needs sequencing

- **WHEN** a request depends on ordering, domain events, policies, or cross-context reactions
- **THEN** the workflow asks for commands, domain events, actors, policies, aggregates, read models, and invariants
- **AND** the resulting event-storming snapshot feeds proposal and spec drafting

#### Scenario: Event storming is skipped for simple bounded work

- **WHEN** a request is already clear, local, and not event- or policy-heavy
- **THEN** the workflow can proceed with the glossary and context map only
- **AND** it does not require an event-storming artifact solely because the DDD lane was used

### Requirement: Proposal handoff summary

The discovery workflow SHALL produce a proposal-ready handoff summary that carries forward the canonical terms, affected bounded contexts, candidate capabilities, commands, domain events when captured, policies, invariants, and unresolved questions.

- **Requirement ID**: `domain-discovery-workflow:proposal-handoff-summary`

#### Scenario: Discovery outputs feed proposal creation

- **WHEN** a discovery session is ready to become a change proposal
- **THEN** the workflow emits a compact handoff summary for proposal scaffolding
- **AND** the summary names the canonical vocabulary, affected contexts, technique-fit decision, and unresolved questions
- **AND** the proposal author does not need to rediscover those concepts from scratch

### Requirement: Context map distinguishes module and capability

The discovery workflow SHALL treat bounded contexts as domain-model boundaries distinct from Ito modules and capabilities.

- **Requirement ID**: `domain-discovery-workflow:context-map-distinguishes-module-and-capability`

#### Scenario: Cross-context work does not collapse concepts

- **WHEN** a request spans more than one bounded context
- **THEN** the workflow records the affected contexts and their relationship
- **AND** it does not treat a module id or capability name as equivalent to a bounded context unless explicitly justified
<!-- ITO:END -->
