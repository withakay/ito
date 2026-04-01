---
name: ito-feature
description: Start a feature-oriented Ito change proposal with feature-biased intake and schema recommendations.
---

Use this when the user is introducing a new capability, expanding an existing one, or otherwise framing the work as a feature.
**If the user already provided a change ID**, skip to Step 5 (Continue with `ito-proposal`) — the change already exists.

## Goal

Drive proposal creation through a feature-oriented intake lane that gives the user enough discovery without forcing a fix-shaped workflow.

## Steps

1. Start with `ito-proposal-intake`.
2. Bias the intake toward feature questions:
   - What new behavior or capability is needed?
   - Why now?
   - What does success look like?
   - Is the solution shape already known, or does it need brainstorming?
3. Recommend a schema:
   - `spec-driven` for new capabilities, behavior changes, and ambiguous feature work
   - `minimalist` only if the requested enhancement is unusually bounded and low-risk
   - `tdd` when the feature is best introduced by writing tests against the desired behavior first
   - `event-driven` if the feature is centered on event or message workflow behavior
4. If intake shows the user is still exploring the solution space, switch to `ito-brainstorming`.
5. If the work is ready for a proposal, continue with `ito-proposal` using the intake summary as the shared understanding.
6. If the request is actually a bounded fix, switch to `ito-fix` or the neutral `ito-proposal` lane.

## Important

- `ito-feature` is an opinionated front door, not a separate schema.
- The user may still override the recommended schema or continue through the neutral `ito-proposal` lane.
