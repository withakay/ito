---
name: ito
description: Route ito commands via the ito skill (skill-first, CLI fallback).
category: Ito
tags: [ito, routing]
---

<ItoCommand>
$ARGUMENTS
</ItoCommand>

<!-- ITO:START -->

Load and follow the `ito` skill. Pass the <ItoCommand> block as input.

**Audit guardrail**

- Before stateful Ito actions: run `ito audit validate`.
- If validation fails or drift is reported, run `ito audit reconcile` and `ito audit reconcile --fix` to remediate.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
