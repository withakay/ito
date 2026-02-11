## Context

Ito currently stores change proposals in the main repository history, which works for single-agent flows but creates contention when multiple agents create or update changes concurrently. The user asked for a deliberately private, system-owned branch that is synchronized frequently and defaults to direct push. The branch name is settled as `ito/internal/changes` and should be mostly invisible during normal usage.

The design must respect enterprise constraints where direct pushes to `main` are blocked. It also needs to avoid disruptive UX (branch switching in user workspace, accidental staging, or noisy prompts).

## Goals / Non-Goals

**Goals:**

- Establish `ito/internal/changes` as the default coordination branch for change metadata.
- Sync before/after critical operations (`create change`, `agent instruction apply`, and task-start entry points).
- Keep behavior configurable (`use_change_branch`, branch name override, optional opt-out).
- Keep synchronization transparent to users and deterministic under contention.

**Non-Goals:**

- Replacing normal implementation branch and PR workflows.
- Guaranteeing global locking semantics across disconnected/offline environments.
- Solving every merge conflict automatically.

## Decisions

- **Decision: Use a dedicated direct-push coordination branch by default.**
  - Rationale: Fast reservation of IDs and reduced collision probability.
  - Alternative considered: use `main` as source-of-truth. Rejected due to branch protection and governance constraints.

- **Decision: Default branch name is `ito/internal/changes`.**
  - Rationale: clearly tool-owned, intentional namespace, low accidental user overlap.
  - Alternative considered: punctuation-heavy names (for example `__ito-changes__`). Rejected for readability and long-term ergonomics.

- **Decision: Sync without changing the user's active branch/worktree.**
  - Rationale: preserves developer context and trust.
  - Alternative considered: checkout/switch workflow. Rejected as intrusive and error-prone in dirty worktrees.

- **Decision: Fail fast with clear remediation on non-fast-forward and protection errors.**
  - Rationale: predictable behavior is better than hidden retries that can mask conflicts.
  - Alternative considered: aggressive auto-retry with force-like semantics. Rejected as unsafe.

## Risks / Trade-offs

- **Concurrent edits to same proposal path** -> Manual conflict resolution still required; document clear resolution flow.
- **Remote rejects direct push** -> Feature loses reservation guarantee; provide actionable error and configurable fallback.
- **Frequent sync overhead** -> Slight latency increase; bound sync scope to required operations only.
- **Offline creation drift** -> Reconciliation may conflict; require sync + revalidation before continuing.

## Migration Plan

1. Add config schema/defaults for coordination branch behavior.
2. Implement git sync primitives and wire into change create/apply/task-start flows.
3. Add integration tests for default path, conflict path, protection failure, and disabled mode.
4. Roll out with default enabled; allow opt-out by configuration.

Rollback:

- Disable `use_change_branch` globally to restore existing behavior without removing code.

## Open Questions

- Should task-start synchronization happen for every `ito tasks start` invocation, or only when entering a new change session?
- Should apply/task-start failures hard-stop when sync fails, or allow explicit override flags for emergency local-only workflows?
