# Tasks for: 024-05_add-backend-archive-sync

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-05_add-backend-archive-sync
ito tasks next 024-05_add-backend-archive-sync
ito tasks start 024-05_add-backend-archive-sync 1.1
ito tasks complete 024-05_add-backend-archive-sync 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement backend-aware archive orchestration in core

- **Files**: `ito-rs/crates/ito-core/`
- **Dependencies**: None
- **Action**: Add an archive flow for backend mode that pulls final artifacts, runs existing archive logic, and then marks backend archived.
- **Verify**: `cargo test -p ito-core archive`
- **Done When**: Core exposes a tested backend-mode archive orchestration API.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

### Task 1.2: Wire backend-mode archive into CLI command

- **Files**: `ito-rs/crates/ito-cli/src/app/archive.rs`, `ito-rs/crates/ito-cli/src/runtime.rs`
- **Dependencies**: Task 1.1
- **Action**: When backend mode is enabled, route `ito archive` through the backend-mode orchestration and print a post-archive commit reminder.
- **Verify**: `cargo test -p ito-cli archive`
- **Done When**: CLI archive behavior matches spec for backend mode and filesystem mode remains unchanged.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add integration tests for backend archive path

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**: Add integration tests for happy path and backend unavailable path; assert repo state is committable.
- **Verify**: `make check`
- **Done When**: Tests cover backend-mode archive end-to-end.
- **Updated At**: 2026-02-28
- **Status**: [ ] pending
