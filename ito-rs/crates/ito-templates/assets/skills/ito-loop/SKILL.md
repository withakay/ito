---
name: ito-loop
description: Run an ito ralph loop for a change, module, or repo-ready sequence, with safe defaults and automatic restart context on early exits.
---

<!-- ITO:START -->
# Iteration and orchestration lifecycle

Keep Ralph available by default for one change, one module, or the next ready work item. Parse identifiers with `ito util parse-id`; quote all parsed input and never use `eval`.

Build one base command with bounded defaults: five iterations, a 15-minute timeout, and at most two outer restarts for restartable early exits.

```bash
ito ralph --no-interactive --harness <harness> --change <change-id> --max-iterations 5 --timeout 15m
ito ralph --no-interactive --harness <harness> --module <module-id> --max-iterations 5 --timeout 15m
ito ralph --no-interactive --harness <harness> --continue-ready --max-iterations 5 --timeout 15m
```

Do not wrap Ralph in an unbounded outer loop. Add restart context only when `ito ralph --status` and `ito tasks status` provide meaningful recovery evidence. Report the final status after success or bounded exhaustion.

For multi-change orchestration, render and follow:

```bash
ito agent instruction orchestrate
```

Treat `.ito/user-prompts/orchestrate.md` as additive project policy. Preserve dependency and gate order, run state, coordinator-only responsibilities, remediation, and resume semantics. Missing project guidance triggers inline setup from the same authoritative instruction; no setup or workflow skill is generated.
<!-- ITO:END -->
