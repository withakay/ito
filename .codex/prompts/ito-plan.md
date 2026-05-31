---
name: ito-plan
description: Explore and shape an idea before creating one or more Ito proposals.
category: Ito
tags: [ito, planning]
---

<PlanningRequest>
$ARGUMENTS
</PlanningRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->

Load and follow the `ito-plan` skill. Pass the <UserRequest> block as input.

Use the least sufficient discovery depth. Inspect repository evidence before asking questions, and route the resulting handoff to `ito-proposal-intake` or `ito-proposal` when proposal-ready.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
