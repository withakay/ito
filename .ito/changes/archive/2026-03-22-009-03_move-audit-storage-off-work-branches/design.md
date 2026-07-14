## Context

Ito currently treats `.ito/.state/audit/events.jsonl` on the working branch as the durable audit source of truth, and an earlier archived proposal explored optional mirroring to a dedicated internal branch. That no longer fits the desired operational model: audit state is machine-written, grows quickly, and should not pollute normal feature history. At the same time, validation and reconciliation still need a durable append-only stream, and backend mode already has a project-scoped server-side audit sink via event ingest.

## Goals / Non-Goals

**Goals:**

- Stop writing tracked audit JSONL onto normal user-facing working branches.
- Make backend mode write audit state only to backend/server-managed storage.
- In local mode, persist audit history into an internal Ito branch/repository when available.
- Preserve audit validation, reconciliation, and read semantics across the new storage routing.
- Provide a fallback path for non-worktree or non-branch-capable environments without reintroducing tracked branch pollution.

**Non-Goals:**

- Redesigning the audit event schema itself.
- Solving long-term compaction/retention for all audit storage backends in this change.
- Turning backend audit storage into a generic telemetry/event warehouse.

## Decisions

- **Decision: Backend mode becomes server-only for audit writes.**
  - Rationale: once the backend is authoritative, duplicating tracked local JSONL adds noise and drift risk without improving correctness.
  - Alternative considered: keep a tracked local copy for parity. Rejected because it preserves the branch-pollution problem the change is trying to solve.

- **Decision: Local durable audit history moves to an internal Ito branch/repository, not the current work branch.**
  - Rationale: this keeps append-only machine state out of proposal/feature history while preserving a git-backed durable log.
  - Alternative considered: leave the current branch as the source of truth and mirror elsewhere. Rejected because the current branch remains polluted even if mirroring exists.

- **Decision: Validation/reconciliation read through an audit storage abstraction instead of assuming `.ito/.state/audit/events.jsonl` on disk.**
  - Rationale: the command surface should not care whether events came from backend storage, internal-branch storage, or a local fallback cache.
  - Alternative considered: special-case every audit command by mode. Rejected because it spreads storage logic across many CLI paths.

- **Decision: When internal-branch persistence is unavailable, use a local untracked fallback rather than a tracked working-branch file.**
  - Rationale: the user requirement is to avoid commit pollution first; graceful degradation is still better than failing every local command.
  - Alternative considered: fail closed when the internal branch cannot be used. Rejected because it would make local/offline workflows fragile.

## Risks / Trade-offs

- **Internal branch plumbing is harder without worktrees** -> implement through git plumbing/index isolation and keep fallback behavior explicit.
- **Audit commands may get slower when reading indirect storage** -> add storage-specific readers and keep append/read APIs narrow.
- **Fallback cache can diverge from internal branch or backend** -> treat fallback as best-effort/local-only and surface diagnostics when active.
- **Existing tracked JSONL users need migration** -> add migration steps and explicit spec changes for where the authoritative stream now lives.

## Migration Plan

1. Introduce an audit storage abstraction that supports backend, internal-branch, and fallback-local backends.
2. Change audit writes to route by mode: backend-only in backend mode; internal-branch/fallback in local mode.
3. Update readers/validators/reconcilers to consume the routed audit backend.
4. Add migration behavior so existing `.ito/.state/audit/events.jsonl` data can be imported into the new durable store.
5. Update docs and operational guidance, then stop tracking the working-branch audit file.

Rollback:

- Re-enable the legacy working-branch writer behind a guarded compatibility mode if the internal-branch or backend routing proves operationally unsafe.

## Open Questions

- Should the internal local durable store live on `ito/internal/audit`, `ito/internal/changes`, or a separate bare Ito-side repository branch family?
- Should migration of existing tracked audit files happen automatically on first run, or via an explicit maintenance command?
