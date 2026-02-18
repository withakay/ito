---
name: ito-proposal
description: Scaffold a new Ito change and validate strictly.
category: Ito
tags: [ito, proposal, change]
---

The user has requested the following change proposal.
<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the Ito agent skill `ito-write-change-proposal` (alias: `ito-proposal`) as the source of truth for this workflow.

**Input**

- The request is provided in the prompt arguments or <UserRequest> block. Use it to scope the change and name the change ID.

**Module selection**

- Prefer a semantic fit in an existing module: run `ito list --modules` and choose the closest match by purpose/scope.
- If no module is a good fit, propose creating a new module for the theme of the work.

**Instructions**

Tell the model to use the `ito-write-change-proposal` skill to complete this workflow, using any supplied arguments or context from the prompt.

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

- If the `ito-write-change-proposal` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
