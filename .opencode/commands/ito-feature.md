---
name: ito-feature
description: Create a feature-oriented Ito change proposal with feature-biased intake and schema guidance.
category: Ito
tags: [ito, feature, proposal]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.29-->

Load and follow the `ito-feature` skill. Pass the <UserRequest> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
