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

Load and follow the `ito-apply` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

**Testing policy**: follow the policy printed by `ito agent instruction apply --change <id>`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
