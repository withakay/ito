<!-- ITO:START -->
## ADDED Requirements

### Requirement: Domain discovery artifacts are schema-addressable

Ito MUST allow workflow schemas to define reusable domain-discovery artifacts or artifact sections that capture discovery depth, business/domain capability, model ownership, ubiquitous language, bounded contexts, technique-fit decisions, optional event-storming outputs, consistency requirements, and handoff summaries.

- **Requirement ID**: `ito-schemas:domain-discovery-artifacts`

#### Scenario: Schema declares discovery artifacts

- **GIVEN** a workflow schema defines a discovery artifact or template section
- **WHEN** Ito loads the schema
- **THEN** the discovery artifact is treated as part of the schema's artifact vocabulary
- **AND** later instruction rendering can reference it as dependency context

### Requirement: Canonical discovery summary contract

Ito MUST define a stable discovery summary contract that schema instructions, proposal scaffolding, review guidance, and validators can consume across artifact locations.

- **Requirement ID**: `ito-schemas:canonical-discovery-summary-contract`

#### Scenario: Discovery summary can be embedded or standalone

- **GIVEN** discovery output exists as a standalone `domain-discovery.md` artifact or as a `Domain Discovery Summary` section inside another planning/proposal artifact
- **WHEN** Ito instructions or validators consume discovery context
- **THEN** they can read stable fields for discovery depth, business/domain capability, primary bounded context, supporting contexts, canonical terms, rejected aliases, owned concepts, external concepts, relationships, relationship pattern or provisional unknown, translation required, consistency requirements, selected techniques, candidate Ito capabilities, evidence checked, proposed documentation updates, and open questions
- **AND** they do not depend on a single physical file path when the schema declares an equivalent artifact section

### Requirement: Strategic DDD reference is bundle-addressable

Ito MUST allow workflow guidance to reference bundled strategic DDD material as supporting context without treating the full reference as a required artifact or validation contract.

- **Requirement ID**: `ito-schemas:strategic-ddd-reference-bundle`

#### Scenario: Instructions link to bundled DDD reference

- **GIVEN** the strategic DDD guide is bundled as reference material
- **WHEN** Ito renders domain-discovery or review guidance
- **THEN** the guidance can point agents at the reference for deeper examples and heuristics
- **AND** schema validation continues to use the compact canonical discovery summary contract

### Requirement: Domain documentation location discovery

Ito schema and instruction guidance SHALL support discovering existing domain documentation locations before creating new context or ADR files.

- **Requirement ID**: `ito-schemas:domain-documentation-location-discovery`

#### Scenario: Context map chooses documentation scope

- **GIVEN** a repository contains a root `CONTEXT-MAP.md` that points to context-specific `CONTEXT.md` and `docs/adr/` locations
- **WHEN** discovery captures a term or decision for a specific bounded context
- **THEN** instructions guide the agent to use that context-specific location rather than defaulting to root-level documentation

#### Scenario: Single-context repository uses root docs lazily

- **GIVEN** no `CONTEXT-MAP.md` exists
- **WHEN** discovery captures durable domain knowledge
- **THEN** instructions guide the agent to use root `CONTEXT.md` and root `docs/adr/` if they exist
- **AND** to create them only when the captured term or ADR-worthy decision justifies it

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
