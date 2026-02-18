---
name: ito-research
description: Conduct Ito research via skills (stack, architecture, features, pitfalls).
category: Ito
tags: [ito, research]
---

Conduct Ito research for the following topic.
<Topic>
$ARGUMENTS
</Topic>

<!-- ITO:START -->

Use the Ito agent skill `ito-research` as the source of truth for this workflow.

**Input**

- The research topic is provided in the prompt arguments or <Topic> block. It may include an optional focus (stack, architecture, features, pitfalls).

**Focus**

- If the user specifies one of: stack, architecture, features, pitfalls, follow the skill's focus guidance.
- If the focus is missing or unclear, ask the user whether they want a single investigation or the full research suite.

**Instructions**

Tell the model to use the `ito-research` skill to complete this workflow, using any supplied arguments or context from the prompt.

**Audit guardrail (GitHub Copilot)**

- In GitHub Copilot repository-agent sessions, treat audit checks as mandatory before stateful Ito actions: run `ito audit validate` first.
- If validation fails or drift is reported, run `ito audit reconcile` and use `ito audit reconcile --fix` when remediation is required.

**Guardrails**

- If the `ito-research` skill is missing or unavailable, ask the user to run `ito init` (or `ito update` if the project is already initialized), then stop.
- Do not duplicate the full workflow here; defer to the skill guidance.

<!-- ITO:END -->
