## Why

The backend can receive events, but without a forwarding path clients will continue to write audit/state events only locally, limiting cross-session observability and making server-side coordination harder to diagnose.

## What Changes

- Add a backend-forwarding workflow that submits locally produced audit/state events to the backend event ingest endpoint.
- Add idempotent batching and retry behavior so forwarding is safe under transient failures.
- Add minimal CLI-visible diagnostics for forwarding success/failure in backend mode.

## Capabilities

### New Capabilities

- `backend-event-forwarding`: Client-side forwarding of locally produced events to the backend with idempotent retries.

### Modified Capabilities

- (none)

## Impact

- **Observability**: Enables centralized event timelines across harness sessions.
- **Reliability**: Requires careful idempotency and retry limits to avoid duplicate amplification.
- **Dependencies**: Depends on backend event ingest endpoints from `024-01_add-shared-state-api`.
