# Tasks for: 024-04_add-backend-event-forwarding

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-04_add-backend-event-forwarding
ito tasks next 024-04_add-backend-event-forwarding
ito tasks start 024-04_add-backend-event-forwarding 1.1
ito tasks complete 024-04_add-backend-event-forwarding 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement core event forwarder with idempotent batching

- **Files**: `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-domain/`
- **Dependencies**: None
- **Action**: Implement forwarder that reads local events, submits batches to backend ingest with idempotency keys, and records an offset/checkpoint.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Forwarder submits once per batch and does not duplicate on retry.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 1.2: Wire forwarder into backend-mode CLI lifecycle

- **Files**: `ito-rs/crates/ito-cli/`, `ito-rs/crates/ito-core/`
- **Dependencies**: Task 1.1
- **Action**: Run forwarder as part of backend-mode command completion (best-effort) and surface minimal diagnostics.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Backend-mode commands attempt forwarding without breaking primary command success.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add integration tests and OpenAPI linkage docs

- **Files**: `ito-rs/crates/ito-core/tests/`, `docs/`
- **Dependencies**: None
- **Action**: Add integration tests for success, transient retry, and invalid payload paths; document how forwarding relates to backend ingest.
- **Verify**: `make check`
- **Done When**: Tests cover forwarding behavior and docs describe troubleshooting.
- **Updated At**: 2026-02-28
- **Status**: [x] complete
