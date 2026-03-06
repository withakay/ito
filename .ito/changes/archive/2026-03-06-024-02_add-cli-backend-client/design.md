## Context

`024-01_add-shared-state-api` defines the server-side contract (API canonical state, change leases, artifact storage, and event ingest). This change wires Ito clients to that contract so normal CLI/harness workflows can coordinate through the backend instead of relying on stale worktree snapshots.

The design must preserve the existing filesystem-first architecture and layered boundaries. `ito-domain` owns traits and domain models, `ito-core` owns adapters and orchestration, and `ito-cli` remains a thin command adapter. Backend mode is opt-in and must fail predictably without corrupting local authored artifacts.

## Goals / Non-Goals

**Goals:**

- Add a backend client runtime that is configurable and testable.
- Support claiming and releasing a change through lease endpoints.
- Support artifact pull and push with revision conflict detection.
- Keep task and change reads compatible with existing repository abstractions.
- Preserve deterministic CLI output semantics for existing commands.

**Non-Goals:**

- Replacing every Ito command with backend behavior in this change.
- Task-level global scheduler or auto-balancer across changes.
- Real-time streaming/long-lived websocket sync.

## Decisions

- Decision: Gate backend behavior behind `backend.enabled` and runtime config resolution.
  - Rationale: allows staged rollout and safe fallback to filesystem mode.
  - Alternative: auto-detect backend by URL presence. Rejected as too implicit.

- Decision: Introduce backend-aware repository adapters in `ito-core` that implement existing repository traits.
  - Rationale: keeps domain API stable and avoids command-specific HTTP logic in `ito-cli`.
  - Alternative: direct HTTP calls from commands. Rejected due to layering violations.

- Decision: Claim/release flows are explicit CLI operations, with allocation support exposed via a dedicated command path.
  - Rationale: keeps ownership intent explicit and auditable.
  - Alternative: implicit claiming on any task mutation. Deferred until command UX is proven.

- Decision: Backend coordination reuses the existing `tasks` command group rather than adding a new top-level command.
  - Rationale: preserves the stable top-level CLI surface while making backend workflows discoverable where agents already manage work.
  - Alternative: introduce a new `backend` or `sync` top-level group. Rejected to avoid command-surface expansion.

- Decision: Command names are fixed for v1 backend mode:
  - `ito tasks claim <change-id>`
  - `ito tasks release <change-id>`
  - `ito tasks allocate`
  - `ito tasks sync pull <change-id>`
  - `ito tasks sync push <change-id>`
  - Rationale: verbs are short, action-oriented, and map directly to lease/allocation/sync API semantics.
  - Alternative: `checkout`, `lock`, `publish`, `fetch`. Rejected as less precise for lease and revision semantics.

- Decision: Artifact writes use optimistic concurrency with conflict surfacing.
  - Rationale: backend is canonical, so stale writes must fail safely with actionable diagnostics.
  - Alternative: last-write-wins. Rejected because it silently loses updates.

- Decision: Retries use idempotency keys for allocation and push operations.
  - Rationale: avoids duplicate claims or duplicate writes under transient network failure.
  - Alternative: blind retries. Rejected as unsafe.

## Risks / Trade-offs

- [Backend latency degrades command UX] -> Add bounded request timeouts and concise retry policy.
- [Backend outage blocks backend mode commands] -> Provide clear fallback guidance and deterministic failure codes.
- [Conflict frequency increases with parallel edits] -> Return structured conflict payloads and suggest pull-retry flow.
- [Mode divergence causes user confusion] -> Print active mode (`filesystem` vs `backend`) in status/help surfaces.

## Migration Plan

1. Add backend config resolution and client factory in core runtime.
2. Implement backend repository adapters for change/task reads and updates.
3. Add CLI command surface for claim/release/allocate and sync flows.
4. Integrate backend mode into `ito tasks` mutation paths with conflict handling.
5. Add integration tests for happy path, lease conflict, stale revision, and backend unavailable cases.

Rollback: disable `backend.enabled` and continue filesystem mode; no destructive migration is required for existing markdown files.

## Open Questions

- Should pull/push be manual commands only in v1, or also auto-run around selected task mutations?
- Which failure classes should trigger automatic retry versus immediate user-visible failure?
