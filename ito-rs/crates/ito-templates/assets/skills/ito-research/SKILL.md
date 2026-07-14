---
name: ito-research
description: Investigate technologies, architecture, risks, project knowledge, and proposal evidence with cited synthesis.
---

<!-- ITO:START -->
# Research lifecycle

Use the phase resources in this directory for stack, feature, architecture, pitfalls, security, scale, edge-case, and synthesis work. Save source investigations under `.ito/research/<topic>/` or change reviews under `.ito/changes/<change-id>/reviews/`.

Read `.ito/wiki/index.md` when present. Treat `.ito/wiki/` as synthesized navigation: cite the underlying spec, change, research, code, or documentation and call out stale or contradictory coverage. Fall back to raw Ito artifacts rather than blocking.

Use Ito's configured, provider-neutral memory operations when helpful:

```bash
ito agent instruction memory-search --query "<terms>" --limit 10
ito agent instruction memory-query --query "<question>"
```

Memory is guidance, not the source of truth. Current specs, code, and rendered instructions win conflicts. Durable capture belongs to the archive phase.

Finish with a concise recommendation containing evidence, alternatives, trade-offs, risks, and confidence. Link the synthesis from the proposal/design when research informs a change, and update wiki topic/query artifacts only when the result has durable reuse value.
<!-- ITO:END -->
