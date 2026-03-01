# Tasks for: 024-02_add-cli-backend-client

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-02_add-cli-backend-client
ito tasks next 024-02_add-cli-backend-client
ito tasks start 024-02_add-cli-backend-client 1.1
ito tasks complete 024-02_add-cli-backend-client 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add backend runtime configuration and client factory

- **Files**: `ito-rs/crates/ito-config/`, `ito-rs/crates/ito-core/`
- **Dependencies**: None
- **Action**: Add resolved backend runtime settings and a client factory that is only created when backend mode is enabled.
- **Verify**: `cargo test -p ito-config && cargo test -p ito-core`
- **Done When**: Backend runtime config resolves predictably and client factory can be constructed in tests.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 1.2: Define backend client interfaces in domain/core boundaries

- **Files**: `ito-rs/crates/ito-domain/`, `ito-rs/crates/ito-core/`
- **Dependencies**: Task 1.1
- **Action**: Add backend client traits and DTO mapping needed for claim, allocation, and artifact sync operations.
- **Verify**: `cargo test -p ito-domain && cargo test -p ito-core`
- **Done When**: Layer boundaries remain clean and backend operations are mockable in unit tests.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement backend-backed change and task repository adapters

- **Files**: `ito-rs/crates/ito-core/src/changes/`, `ito-rs/crates/ito-core/src/tasks/`
- **Dependencies**: None
- **Action**: Implement repository adapters that read change/task state from backend when backend mode is enabled.
- **Verify**: `cargo test -p ito-core repository`
- **Done When**: Change and task repository calls resolve from backend in backend mode and filesystem mode remains intact.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 2.2: Add pull/push artifact sync service with revision conflict handling

- **Files**: `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-common/`
- **Dependencies**: Task 2.1
- **Action**: Implement artifact bundle pull/push orchestration, structured stale-revision conflict reporting, and timestamped local backup snapshots under `backend.backup_dir`.
- **Verify**: `cargo test -p ito-core sync`
- **Done When**: Pull writes local artifacts with revision metadata and push fails safely on stale revisions.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add CLI claim/release/allocate command surface for backend mode

- **Files**: `ito-rs/crates/ito-cli/src/commands/`, `ito-rs/crates/ito-cli/src/runtime.rs`
- **Dependencies**: None
- **Action**: Add command handlers for `ito tasks claim <change-id>`, `ito tasks release <change-id>`, and `ito tasks allocate` in backend mode.
- **Verify**: `cargo test -p ito-cli claim && cargo test -p ito-cli allocate`
- **Done When**: Commands call backend services, print deterministic output, and surface lease conflicts clearly.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 3.2: Integrate backend sync into task mutation command path

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-core/`
- **Dependencies**: Task 3.1
- **Action**: Wire `ito tasks sync pull <change-id>` and `ito tasks sync push <change-id>` plus task mutations to backend sync in backend mode while preserving existing ordering and status behavior.
- **Verify**: `cargo test -p ito-cli tasks`
- **Done When**: Task mutations in backend mode persist through backend sync and conflict conditions fail with actionable guidance.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Add integration coverage for backend client mode

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**: Add integration tests for claim success/conflict, allocate no-work, pull/push success, and stale revision conflict.
- **Verify**: `make check`
- **Done When**: Backend mode behavior is covered by end-to-end tests with deterministic assertions.
- **Updated At**: 2026-02-28
- **Status**: [x] complete

### Task 4.2: Document backend client mode usage and failure recovery

- **Files**: `docs/`, `.ito/user-prompts/` (if needed)
- **Dependencies**: Task 4.1
- **Action**: Document how to enable backend mode, claim/release workflow, sync flow, and conflict recovery steps.
- **Verify**: `make check`
- **Done When**: Documentation covers setup, normal usage, and troubleshooting for backend mode.
- **Updated At**: 2026-02-28
- **Status**: [x] complete
