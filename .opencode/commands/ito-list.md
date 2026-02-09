---
name: ito-list
description: List Ito changes, specs, or modules with status summaries.
category: Ito
tags: [ito, list]
---

The user wants to list Ito project items.
<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the Ito agent skill `ito-list` as the source of truth for this workflow.

**Input**

- Any arguments or filters are provided in the prompt arguments or <UserRequest> block.
- Common arguments: `--ready`, `--completed`, `--partial`, `--pending`, `--specs`, `--modules`, `--json`.
- If no arguments are given, list all changes (the default).

**Instructions**

Tell the model to use the `ito-list` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Guardrails**

- If the `ito-list` skill is missing or unavailable, fall back to running `ito list` directly via the CLI with any user-supplied flags.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
