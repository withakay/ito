---
name: ito-list
description: List Ito changes, specs, or modules with status summaries.
category: Ito
tags: [ito, list]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Load and follow the `ito-list` skill. Pass the <UserRequest> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

If the skill is missing, fall back to running `ito list` directly with any user-supplied flags.

<!-- ITO:END -->
