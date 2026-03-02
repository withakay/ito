## Why

Defining a backend API is not enough to solve multi-agent drift unless the CLI and harness flows actually use it for allocation, claiming, and artifact sync. We need a backend-aware client mode in Ito so agents can consistently pull fresh change state, lock ownership, and push updates through a single coordination path.

## What Changes

- Add a backend client runtime in Ito that can call the shared-state API when backend mode is enabled.
- Add explicit command UX for backend coordination under `ito tasks`: `claim`, `release`, and `allocate`.
- Add explicit sync command UX under `ito tasks sync`: `pull` and `push` for markdown artifact round-trips with revision conflict handling.
- Add repository adapter support so change and task reads can resolve from backend state in backend mode.
- Add retry/idempotency handling for allocation and sync calls to keep CLI operations safe across transient failures.
- Keep filesystem mode as a supported fallback when backend mode is disabled.

## Capabilities

### New Capabilities

- `backend-client-runtime`: Backend API client initialization, request lifecycle, and resilience behavior.
- `backend-change-claim`: CLI-facing change claim/release flow using backend leases.
- `backend-change-sync`: CLI artifact pull/push synchronization contract with revision-aware conflict handling.

### Modified Capabilities

- `change-repository`: Add backend-backed repository behavior when backend mode is enabled.
- `task-repository`: Add backend-backed task access and update pathways when backend mode is enabled.
- `cli-tasks`: Add backend-aware task mutation behavior while preserving deterministic task ordering output.
- `config`: Add backend mode selection and runtime resolution behavior across config and environment variables.

## Impact

- **Affected code**: `ito-cli`, `ito-core`, `ito-domain`, and config/runtime wiring.
- **Affected workflows**: agent assignment, claim/release, and markdown sync during active work.
- **Dependencies**: relies on the backend API contract and auth model from `024-01_add-shared-state-api`.
- **Operational impact**: introduces backend connectivity/error handling paths in normal CLI execution.
