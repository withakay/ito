<!-- ITO:START -->
## ADDED Requirements

### Requirement: DDD discovery bundle

The system SHALL provide a DDD-oriented discovery bundle for ambiguous, architectural, or cross-context work before proposal scaffolding. The minimum bundle MUST extract business/domain capability, ubiquitous language, bounded contexts, model ownership, technique-fit decisions, and proposal-relevant open questions; command, query, event, policy, aggregate, read-model, consistency, and invariant details are required only when the selected technique needs them.

- **Requirement ID**: `domain-discovery-workflow:ddd-discovery-bundle`

#### Scenario: Planning lane enters domain discovery mode

- **WHEN** a user starts planning for a broad or ambiguous change
- **THEN** the workflow asks discovery questions about domain terms, responsibilities, actors, commands, events, policies, and constraints
- **AND** it records the selected discovery outputs in a canonical discovery handoff rather than jumping straight to proposal prose

### Requirement: Discovery depth gate

The discovery workflow SHALL classify the appropriate discovery depth before questioning begins. Routine bounded work MAY keep the direct path; terminology ambiguity SHOULD use lightweight discovery; clear cross-context work MUST use at least bounded-context discovery; high-impact, architectural, policy-heavy, sequencing-heavy, or explicitly opted-in work SHOULD use rigorous domain-grill mode.

- **Requirement ID**: `domain-discovery-workflow:discovery-depth-gate`

#### Scenario: Routine work skips DDD discovery

- **WHEN** a request is local, low-risk, clear, and already bounded to one domain model
- **THEN** the workflow may skip DDD discovery and continue through the direct proposal or implementation path
- **AND** it does not ask domain-grill questions solely because DDD guidance exists

#### Scenario: User opts into rigorous grilling

- **WHEN** the user asks to be rigorously interviewed or stress-test a plan
- **THEN** the workflow enters rigorous domain-grill mode
- **AND** it asks dependency-ordered questions one at a time with recommended answers

#### Scenario: High-impact ambiguity auto-recommends rigorous grilling

- **WHEN** a request is architectural, public-contract-changing, hard to reverse, policy-heavy, sequencing-heavy, or spans multiple bounded contexts with unresolved ownership
- **THEN** the workflow recommends rigorous domain-grill mode before proposal scaffolding
- **AND** the user can still keep the scope lighter by explicitly accepting the modeling risk

### Requirement: Business capability first

The discovery workflow SHALL identify the business or domain capability being changed before choosing code locations, Ito modules, or Ito capabilities. Business/domain capability MUST remain distinct from bounded context and Ito capability in the discovery handoff.

- **Requirement ID**: `domain-discovery-workflow:business-capability-first`

#### Scenario: Capability is not inferred from file location

- **WHEN** a request names a code directory, service, table, or shared helper
- **THEN** the workflow asks what business/domain capability is changing
- **AND** it records candidate Ito capabilities separately from the business/domain capability

### Requirement: Model ownership over data location

The discovery workflow SHALL identify which bounded context owns the rules, lifecycle, language, and decision authority for a concept instead of inferring ownership from database tables, file paths, API access, or existing service dependencies.

- **Requirement ID**: `domain-discovery-workflow:model-ownership-over-data-location`

#### Scenario: Data access does not imply model ownership

- **WHEN** an existing table, service, or shared model has the data needed for a feature
- **THEN** the workflow asks which context owns the rule, lifecycle, and language for the behavior
- **AND** it treats data/code access as implementation evidence rather than ownership proof

### Requirement: Canonical discovery handoff

The discovery workflow SHALL produce a canonical discovery handoff that downstream proposal, spec, task, review, and validation steps can consume. The handoff MUST use stable headings or fields for discovery depth, business/domain capability, primary bounded context, supporting contexts, canonical terms, rejected aliases, owned concepts, external concepts, context relationships, relationship pattern or provisional unknown, translation required, consistency requirements, selected techniques, candidate Ito capabilities, evidence checked, proposed documentation updates, and open questions.

- **Requirement ID**: `domain-discovery-workflow:canonical-discovery-handoff`

#### Scenario: Downstream workflow reads stable discovery fields

- **WHEN** proposal scaffolding, review guidance, or validation needs discovery context
- **THEN** it reads the canonical discovery handoff or embedded `Domain Discovery Summary` section
- **AND** it can identify capability, glossary, context, ownership, relationship, consistency, technique-fit, evidence, proposed-documentation, and open-question fields without relying on free-form prose

### Requirement: Domain grill interview mode

The discovery workflow SHALL provide a domain-grill interview mode that challenges plans against existing domain language, documented decisions, and code behavior one unresolved decision at a time.

- **Requirement ID**: `domain-discovery-workflow:domain-grill-interview-mode`

#### Scenario: Repository evidence replaces answerable questions

- **WHEN** a discovery question can be answered by reading existing specs, `CONTEXT.md`, `CONTEXT-MAP.md`, ADRs, or code
- **THEN** the workflow explores those sources before asking the user
- **AND** it presents the discovered evidence with a recommended answer instead of making the user repeat documented facts

#### Scenario: One decision is resolved at a time

- **WHEN** repository evidence cannot resolve a domain decision
- **THEN** the workflow asks one targeted question with a recommended answer
- **AND** it waits for feedback before moving to dependent decisions

### Requirement: Glossary conflict challenge

The discovery workflow SHALL challenge terminology that conflicts with existing domain language and SHALL propose canonical terms for vague or overloaded language.

- **Requirement ID**: `domain-discovery-workflow:glossary-conflict-challenge`

#### Scenario: Existing glossary conflicts with user language

- **GIVEN** existing domain documentation defines a term one way
- **WHEN** the user uses the term to mean something different
- **THEN** the workflow calls out the conflict immediately
- **AND** it asks whether to preserve the existing meaning, rename the new concept, or explicitly record an intentional semantic change

#### Scenario: Fuzzy term is sharpened

- **WHEN** the user uses an overloaded term such as `account`, `project`, or `workspace`
- **THEN** the workflow proposes a precise canonical term based on discovered domain language
- **AND** unresolved ambiguity is captured in the discovery handoff

### Requirement: Scenario-based boundary probing

The discovery workflow SHALL use concrete scenarios to test domain relationships, edge cases, and bounded-context boundaries before proposal scope is finalized.

- **Requirement ID**: `domain-discovery-workflow:scenario-boundary-probing`

#### Scenario: Edge case exposes boundary ambiguity

- **WHEN** a domain relationship is unclear or crosses contexts
- **THEN** the workflow invents a concrete scenario that probes ownership, lifecycle, failure, or translation-boundary behavior
- **AND** the answer updates the context map or open questions before proposal drafting continues

### Requirement: Code and documentation cross-check

The discovery workflow SHALL cross-check user claims against existing code, specs, and documentation when those sources are available.

- **Requirement ID**: `domain-discovery-workflow:code-documentation-cross-check`

#### Scenario: Code contradicts stated domain behavior

- **WHEN** the user states a behavior that differs from the current code, specs, or documented decision records
- **THEN** the workflow surfaces the contradiction with source references
- **AND** it asks whether the proposal should preserve current behavior, change behavior, or correct documentation

### Requirement: Ubiquitous language glossary

The discovery workflow SHALL produce a glossary that names canonical domain terms, short definitions, rejected aliases, overloaded terms, and unresolved vocabulary questions.

- **Requirement ID**: `domain-discovery-workflow:ubiquitous-language-glossary`

#### Scenario: Vocabulary ambiguity is resolved before proposal drafting

- **WHEN** a user describes the same concept with multiple names during discovery
- **THEN** the workflow asks which term is canonical or marks the vocabulary question as unresolved
- **AND** proposal, spec, and task guidance can reuse the canonical term instead of inventing a new synonym

### Requirement: Bounded context map

The discovery workflow SHALL produce a bounded context map that identifies context names, responsibilities, owned language, ownership, upstream/downstream relationships, and translation boundaries when more than one model is involved.

- **Requirement ID**: `domain-discovery-workflow:bounded-context-map`

#### Scenario: Context ownership is explicit

- **WHEN** a change crosses multiple domain models or teams' responsibilities
- **THEN** the workflow records the affected bounded contexts and what each context owns
- **AND** it describes ownership, relationships, and translation boundaries before proposal scope is finalized

### Requirement: Context relationship pattern selection

For materially cross-context work, the discovery workflow SHALL record a context relationship pattern such as customer/supplier, conformist, anti-corruption layer, shared kernel, or separate ways, or it SHALL explicitly mark the relationship as provisional or unknown.

- **Requirement ID**: `domain-discovery-workflow:context-relationship-pattern-selection`

#### Scenario: Cross-context relationship is classified or left provisional

- **WHEN** a request involves more than one bounded context
- **THEN** the workflow asks whether the relationship is customer/supplier, conformist, anti-corruption layer, shared kernel, separate ways, or another explicit relationship
- **AND** if the relationship cannot be resolved, it records the relationship as provisional or unknown instead of forcing false precision

### Requirement: Consistency requirement capture

For cross-context, event-heavy, policy-heavy, or workflow-sequencing changes, the discovery workflow SHALL record consistency expectations, including which invariants require strong consistency, which updates can be eventually consistent, stale-data impact, conflict ownership, and downstream-unavailable behavior when relevant.

- **Requirement ID**: `domain-discovery-workflow:consistency-requirement-capture`

#### Scenario: Cross-context consistency is explicit

- **WHEN** a change coordinates behavior across bounded contexts
- **THEN** the workflow records whether each important rule needs strong consistency or can tolerate eventual consistency
- **AND** it names who owns conflict resolution or records the question as unresolved

### Requirement: Technique-fit triage

The discovery workflow SHALL explicitly record which DDD techniques are selected for the request and why omitted techniques are unnecessary.

- **Requirement ID**: `domain-discovery-workflow:technique-fit-triage`

#### Scenario: Discovery stays proportional

- **WHEN** the workflow considers ubiquitous language, bounded context mapping, and event storming
- **THEN** it records which techniques are selected and which are skipped
- **AND** it explains the decision using the request's ambiguity, context spread, temporal behavior, policy complexity, and implementation risk

### Requirement: Event storming technique fit

The discovery workflow SHALL treat event storming as an optional DDD technique, not a mandatory artifact. It MUST recommend event storming when behavior is temporal, event-heavy, policy-driven, or unclear from static requirements alone.

- **Requirement ID**: `domain-discovery-workflow:event-storming-technique-fit`

#### Scenario: Event storming is used when behavior needs sequencing

- **WHEN** a request depends on ordering, domain events, policies, or cross-context reactions
- **THEN** the workflow asks for commands, queries when relevant, domain events, actors, policies, aggregates, read models, consistency requirements, and invariants
- **AND** the resulting event-storming snapshot feeds proposal and spec drafting

### Requirement: Boundary smell probes

The discovery workflow SHALL include optional domain-grill probes for common boundary smells such as adding a status, reusing an existing model, syncing data, exposing a field, putting behavior in shared/common/helper code, or adding flags to generic models.

- **Requirement ID**: `domain-discovery-workflow:boundary-smell-probes`

#### Scenario: Shared helper request is challenged

- **WHEN** a plan proposes putting domain behavior in shared, common, helper, manager, processor, or generic model code
- **THEN** the workflow asks whether the behavior is truly cross-cutting or whether a bounded context owns the rule
- **AND** it records the chosen owner or unresolved boundary risk in the discovery handoff

### Requirement: Strategic DDD reference material

The discovery workflow SHALL preserve the full strategic DDD guide as bundled non-normative reference material and SHALL keep the canonical workflow contract compact. The guide MAY inform prompts and review checklists, but it MUST NOT make every tactical DDD heuristic mandatory for every proposal.

- **Requirement ID**: `domain-discovery-workflow:strategic-ddd-reference-material`

#### Scenario: Agent can consult the reference without expanding mandatory workflow

- **WHEN** an agent needs deeper strategic DDD examples or implementation heuristics
- **THEN** Ito can point to the bundled strategic DDD reference artifact
- **AND** routine proposal validation remains governed by the compact discovery handoff and selected depth gate

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

### Requirement: Lazy domain documentation capture

The discovery workflow SHALL update or propose updates to durable domain documentation only when a domain term, context boundary, or decision has crystallized. It MUST prefer existing `CONTEXT.md`, `CONTEXT-MAP.md`, and ADR locations when present, and it MUST create those files lazily only when there is durable domain knowledge to record.

- **Requirement ID**: `domain-discovery-workflow:lazy-domain-documentation-capture`

#### Scenario: Resolved term updates domain context

- **WHEN** a canonical term or bounded-context responsibility is resolved during discovery
- **THEN** the workflow records it in the discovery handoff
- **AND** if the change is approved for documentation updates, it updates the relevant `CONTEXT.md` or proposes creating one in the appropriate context location

#### Scenario: ADR is offered only for consequential trade-offs

- **WHEN** a decision is hard to reverse, surprising without context, and the result of a real trade-off
- **THEN** the workflow offers an ADR in the appropriate system-wide or context-specific `docs/adr/` location
- **AND** it does not create an ADR for ordinary naming, formatting, or low-consequence implementation details

### Requirement: Approved domain documentation promotion

The discovery workflow SHALL define how proposed `CONTEXT.md`, `CONTEXT-MAP.md`, and ADR updates are promoted after approval. Promotion MUST happen only through the apply/archive/finish path for an approved change, and proposed documentation updates MUST remain non-canonical until that point.

- **Requirement ID**: `domain-discovery-workflow:approved-domain-documentation-promotion`

#### Scenario: Approved change promotes proposed domain docs

- **GIVEN** an approved change includes proposed context or ADR updates from discovery
- **WHEN** the change is applied or archived according to the selected workflow
- **THEN** the proposed updates are written to the appropriate root or context-specific documentation locations
- **AND** the canonical discovery handoff remains traceable to the promoted documentation

#### Scenario: Unapproved discovery does not change canonical docs

- **GIVEN** discovery captured proposed domain documentation updates
- **WHEN** the change is not yet approved or is abandoned
- **THEN** those updates remain in the change package or worktree only
- **AND** they are not treated as accepted project domain language
<!-- ITO:END -->
