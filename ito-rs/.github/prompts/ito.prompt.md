---
name: ito
description: Route ito commands via the ito skill (skill-first, CLI fallback).
category: Ito
tags: [ito, routing]
---

The user has requested the following ito command.
<ItoCommand>
$ARGUMENTS
</ItoCommand>

<!-- ITO:START -->

Use the Ito agent skill `ito` as the source of truth for this workflow.

**Input**

- The ito command and arguments are provided in the prompt arguments or <ItoCommand> block.

**Instructions**

Tell the model to use the `ito` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Guardrails**

- If the `ito` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
