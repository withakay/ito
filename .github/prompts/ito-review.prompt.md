---
name: ito-review
description: Conduct adversarial review via Ito review skill.
category: Ito
tags: [ito, review]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

Load and follow the `ito-review` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
