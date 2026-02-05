---
name: ito-archive
description: Archive a deployed Ito change and update specs.
category: Ito
tags: [ito, archive]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the Ito agent skill `ito-archive` as the source of truth for this workflow.

**Input**

- The change ID is provided in the prompt arguments or <UserRequest> block.

**Instructions**

Tell the model to use the `ito-archive` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Guardrails**

- If the `ito-archive` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
