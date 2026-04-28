---
name: ito-orchestrate
description: Coordinate multi-change apply runs with the Ito orchestrator.
category: Ito
tags: [ito, orchestrate]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.29-->

Load and follow the `ito-orchestrate` skill. Pass the <UserRequest> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
