---
name: loop
description: Run an Ito Ralph loop for a change.
category: Ito
tags: [ito, ralph, loop]
---

<UserRequest>
$ARGUMENTS
</UserRequest>

<!-- ITO:START -->

Load and follow the `ito-loop` skill. Pass the <UserRequest> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

**Notes**

- Ralph supports appending restart context: `ito ralph --add-context "..."`
- Ralph supports inactivity restarts: `ito ralph --timeout 15m`

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
