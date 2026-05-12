<!-- ITO:START -->
# Domain Discovery

Use this optional artifact when a change needs DDD-oriented discovery before proposal, specs, or tasks. Keep routine one-context work on the normal fast path.

## Domain Discovery Summary

- **Primary problem**: <!-- One sentence describing the domain problem. -->
- **Discovery depth**: <!-- direct, lightweight, bounded-context, or rigorous domain-grill; include trigger rationale. -->
- **Business/domain capability**: <!-- Business capability being changed, distinct from Ito capability names. -->
- **Primary bounded context**: <!-- Context that owns the main behavior. -->
- **Supporting contexts**: <!-- Other contexts referenced or affected, or None. -->
- **Candidate Ito capabilities**: <!-- Proposed spec capability names informed by discovery. -->

## Ubiquitous Language

| Term | Definition | Owner / Context | Notes |
| --- | --- | --- | --- |
| <!-- canonical term --> | <!-- precise meaning --> | <!-- bounded context --> | <!-- aliases or caveats --> |

## Rejected Aliases / Overloaded Terms

| Term or Alias | Use Instead / Clarify As | Rationale |
| --- | --- | --- |
| <!-- alias --> | <!-- canonical term --> | <!-- why this avoids confusion --> |

## Bounded Context Map

| Context | Responsibilities | Owner | Owned Language / Concepts | Relationship Pattern |
| --- | --- | --- | --- | --- |
| <!-- context --> | <!-- business responsibility --> | <!-- owning team/context/person --> | <!-- owned terms --> | <!-- customer/supplier, conformist, ACL, shared kernel, separate ways, provisional/unknown --> |

## Model Ownership

- **Owned concepts changed**: <!-- Rules, lifecycle, language, or decisions owned by the primary context. -->
- **External concepts referenced**: <!-- Concepts borrowed from supporting contexts. -->
- **Translation boundaries**: <!-- Where external concepts become local concepts, or None. -->

## Consistency Requirements

- **Consistency expectation**: <!-- strong, eventual, or not applicable. -->
- **Conflict owner**: <!-- Context/actor that resolves conflicts. -->
- **Stale-data impact**: <!-- What breaks or degrades if downstream data is stale. -->
- **Downstream-unavailable behavior**: <!-- Retry, queue, fail closed/open, or not applicable. -->

## Technique Fit

| Technique | Selected? | Rationale |
| --- | --- | --- |
| Ubiquitous language | <!-- yes/no --> | <!-- why enough or unnecessary --> |
| Bounded context mapping | <!-- yes/no --> | <!-- why enough or unnecessary --> |
| Event storming / event modeling | <!-- yes/no --> | <!-- why useful or skipped --> |
| Domain-grill questioning | <!-- yes/no --> | <!-- why rigorous questioning is or is not needed --> |

## Event Storming Snapshot

<!-- Optional. Use when behavior depends on sequence, policies, reactions, or invariants. -->

- **Actors**: <!-- People, systems, or roles. -->
- **Commands**: <!-- Intentful actions. -->
- **Queries / read-model questions**: <!-- Questions the system must answer. -->
- **Domain events**: <!-- Past-tense business facts. -->
- **Policies**: <!-- Reactions or automations. -->
- **Aggregates / entities**: <!-- Consistency boundaries or long-lived domain objects. -->
- **Read models**: <!-- Projection needs. -->
- **Invariants**: <!-- Rules that must always hold. -->

## Evidence Checked

- <!-- Specs, code, CONTEXT.md, CONTEXT-MAP.md, ADRs, docs, or other evidence consulted before asking questions. -->

## Proposed Documentation Updates

- **CONTEXT.md**: <!-- Proposed durable language updates, or None. -->
- **CONTEXT-MAP.md**: <!-- Proposed context-map updates, or None. -->
- **ADR**: <!-- ADR-worthy decisions, or None. -->

## Open Questions

- <!-- Unresolved vocabulary, ownership, policy, sequencing, or boundary questions. -->
<!-- ITO:END -->
