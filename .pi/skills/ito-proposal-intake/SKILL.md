---
name: ito-proposal-intake
description: Clarify a requested change before scaffolding a proposal, then recommend the next workflow lane and schema.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->


Use this before proposal scaffolding when the request is underspecified or when starting from an intent-biased entrypoint such as `ito-fix` or `ito-feature`.
**If the user already provided a change ID**, skip to the Handoff Format and continue with `ito-proposal` — the change already exists.

## Goal

Determine what the user is actually asking for, whether a proposal is warranted, and which workflow lane and schema fit best.

## Guardrails

- Do NOT scaffold a change or create files in this skill.
- Ask one question at a time.
- Prefer multiple-choice questions when possible.
- For brownfield work, inspect the repo or existing specs before asking the user to rediscover facts already available.
- End with an explicit handoff outcome.

## Intake Checklist

Clarify only the missing pieces needed to route the request safely:

- What problem is being solved?
- Is this a fix, a feature, or still unclear?
- What behavior is broken or missing today?
- What outcome would count as success?
- What is in scope, and what is explicitly out of scope?
- Is the blast radius local, cross-cutting, or architectural?
- Does existing code or an existing spec already define the intended behavior?

## Domain Discovery Gate

Before recommending a schema, choose the least sufficient discovery depth:

| Depth | Use When | Outcome |
| --- | --- | --- |
| Direct / skip | Routine, low-risk, one-context work with clear vocabulary | Continue with normal intake. |
| Lightweight discovery | Terms are fuzzy, overloaded, or domain-specific | Resolve canonical terms, rejected aliases, and open vocabulary questions. |
| Bounded-context discovery | Work crosses ownership, integrations, modules, capabilities, or domain models | Identify primary/supporting bounded contexts, model ownership, relationship pattern or `provisional/unknown`, and translation boundary. |
| Rigorous domain-grill | User opts in, or work is high-impact, architectural, public-contract-changing, hard to reverse, policy-heavy, sequencing-heavy, or cross-context with unresolved ownership | Use evidence-backed, one-question-at-a-time domain grilling with recommended answers. |

Clear cross-context work must use at least bounded-context discovery. Keep routine work on the fast path.

When discovery is needed, capture this grammar before proposal scaffolding:

- **Business/domain capability**: the business capability being changed, distinct from Ito capability names.
- **Model ownership**: who owns the rules, lifecycle, language, and decisions; do not infer ownership from data, table, service, or file location alone.
- **Ubiquitous language**: canonical terms, definitions, rejected aliases, overloaded terms, and unresolved language questions.
- **Bounded contexts**: primary context, supporting contexts, responsibilities, owned language, and external concepts referenced.
- **Relationship pattern**: customer/supplier, conformist, anti-corruption layer, shared kernel, separate ways, another explicit pattern, or `provisional/unknown`.
- **Consistency requirements**: strong/eventual consistency, conflict owner, stale-data impact, and downstream-unavailable behavior when relevant.
- **Technique fit**: selected and skipped DDD techniques with rationale.
- **Evidence checked**: code, specs, context docs, ADRs, or prior plans consulted before asking the user.

Use optional event-storming concepts when sequencing, policy, reactions, or invariants clarify behavior: actors, commands, queries, domain events, policies, aggregates/entities, read models, and invariants.

In rigorous domain-grill mode, challenge fuzzy or boundary-smell requests like `add a status`, `reuse the existing model`, `just sync the data`, `expose this field`, `put it in shared`, `add a flag`, or `use a common helper`. Probe ownership, lifecycle, failure behavior, and translation boundaries with concrete scenarios.

## Schema Recommendation Rules

- Recommend `minimalist` for bounded fixes and small, rigorous platform, tooling, CI, or infrastructure changes.
- Recommend `tdd` for regression-oriented changes where reproducing the failure with a test is the safest starting point.
- Recommend `spec-driven` for new capabilities, broad behavior changes, architecture work, or requests that remain ambiguous after intake.
- If the request is event- or message-centric, consider `event-driven`.
- If the request needs more discovery before any proposal is safe, route to `ito-plan` with the selected discovery depth instead of forcing a schema decision.

## Outcomes

End the intake with one of these outcomes:

1. **Ready for `ito-proposal`**
   - Provide a concise summary and a recommended schema.
2. **Needs `ito-brainstorming` first**
   - Use this when the user is still exploring solution shape rather than scoping a concrete change.
3. **Needs `ito-plan` domain discovery first**
   - Use this when the request is proposal-shaped but needs language, ownership, boundary, consistency, or event-storming discovery before scaffolding.
4. **No proposal needed**
   - Use this for straightforward fixes or edits that should be handled directly.

## Handoff Format

When the change is ready for proposal creation, hand off this summary to the next lane:

```markdown
## Intake Summary
- Request type: <fix|feature|neutral>
- Problem: <one sentence>
- Desired outcome: <one sentence>
- Scope: <what is in>
- Non-goals: <what is out>
- Brownfield evidence: <specs/files/patterns, if relevant>
- Domain discovery depth: <direct|lightweight|bounded-context|rigorous domain-grill>
- Domain discovery summary: <business capability, primary context, model ownership, canonical terms, relationship pattern, consistency requirements, technique fit, open questions>
- Recommended schema: <minimalist|tdd|spec-driven|event-driven>
- Rationale: <why this schema fits>
```

If domain discovery has produced a full handoff, include it immediately after the intake summary:

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

Then continue with `ito-proposal` using that summary as the shared understanding. Do not restart discovery unless a blocking ambiguity remains.

If intake has already been attempted and the request still is not concrete enough for safe scaffolding, route to `ito-brainstorming` or ask the user for more context rather than restarting intake.

<!-- ITO:END -->
