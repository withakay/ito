# Tasks for: 024-01_add-shared-state-api

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-01_add-shared-state-api
ito tasks next 024-01_add-shared-state-api
ito tasks start 024-01_add-shared-state-api 1.1
ito tasks complete 024-01_add-shared-state-api 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Finalize OpenAPI contract and endpoint shapes

- **Files**: `ito-rs/crates/ito-web/`, `docs/`, `.ito/changes/024-01_add-shared-state-api/specs/`
- **Dependencies**: None
- **Action**: Define the `v1` OpenAPI resources for projects, changes, leases, allocation, artifacts, and event ingest, including request/response schemas and error envelopes.
- **Verify**: `make check`
- **Done When**: OpenAPI contract is versioned, complete for v1 scope, and aligned with spec deltas.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 1.2: Define backend persistence model for v1 entities

- **Files**: `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-domain/`, backend migration files
- **Dependencies**: Task 1.1
- **Action**: Define storage schema and repository interfaces for projects, change leases, artifact revisions, allocation metadata, and ingested events.
- **Verify**: `make check`
- **Done When**: Domain and core persistence contracts exist for all v1 entities with migration plan documented.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement auth and project-scoping middleware

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-config/`, `ito-rs/crates/ito-core/`
- **Dependencies**: None
- **Action**: Implement bearer token validation, project-scope authorization checks, and config/env token resolution (`ITO_BACKEND_TOKEN` by default).
- **Verify**: `cargo test -p ito-web`
- **Done When**: Protected endpoints reject missing/invalid/mismatched tokens and accept valid project-scoped tokens.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 2.2: Implement change lease lifecycle endpoints

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-domain/`
- **Dependencies**: Task 2.1
- **Action**: Implement acquire, heartbeat, and release endpoints with TTL-based expiration handling and conflict responses.
- **Verify**: `cargo test -p ito-web lease`
- **Done When**: Only one active lease exists per change, heartbeats renew owner leases, and expired leases are reclaimed.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Implement atomic change allocation endpoint

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`
- **Dependencies**: None
- **Action**: Implement "claim next available change" with atomic lease creation and idempotency-key retry behavior.
- **Verify**: `cargo test -p ito-web allocation`
- **Done When**: Allocation either returns one leased change or a no-work response, and idempotent retries do not duplicate leases.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 3.2: Implement markdown artifact bundle read/write endpoints

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`, persistence adapters
- **Dependencies**: Task 3.1
- **Action**: Add artifact bundle read endpoint and revision-checked artifact write endpoint with conflict payloads.
- **Verify**: `cargo test -p ito-web artifacts`
- **Done When**: Clients can pull all authored artifacts for a change and push updates with optimistic concurrency conflict handling.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Implement event ingest endpoint with idempotency

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-domain/`
- **Dependencies**: None
- **Action**: Implement authenticated batch event ingest with schema validation, append-only persistence, and idempotency-key deduplication.
- **Verify**: `cargo test -p ito-web events`
- **Done When**: Valid event batches persist once, retries with same idempotency key do not duplicate, and invalid payloads fail cleanly.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 4.2: Add end-to-end tests and rollout docs

- **Files**: `ito-rs/crates/ito-web/tests/`, `docs/`, `.ito/specs/` (as needed)
- **Dependencies**: Task 4.1
- **Action**: Add integration tests for auth, leasing, allocation, artifact sync, and event ingest flows; document operational setup and failure modes.
- **Verify**: `make check`
- **Done When**: Test suite covers happy path and conflict/error paths, and docs describe v1 backend setup and migration toggles.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Implement backend archive endpoint and archived status field

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`
- **Dependencies**: None
- **Action**: Implement change archive operation and ensure change reads include archived status.
- **Verify**: `cargo test -p ito-web archive`
- **Done When**: Backend can mark a change archived, and subsequent reads return archived status.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 5.2: Enforce immutability rules for archived changes

- **Files**: `ito-rs/crates/ito-web/`, `ito-rs/crates/ito-core/`
- **Dependencies**: Task 5.1
- **Action**: Reject artifact writes, lease operations, and allocations for archived changes.
- **Verify**: `cargo test -p ito-web archive-immutability`
- **Done When**: Archived changes are read-only and excluded from work allocation.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
