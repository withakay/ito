<!-- ITO:START -->
## ADDED Requirements

### Requirement: Domain discovery artifacts are schema-addressable

Ito MUST allow workflow schemas to define reusable domain-discovery artifacts or artifact sections that capture ubiquitous language, bounded contexts, commands, domain events, policies, and handoff summaries.

- **Requirement ID**: `ito-schemas:domain-discovery-artifacts`

#### Scenario: Schema declares discovery artifacts

- **GIVEN** a workflow schema defines a discovery artifact or template section
- **WHEN** Ito loads the schema
- **THEN** the discovery artifact is treated as part of the schema's artifact vocabulary
- **AND** later instruction rendering can reference it as dependency context

### Requirement: Cross-schema discovery vocabulary

Built-in schemas that support proposal-oriented work SHALL share a compatible discovery vocabulary so domain-discovery outputs can feed either `spec-driven` or `event-driven` proposals without semantic drift.

- **Requirement ID**: `ito-schemas:cross-schema-discovery-vocabulary`

#### Scenario: Spec-driven and event-driven share discovery semantics

- **GIVEN** a user captures commands, domain events, actors, policies, aggregates, and bounded contexts during discovery
- **WHEN** the user chooses either `spec-driven` or `event-driven` as the final schema
- **THEN** the workflow reuses those concepts without forcing the user to rename or remodel them for the chosen schema

### Requirement: Discovery artifact optionality

Ito SHALL allow discovery artifacts or sections to be optional and technique-specific so a schema can request ubiquitous language and bounded context mapping without requiring event storming for every proposal.

- **Requirement ID**: `ito-schemas:discovery-artifact-optionality`

#### Scenario: Schema renders only selected discovery sections


- **GIVEN** a discovery handoff includes a glossary and bounded context map but no event-storming snapshot
- **WHEN** Ito renders proposal or design instructions for the final schema
- **THEN** the instructions include the available discovery context
- **AND** they do not treat the missing event-storming snapshot as an error unless the schema explicitly requires it
<!-- ITO:END -->
