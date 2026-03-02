## Context

Ito already has append-only local event streams (execution logs, and optionally audit events). The backend introduces a project-scoped event ingest API but does not yet receive those client-side events automatically.

This change defines a forwarding workflow that reads new local events and posts them to the backend using idempotency keys and bounded retries.

## Goals / Non-Goals

**Goals:**

- Forward locally produced events to the backend in backend mode.
- Ensure forwarding is idempotent under retries.
- Provide minimal user-facing diagnostics when forwarding is failing.

**Non-Goals:**

- Real-time streaming transport (websockets) or server fan-out.
- Replacing local event logs.
- Complex filtering, enrichment, or analytics queries.

## Decisions

- Decision: Forward in bounded batches with idempotency keys.
  - Rationale: allows retry without duplicating events.

- Decision: Track the last forwarded offset per project locally.
  - Rationale: avoids re-sending the entire log each time.

## Risks / Trade-offs

- [Offset tracking drift] -> Treat forwarding as best-effort; allow manual reset/replay later.
- [Duplicate events] -> Use server-side idempotency keys plus client-side offsets.
- [Backpressure] -> Bound batch size and retry counts.

## Migration Plan

1. Define event-forwarding config defaults and offset file location under `.ito/.state/`.
2. Implement forwarder in core and wire into backend-mode command lifecycle.
3. Add integration tests with a fake backend ingest endpoint.
