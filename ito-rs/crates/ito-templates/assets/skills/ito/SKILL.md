---
name: ito
description: Route Ito work through its six lifecycle phases or the direct CLI without dynamic helper-skill discovery.
---

<!-- ITO:START -->
# Ito lifecycle router

Ito has six lifecycle destinations beneath this root entrypoint:

| Intent | Retained skill |
| --- | --- |
| clarify, plan, or propose | `ito-proposal` |
| investigate or synthesize | `ito-research` |
| implement an accepted proposal | `ito-apply` |
| review or verify | `ito-review` |
| archive and promote accepted specs | `ito-archive` |
| iterate, Ralph, or orchestrate | `ito-loop` |

Parse the first intent and preserve every remaining argument unchanged. Route only to this fixed table. If the matching retained skill is missing, report an installation error and recommend `ito init --upgrade`; do not silently execute a different phase.

Helper-shaped requests are phase intents, not separate skills: feature/fix/intake/planning go to `ito-proposal`; worktrees/tasks/commits go to `ito-apply`; testing/verification go to `ito-review`; finish goes to `ito-archive`; orchestration goes to `ito-loop`.

Use direct CLI fallback for operational commands such as `ito list`, `ito path`, `ito config`, `ito status`, `ito validate`, `ito update`, and `ito plan init|status`. Repository cleanup guidance comes from `ito agent instruction cleanup`; managed refresh uses `ito init --upgrade`. Preserve CLI argument order and errors.

An explicit retired skill request receives a short replacement explanation using the lifecycle table. Tmux integration was removed and has no Ito replacement. There is no wildcard skill discovery, filesystem cache, or dynamically constructed `ito-*` activation.

For first-run project orientation, render `ito agent instruction project-setup`. User-authored skills remain outside the Ito-managed inventory.
<!-- ITO:END -->
