---
name: ito-proposal
description: Scaffold a new Ito change and validate strictly.
category: Ito
tags: [ito, proposal, change]
---

The user wants to propose a change.
<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the `ito-write-change-proposal` skill as the source of truth for this workflow.

**Before anything else: collaborate with the user.** The request above is a starting point, not a final spec. Ask clarifying questions, surface ambiguity, and confirm your understanding before creating any files or changes. See the skill for detailed guidance.

**Module selection:** Prefer an existing module that fits (`ito list --modules`). Create a new one only if nothing fits.

**Audit:** In GitHub Copilot sessions, run `ito audit validate` before stateful actions.

**Testing Policy:** Follow the Testing Policy printed by `ito agent instruction proposal|apply`. Default: RED/GREEN/REFACTOR, 80% coverage target (projects may override).

**Guardrails:** If the `ito-write-change-proposal` skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
