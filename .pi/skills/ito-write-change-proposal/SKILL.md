---
name: ito-write-change-proposal
description: Use when creating, designing, planning, proposing, or specifying a feature, change, requirement, enhancement, fix, modification, or spec. Use when writing tasks, proposals, specifications, or requirements for new work.
---

Collaborate with the user to understand their intent, then create a change and generate proposal artifacts.

**Step 0: Understand the change (do this first)**

Do NOT jump straight into creating files. Interview the user to build a shared understanding:

- Ask clarifying questions one at a time. Prefer multiple-choice when possible.
- Identify: What problem does this solve? Why now? What does success look like?
- Surface ambiguity early — if something is unclear or could be interpreted multiple ways, ask.
- Explore scope: What's in? What's explicitly out? Are there simpler alternatives?
- If the user's request is vague, propose 2-3 interpretations and ask which fits.
- If the request is already well-defined, confirm your understanding and move on — don't over-interview.

Only proceed to Step 1 once you and the user agree on what the change is and why it matters.

**Step 1: Create the change**

If the user provided an existing change ID, use it. Otherwise:

- Pick a module (default to `000` if unsure). Run `ito list --modules` to check.
- Run:
  ```bash
  ito create change "<change-name>" --module <module-id>
  ```

**Step 2: Generate artifacts**

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
