---
name: ito-list
description: List Ito changes, archived changes, specs, or modules with status summaries.
category: Ito
tags: [ito, list]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

Load and follow the `ito-list` skill. Pass the <UserRequest> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, fall back to running `ito list` or `ito list-archive` directly with any user-supplied flags.

<!-- ITO:END -->
