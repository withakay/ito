---
name: ito-proposal
description: Scaffold a new Ito change and validate strictly.
category: Ito
tags: [ito, proposal, change]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

Load and follow the `ito-proposal` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

**Testing policy**: follow the policy printed by `ito agent instruction proposal --change <id>`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
