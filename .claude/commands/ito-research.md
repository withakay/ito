---
name: ito-research
description: Conduct Ito research via skills (stack, architecture, features, pitfalls).
category: Ito
tags: [ito, research]
---

<Topic>
$ARGUMENTS
</Topic>

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

Load and follow the `ito-research` skill. Pass the <Topic> block as input.

Before stateful Ito actions, run `ito audit validate`; if it fails or reports drift, run `ito audit reconcile` then `ito audit reconcile --fix`.

If the skill is missing, ask the user to run `ito init` or `ito update`, then stop.

<!-- ITO:END -->
