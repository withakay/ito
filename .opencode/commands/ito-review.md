---
name: ito-review
description: Conduct adversarial review via Ito review skill.
category: Ito
tags: [ito, review]
---

Review the following change or scope.
<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Use the Ito agent skill `ito-review` as the source of truth for this workflow.

**Input**

- The change ID or review target is provided in the prompt arguments or <UserRequest> block.

**Instructions**

Tell the model to use the `ito-review` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Audit guardrail (GitHub Copilot)**

- In GitHub Copilot repository-agent sessions, treat audit checks as mandatory before stateful Ito actions: run `ito audit validate` first.
- If validation fails or drift is reported, run `ito audit reconcile` and use `ito audit reconcile --fix` when remediation is required.

**Guardrails**

- If the `ito-review` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
