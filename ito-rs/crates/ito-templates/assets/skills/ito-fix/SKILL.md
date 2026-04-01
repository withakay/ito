---
name: ito-fix
description: Start a fix-oriented Ito change proposal with fix-biased intake and schema recommendations.
---

Use this when the user frames the work as a fix, regression, correction, or supporting platform/tooling/infrastructure change that should be handled like a fix.

## Goal

Drive proposal creation through a fix-oriented intake lane without locking the user into a specific schema.

## Steps

1. Start with `ito-proposal-intake`.
2. Bias the intake toward fix questions:
   - What is broken?
   - What is the expected behavior?
   - Is there a reproduction or regression signal?
   - How bounded is the blast radius?
3. Recommend a schema:
   - `minimalist` for bounded fixes and small supporting platform or infrastructure changes
   - `tdd` for regression-oriented changes where test-first work is safest
   - `spec-driven` if the fix turns out to be broad, architectural, or still ambiguous
   - `event-driven` if the change is centered on event or message workflow behavior
4. If intake concludes no proposal is needed, say so and stop.
5. If the work is ready for a proposal, continue with `ito-proposal` using the intake summary as the shared understanding.
6. If the request turns into a new capability or open-ended design discussion, switch to `ito-feature` or `ito-brainstorming`.

## Important

- `ito-fix` is an opinionated front door, not a separate schema.
- The user may still override the recommended schema or continue through the neutral `ito-proposal` lane.
