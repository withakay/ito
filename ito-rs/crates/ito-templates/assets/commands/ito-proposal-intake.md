---
name: ito-proposal-intake
description: Clarify a requested change before scaffolding an Ito proposal.
category: Ito
tags: [ito, proposal, intake]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Load and follow the `ito-proposal-intake` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
