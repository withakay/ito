---
name: ito-loop
description: Run an ito ralph loop for a change, module, or repo-ready sequence, with safe defaults and automatic restart context on early exits.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.32-->
# Iteration and orchestration lifecycle

Keep Ralph available by default for one change, one module, or the next ready work item. Parse identifiers with `ito util parse-id`; quote parsed input and never use `eval`.

Before change-mode iteration, resolve the guarded worktree with `ito worktree ensure --change "<change-id>"`, enter it, and run `ito change preflight "<change-id>" --for execute`. For module and ready-work modes, let Ralph apply the same gate after each dynamic selection. Treat readiness failures as terminal for the attempt.

Build one command with bounded defaults: five iterations, a 15-minute timeout, and at most two outer restarts for restartable early exits.

```bash
ito ralph --no-interactive --harness <harness> --change <change-id> --max-iterations 5 --timeout 15m
ito ralph --no-interactive --harness <harness> --module <module-id> --max-iterations 5 --timeout 15m
ito ralph --no-interactive --harness <harness> --continue-ready --max-iterations 5 --timeout 15m
```

Do not wrap Ralph in an unbounded loop. Add restart context only when `ito ralph --status` and `ito tasks status` provide meaningful recovery evidence. Report final status after success or bounded exhaustion.

For multi-change orchestration, render and follow:

```bash
ito agent instruction orchestrate
```

Treat `.ito/user-prompts/orchestrate.md` as additive project policy. Preserve dependency and gate order, run state, coordinator-only responsibilities, remediation, and resume semantics. Missing project guidance triggers inline setup from the same authoritative instruction; no setup or workflow skill is generated.
<!-- ITO:END -->
