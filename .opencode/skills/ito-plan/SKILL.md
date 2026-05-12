---
name: ito-plan
description: Exploratory, question-driven planning before creating Ito change proposals. Use when an idea needs shaping, scoping, sequencing, or research before proposal scaffolding.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Ito Plan

Use this skill before proposal creation when the user has an idea, rough feature request, product direction, or ambiguous improvement that needs exploration before it becomes one or more Ito changes.

## Goal

Turn an open-ended idea into a useful planning artifact under `.ito/planning/`, with supporting research under `.ito/research/` when deeper investigation is needed.

## Guardrails

- Do not scaffold a change proposal from this skill.
- Ask clarifying questions before writing a plan; always confirm scope, goals, constraints, or success criteria, even when the request seems clear.
- Prefer one question at a time; use multiple choice when it reduces user effort.
- Inspect existing specs, changes, code, and planning documents before asking for facts the repo already contains.
- Treat planning output as a precursor to one or more later `ito-proposal` runs.

## Planning Questions

Clarify enough to make the plan actionable:

- What problem or opportunity is this addressing?
- Who is affected, and what outcome would count as success?
- What constraints, deadlines, compatibility concerns, or non-goals matter?
- What options or trade-offs need comparison?
- Could this become one proposal, or should it split into multiple proposals?
- What evidence is missing and should be researched first?

## Artifact Locations

Use Ito path helpers instead of hard-coded absolute paths:

```bash
ITO_ROOT="$(ito path ito-root)"
```

Save planning synthesis as markdown under:

- `$ITO_ROOT/planning/<topic>.md`

Save deeper research evidence under:

- `$ITO_ROOT/research/<topic>/`

Plans should reference relevant research files instead of duplicating long evidence dumps.

## Plan Shape

Use a lightweight structure that fits the topic. A good default is:

```markdown
# <Topic> Plan

## Context
## Goals
## Non-Goals
## Constraints
## Options
## Recommendation
## Proposal Split
## Open Questions
## Research References
```

Keep the document practical. It should help the next agent create focused proposals, not become a rigid specification.

## Handoff

End with one of these outcomes:

1. **Ready for proposal**: summarize the recommended proposal boundary or boundaries and suggest `ito-proposal` as the next step.
2. **Needs research**: identify the research topic and save or request research under `.ito/research/` before proposal work.
3. **Needs more planning**: list the blocking questions and update the plan with current understanding.

<!-- ITO:END -->
