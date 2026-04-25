---
name: ito-apply
description: Implement an approved Ito change and keep tasks in sync.
category: Ito
tags: [ito, apply]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.27-->

Load and follow the `ito-apply` skill. Pass the <UserRequest> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

**Testing policy**: follow the policy printed by `ito agent instruction apply --change <id>`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
