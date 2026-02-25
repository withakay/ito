---
name: ito-research
description: Conduct Ito research via skills (stack, architecture, features, pitfalls).
category: Ito
tags: [ito, research]
---

<Topic>
$ARGUMENTS
</Topic>

<!-- ITO:START -->

Load and follow the `ito-research` skill. Pass the <Topic> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
