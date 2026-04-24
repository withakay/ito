---
preset: generic
max_parallel: auto
failure_policy: remediate
gate_overrides: {}
---

<!-- ITO:START -->

# Orchestrate Guidance

This file configures `ito agent instruction orchestrate`.

- Ito may update this header block over time.
- Put strict policies in `## MUST`.
- Put preferences in `## PREFER`.

<!-- ITO:END -->

## MUST

- The orchestrator MUST NOT write code itself; it only coordinates workers.

## PREFER

- Prefer objective gates before reviewer gates.

## Notes

- Add any additional context that helps the orchestrator coordinate work.

<!-- ITO:INTERNAL:START -->
## Your Orchestrate Guidance

(Add orchestrator-specific guidance here: presets, parallelism policy, gate overrides, worker agent suggestions, and workflow conventions.)
<!-- ITO:INTERNAL:END -->
