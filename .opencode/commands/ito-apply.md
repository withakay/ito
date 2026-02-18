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

**Audit guardrail (GitHub Copilot)**

- In GitHub Copilot repository-agent sessions, treat audit checks as mandatory before stateful Ito actions: run `ito audit validate` first.
- If validation fails or drift is reported, run `ito audit reconcile` and use `ito audit reconcile --fix` when remediation is required.

**Testing Policy (TDD + coverage)**

- Follow the Testing Policy printed by `ito agent instruction proposal` / `ito agent instruction apply`.
- Default workflow: RED/GREEN/REFACTOR (write a failing test, implement the minimum to pass, then refactor).
- Coverage target: 80% (guidance; projects may override).
- Override defaults via cascading project config (low -> high precedence): `ito.json`, `.ito.json`, `.ito/config.json`, `$PROJECT_DIR/config.json` (when set).
- Keys: `defaults.testing.tdd.workflow`, `defaults.testing.coverage.target_percent`.

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
