---
name: ito-proposal
description: Use when creating and writing an Ito change proposal (new change or existing change id). Delegates to Ito CLI instruction artifacts.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.29-->


Collaborate with the user to understand their intent, then create a change and generate proposal artifacts.

**If the user already provided a change ID**, skip to Step 4 (Generate artifacts) — the change already exists.

**If the request arrived from `ito-proposal-intake`, `ito-fix`, or `ito-feature`**, treat the intake summary as the shared understanding. Ask only the smallest number of follow-up questions needed to unblock change creation.

**Step 0: Understand the change (do this first)**

Do NOT jump straight into creating files. Confirm the change shape first:

- Ask clarifying questions one at a time. Prefer multiple-choice when possible.
- Identify: What problem does this solve? Why now? What does success look like?
- Surface ambiguity early — if something is unclear or could be interpreted multiple ways, ask.
- Explore scope: What's in? What's explicitly out? Are there simpler alternatives?
- If the user's request is vague, propose 2-3 interpretations and ask which fits.
- If the request is still too underspecified for safe scaffolding, switch to `ito-proposal-intake` before continuing.
- If intake has already happened and the request still is not concrete enough, switch to `ito-brainstorming` instead of looping back into intake.
- If the request is already well-defined, confirm your understanding and move on — don't over-interview.

Only proceed to Step 1 once you and the user agree on what the change is and why it matters.

**Step 1: Choose a schema**

```bash
ito agent instruction schemas
```

Recommend the best-fit schema for the request shape:

- **spec-driven**: new capabilities, cross-cutting behavior changes, architecture work, or requests that remain broad or ambiguous
- **minimalist**: bounded fixes and small, rigorous platform/tooling/CI/infrastructure changes
- **tdd**: regression-oriented fixes where test-first work is the safest path
- **event-driven**: event- or message-centric systems and workflows

If the user has no preference, recommend the best fit rather than defaulting automatically to `spec-driven`. Keep `spec-driven` as the safe fallback when the request still needs the full proposal pipeline.

**Step 2: Confirm the module (mandatory gate)**

⛔ **Do NOT create any change scaffolding until the user has confirmed their module choice.**

1. Run `ito list --modules` to show available modules and sub-modules.
2. Present the user with these options and **wait for their response**:
   - **Use an existing module** — pick from the list (provide the ID)
   - **Create a new module** — enter a name (`ito create module "<name>"`)
   - **Create a new sub-module** under an existing module — specify parent ID and name (`ito create sub-module "<name>" --module <parent-id>`)
3. Do NOT silently default to module `000`. Always ask.

**Step 3: Create the change**

After the user confirms the module:

```bash
# For a module:
ito create change "<change-name>" --module <module-id> --schema <schema>

# For a sub-module:
ito create change "<change-name>" --sub-module <NNN.SS> --schema <schema>
```

**Step 4: Generate artifacts**

```bash
ito agent instruction proposal --change "<change-id>"
ito agent instruction specs --change "<change-id>"
ito agent instruction design --change "<change-id>"
ito agent instruction tasks --change "<change-id>"
```

Follow the printed instructions for each artifact exactly.

**Testing Policy**

- Default workflow: RED/GREEN/REFACTOR. Coverage target: 80% (projects may override).
- Follow the "Testing Policy" section emitted by `ito agent instruction proposal|apply`.

<!-- ITO:END -->
