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

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
