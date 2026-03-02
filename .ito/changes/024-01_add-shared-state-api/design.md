## Context

Ito workflows currently treat markdown files plus git history as the shared system of record. In single-harness workflows this is acceptable, but in parallel workflows each harness can hold a stale worktree snapshot, causing ownership and task-state conflicts. The requested direction is API-canonical state with markdown artifacts persisted on the server so clients can pull/push through a consistent interface.

The immediate objective is a backend contract that can coordinate changes first (ownership and artifact sync), then extend to tasks and richer state. The service must remain compatible with existing Ito domain layering: domain traits stay pure, core provides adapters, and CLI/harness integration follows in subsequent module changes.

The backend is not inherently local-only. While v1 may often be run as a local service for a single developer machine, the contract assumes the backend can run anywhere reachable by the client (for example, on a different host inside a private network). Client configuration must therefore treat the backend as an arbitrary base URL, not a fixed loopback server.

## Goals / Non-Goals

**Goals:**

- Define and implement a versioned OpenAPI contract for project-scoped state management.
- Guarantee exclusive change ownership using lease + heartbeat semantics.
- Support markdown artifact round-trip (read/write) with optimistic concurrency.
- Provide atomic work allocation for "claim next available change".
- Accept event ingest payloads for audit/state telemetry.
- Keep authentication simple for v1: project-scoped bearer token.

**Non-Goals:**

- Full replacement of local git workflows in this change.
- Fine-grained per-user RBAC or OIDC federation.
- Real-time collaborative editing/merge UI for markdown content.
- Task-level scheduling optimization across all changes (handled in later change).

## Decisions

- Decision: API-canonical state for shared workflow entities.
  - Rationale: Eliminates stale-snapshot coordination failures that git alone cannot solve for distributed harness sessions.
  - Alternative: Git-canonical with lock metadata only. Rejected because artifact reads still drift across sessions.

- Decision: Change ownership uses leases (TTL) plus heartbeat renewals.
  - Rationale: Auto-recovers from crashed/abandoned sessions without manual cleanup.
  - Alternative: Permanent manual locks. Rejected due to operational deadlocks and support burden.

- Decision: Artifacts stored as markdown blobs with per-artifact revision numbers.
  - Rationale: Preserves current authoring model while enabling optimistic concurrency (`If-Match`/revision checks).
  - Alternative: Structured JSON task/change models only. Rejected for v1 to avoid schema migration of existing markdown workflows.

- Decision: Single project-scoped bearer token for v1 auth.
  - Rationale: Minimal rollout complexity and aligns with env-var-based harness configuration.
  - Alternative: Per-agent tokens or OIDC. Deferred to a follow-up once base workflow is stable.

- Decision: Work allocation endpoint claims changes, not tasks.
  - Rationale: Matches desired ownership granularity and minimizes lock contention.
  - Alternative: Cross-change task allocator. Deferred until change-level coordination is proven.

- Decision: Event ingest endpoint is append-only and idempotent.
  - Rationale: Allows ingestion from multiple harnesses without duplicate amplification.
  - Alternative: Mutating event records. Rejected to preserve audit semantics.

- Decision: Local backend persistence (when used) lives outside the repo.
  - Rationale: keeps the git working tree clean and allows the backend to maintain durable state and backups even when the repo is moved, deleted, or checked out in multiple worktrees.
  - Alternative: store backend data under `.ito/.state/`. Rejected because it couples server durability to repo checkout state.

## Risks / Trade-offs

- [Backend outage blocks coordination] -> Provide local fallback mode and clear degraded-state diagnostics.
- [Lease TTL too short causes accidental ownership loss] -> Use conservative default TTL and explicit heartbeat cadence in clients.
- [Concurrent artifact edits create frequent conflicts] -> Return structured 409 conflicts with latest revision and server copy for automated retry.
- [Single token model has coarse authorization] -> Limit token scope per project and rotate tokens; plan per-agent scopes in follow-up.
- [Data drift between backend and git] -> Define a deterministic sync contract and require explicit push/pull boundaries.

## Migration Plan

1. Add backend service with OpenAPI docs and persistence schema.
2. Implement project registration/bootstrap and token validation.
3. Implement change lease + allocation endpoints.
4. Implement artifact read/write endpoints with revision checks.
5. Implement event ingest endpoint with idempotency keys.
6. Add CLI/client integration behind feature/config gate in follow-up change.
7. Roll out per project in opt-in mode; keep filesystem mode as default fallback.

Rollback: disable backend configuration (`backend.enabled=false`) and return clients to existing filesystem/git workflows; backend data remains intact for later recovery.

## Open Questions

- Should v1 backend persistence be SQLite (single-node simplicity) or Postgres (production-ready concurrency) as default?
- Should leases support a privileged force-release endpoint in v1 or defer until admin model exists?
- Should artifact updates be whole-file only, or support patch-based writes in a later change?
- Should event ingest support server fan-out (webhooks/streams) in this module or separate observability module?
