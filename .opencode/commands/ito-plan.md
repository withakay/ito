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
<!--ITO:VERSION:0.1.30-->

Load and follow the `ito-plan` skill. Pass the <PlanningRequest> block as input unchanged.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
