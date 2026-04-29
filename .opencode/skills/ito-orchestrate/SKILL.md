---
name: ito-orchestrate
description: Coordinate multi-change runs with gates, run state, and remediation.
---

<!-- ITO:START -->

Coordinate multi-change runs by delegating workflow policy to Ito's rendered orchestrate instruction.

## Steps

1. Run `ito agent instruction orchestrate`.
2. Follow the rendered instruction exactly, including setup guidance, run-state rules, gates, and remediation policy.
3. After consulting the rendered instruction, load `ito-orchestrator-workflow` when repo-specific commands, services, reviewer expectations, or gotchas are relevant.
4. Keep the orchestrator coordinator-only: dispatch implementation or remediation to worker agents instead of editing code directly.

Do not duplicate gate order, run-state schema, or remediation details here; those belong in `ito agent instruction orchestrate`.

<!-- ITO:END -->
