---
name: ito-orchestrate
description: Coordinate multi-change runs with gates, run state, and remediation.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->


Coordinate applying changes across a repo with an orchestrator role.

## Goal

Provide a coordinator workflow that:

- Reads project policy from `.ito/user-prompts/orchestrate.md`
- Builds a dependency-aware plan from per-change `.ito.yaml` metadata
- Executes a consistent gate pipeline per change
- Persists run state under `.ito/.state/orchestrate/runs/<run-id>/` for resumability

## Key Rules

- The orchestrator MUST NOT write code itself. It only coordinates workers.
- Objective gates (format/lint/tests) MUST run before reviewer gates (code-review/security-review).
- Always write run state before moving to the next gate.

## Definitions

**Dependency between changes.** Declared explicitly in each change's
`.ito/changes/<id>/.ito.yaml` under `orchestrate.depends_on`:

```yaml
orchestrate:
  depends_on:
    - 001-02_other-change
  preferred_gates:
    - apply-complete
    - tests
    - code-review
```

A change is not dispatched until all declared dependencies have completed their
pipelines. Circular dependencies are rejected at plan-build time.

**Gate.** One stage in a change's pipeline. The canonical default order is:

1. `apply-complete` — verify the apply worker finished its work
2. `format` — project formatter (e.g. `cargo fmt --check`)
3. `lint` — project linter (e.g. `cargo clippy -- -D warnings`)
4. `tests` — project test suite
5. `style` — style/consistency checks (often skill-driven)
6. `code-review` — reviewer agent pass for correctness
7. `security-review` — reviewer agent pass for security concerns

Gates can be overridden by the active preset (`.ito/user-prompts/orchestrate.md`
`preset:`) or per-change via `orchestrate.preferred_gates` in `.ito.yaml`.

## Steps

1. Ensure orchestrator setup exists:
   - If `.ito/user-prompts/orchestrate.md` is missing, run `ito agent instruction orchestrate` and follow the setup guidance (it points to the `ito-orchestrate-setup` skill).

2. Render and follow the orchestrator instruction:
   - Run: `ito agent instruction orchestrate`
   - Follow the gate order and policy described in the rendered instruction.

3. Track state under `.ito/.state/orchestrate/runs/<run-id>/`:
   - `run.json` — run metadata: `run_id`, `started_at`, `finished_at`, `status` (`running`/`complete`/`failed`), `preset`, `max_parallel`, `failure_policy`.
   - `plan.json` — the resolved execution plan: dependency-ordered list of changes with their gate pipeline and policy per gate (`run`/`skip`).
   - `events.jsonl` — append-only log of lifecycle events (`run-start`, `gate-start`, `gate-pass`, `gate-fail`, `gate-skip`, `worker-dispatch`, `worker-complete`, `remediation-dispatch`, `run-complete`). Never truncated during a run; used for audit and recovery.
   - `changes/<change-id>.json` — terminal gate outcomes observed so far for the change, plus the `updated_at` timestamp. Used for O(1) resume lookup without replaying the full event log.

4. Remediate on failure:
   - On a gate failure, construct a **remediation packet** containing:
     - `change_id` — the canonical change id
     - `failed_gate` — the gate that failed
     - `error` — the error payload captured from the gate
     - `rerun_gates` — the failed gate plus all downstream gates that are still scheduled to run
   - Dispatch the packet to a **fresh apply worker** (a new worker agent session; do not reuse the one that failed).
   - After remediation, rerun only the failed gate and its downstream gates. Previously passed gates are not rerun.
   - Respect `failure_policy` from `orchestrate.md`: `remediate` (default), `stop`, or `continue`.

<!-- ITO:END -->
