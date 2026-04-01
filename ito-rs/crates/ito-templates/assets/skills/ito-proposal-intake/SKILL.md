---
name: ito-proposal-intake
description: Clarify a requested change before scaffolding a proposal, then recommend the next workflow lane and schema.
---

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

## Schema Recommendation Rules

- Recommend `minimalist` for bounded fixes and small, rigorous platform, tooling, CI, or infrastructure changes.
- Recommend `tdd` for regression-oriented changes where reproducing the failure with a test is the safest starting point.
- Recommend `spec-driven` for new capabilities, broad behavior changes, architecture work, or requests that remain ambiguous after intake.
- If the request is event- or message-centric, consider `event-driven`.

## Outcomes

End the intake with one of these outcomes:

1. **Ready for `ito-proposal`**
   - Provide a concise summary and a recommended schema.
2. **Needs `ito-brainstorming` first**
   - Use this when the user is still exploring solution shape rather than scoping a concrete change.
3. **No proposal needed**
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
- Recommended schema: <minimalist|tdd|spec-driven|event-driven>
- Rationale: <why this schema fits>
```

Then continue with `ito-proposal` using that summary as the shared understanding. Do not restart discovery unless a blocking ambiguity remains.

If intake has already been attempted and the request still is not concrete enough for safe scaffolding, route to `ito-brainstorming` or ask the user for more context rather than restarting intake.
