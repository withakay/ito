---
name: ito-plan
description: Exploratory, question-driven planning before creating Ito change proposals. Use when an idea needs shaping, scoping, sequencing, or research before proposal scaffolding.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->

# Plan Before Proposal

Use this skill when the request is too rough for safe proposal scaffolding or when the user asks to plan before creating a change.

## Goal

Turn an open-ended idea into a useful planning artifact and proposal-ready plan without prematurely choosing vocabulary, ownership, module boundaries, or schema details.

Store durable research notes under `.ito/research/` when the planning work produces reusable evidence.

## Guardrails

- Inspect existing code, specs, `CONTEXT.md`, `CONTEXT-MAP.md`, and ADRs before asking questions the repository can answer.
- Ask one unresolved question at a time, with a recommended answer and short rationale.
- Keep routine work lightweight; do not force DDD ceremony onto local mechanical changes.
- Distinguish business/domain capabilities, bounded contexts, Ito modules, and Ito capabilities.
- End with either a proposal handoff, a research task, or a clear reason no proposal is needed.

## Discovery Depth Gate

Choose the least sufficient discovery depth:

| Depth | Use When | Output |
| --- | --- | --- |
| Direct / skip | The work is routine, low-risk, one-context, and vocabulary is clear | Continue to proposal or implementation with a brief summary. |
| Lightweight discovery | Terms are fuzzy, overloaded, or domain-specific but scope is otherwise bounded | Resolve canonical terms, rejected aliases, and open vocabulary questions. |
| Bounded-context discovery | Work crosses ownership, integrations, modules, capabilities, or domain models | Name primary/supporting contexts, ownership, relationship pattern or `provisional/unknown`, and translation boundary. |
| Rigorous domain-grill | User opts in, or work is high-impact, architectural, public-contract-changing, hard to reverse, policy-heavy, sequencing-heavy, or cross-context with unresolved ownership | Challenge one decision at a time using repository evidence, concrete scenarios, and recommended defaults. |

Clear cross-context work must use at least bounded-context discovery.

## DDD Discovery Bundle

Capture only the sections that add signal for the selected depth:

- **Primary problem**: one sentence describing the domain problem.
- **Discovery depth**: selected depth and trigger rationale.
- **Business/domain capability**: the business capability being changed, distinct from Ito capability names.
- **Model ownership**: which bounded context owns the rules, lifecycle, language, and decision authority; do not use table, file, or service location as ownership proof.
- **Ubiquitous language**: canonical terms, definitions, rejected aliases, overloaded terms, and unresolved vocabulary.
- **Bounded contexts**: context names, responsibilities, ownership, owned language, primary context, and supporting contexts.
- **Cross-context relationships**: customer/supplier, conformist, anti-corruption layer, shared kernel, separate ways, another explicit pattern, or `provisional/unknown`.
- **Translation boundaries**: where external concepts become local concepts and which published language is consumed.
- **Consistency requirements**: strong/eventual consistency, conflict owner, stale-data impact, and downstream-unavailable behavior when relevant.
- **Technique fit**: selected and skipped techniques with rationale.
- **Evidence checked**: code, specs, context docs, ADRs, or prior plans consulted before asking the user.
- **Proposed documentation updates**: candidate `CONTEXT.md`, `CONTEXT-MAP.md`, or ADR updates, created lazily only for durable terms, context boundaries, or decisions that are hard to reverse, surprising without context, and based on a real trade-off.

When behavior is sequence-, policy-, or reaction-heavy, optionally add an event-storming snapshot:

- **Actors**
- **Commands**
- **Queries / read-model questions**
- **Domain events**
- **Policies**
- **Aggregates / entities**
- **Read models**
- **Invariants**

## Technique Fit Triage

- Use ubiquitous language work when terminology is overloaded, inconsistent, or domain-specific.
- Use bounded-context mapping when the request crosses ownership, capabilities, modules, integrations, or multiple domain models.
- Use event storming when sequencing, actors, commands, policies, reactions, invariants, or read-model questions clarify behavior.
- Skip any technique that does not reduce ambiguity for the current request.

## Domain-Grill Mode

Use rigorous domain-grill only at the selected depth. Challenge fuzzy plans by:

- Comparing user language against existing docs, specs, code, and the current discovery handoff.
- Proposing precise canonical terms when language is vague.
- Testing boundaries with concrete lifecycle, ownership, failure, and translation scenarios.
- Cross-checking claims against code and docs before accepting them as domain truth.
- Asking who owns rules, lifecycle, language, and decisions instead of who owns the data location.
- Treating `add a status`, `reuse the existing model`, `just sync the data`, `expose this field`, `put it in shared`, `add a flag`, and `use a common helper` as boundary-smell prompts.

## Proposal Handoff Format

When the plan is ready for proposal creation, hand off this summary:

```markdown
## Domain Discovery Summary
- Primary problem: <one sentence>
- Discovery depth: <direct|lightweight|bounded-context|rigorous domain-grill> because <trigger>
- Business/domain capability: <capability distinct from Ito capability>
- Primary bounded context: <context that owns the main behavior>
- Supporting contexts: <other contexts involved, or none>
- Model ownership: <who owns rules/lifecycle/language/decisions>
- Canonical terms: <term -> definition>
- Rejected aliases / overloaded terms: <alias or term -> guidance>
- Bounded contexts: <name -> responsibility, ownership, owned language>
- Owned concepts changed: <rules/lifecycle/language/decisions>
- External concepts referenced: <borrowed concepts from other contexts>
- Cross-context relationships: <pattern or provisional/unknown, published language, translation boundary>
- Translation boundaries: <where external concepts become local concepts>
- Consistency requirements: <strong/eventual, conflict owner, stale-data impact, unavailable-downstream behavior>
- Technique fit: <selected and skipped DDD techniques with rationale>
- Event-storming snapshot: <actors, commands, queries, events, policies, aggregates, read models, invariants if used>
- Candidate Ito capabilities: <proposal/spec capability names>
- Open questions: <unresolved vocabulary, ownership, policy, or sequencing questions>
- Evidence checked: <specs/files/docs/ADRs consulted>
- Proposed documentation updates: <CONTEXT.md, CONTEXT-MAP.md, ADR candidates, or none>
```

Then route to `ito-proposal-intake` or `ito-proposal` using this handoff as shared context. Do not restart discovery unless a blocking ambiguity remains.

<!-- ITO:END -->
