---
name: ito-archive
description: Archive a deployed Ito change and update specs.
category: Ito
tags: [ito, archive]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Load and follow the `ito-archive` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
