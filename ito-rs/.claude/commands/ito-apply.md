---
name: ito-apply
description: Implement an approved Ito change and keep tasks in sync.
category: Ito
tags: [ito, apply]
---

The user has requested to implement the following change proposal.
<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the Ito agent skill `ito-apply-change-proposal` (alias: `ito-apply`) as the source of truth for this workflow.

**Input**

- The change ID or implementation request is provided in the prompt arguments or <UserRequest> block.

**Instructions**

Tell the model to use the `ito-apply-change-proposal` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Testing Policy (TDD + coverage)**

- Default workflow: RED/GREEN/REFACTOR (write a failing test, implement the minimum to pass, then refactor).
- Coverage target: 80% (guidance; projects may override).
- Override defaults in `.ito/config.json` (or `.ito.json`, `ito.json`):

```json
{
  "defaults": {
    "testing": {
      "tdd": { "workflow": "red-green-refactor" },
      "coverage": { "target_percent": 80 }
    }
  }
}
```

**Guardrails**

- If the `ito-apply-change-proposal` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
