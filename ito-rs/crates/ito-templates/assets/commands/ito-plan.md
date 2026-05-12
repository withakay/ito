---
name: ito-plan
description: Explore rough ideas before proposal scaffolding, including DDD discovery when useful.
category: Ito
tags: [ito, plan, discovery, ddd]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Load and follow the `ito-plan` skill. Pass the <UserRequest> block as input.

Use the least sufficient discovery depth. Inspect repository evidence before asking questions, and route the resulting handoff to `ito-proposal-intake` or `ito-proposal` when proposal-ready.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
