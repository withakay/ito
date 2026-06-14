# Tasks for: 030-03_coordination-branch-sync

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-03_coordination-branch-sync
ito tasks next 030-03_coordination-branch-sync
ito tasks start 030-03_coordination-branch-sync 1.1
ito tasks complete 030-03_coordination-branch-sync 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define coordination sync tests

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for missing branch, clean branch, remote-ahead branch, and non-fast-forward retry during coordination writes.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests reproduce the remote-ahead failure without manual git recovery.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Define doctor and lock tests

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito coordination doctor --json` and `ito coordination lock <change-id> --json` output.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert ahead/behind state, retryable conflicts, lock owner, timestamp, and expiration fields.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement coordination sync engine

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-common/src/**`
- **Dependencies**: None
- **Action**: Implement fetch, merge, commit, push, bounded retry, and semantic conflict detection for internal coordination metadata.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Sync safely resolves remote-ahead metadata without mutating the user's feature branch.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Implement coordination lock model

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-domain/src/**`
- **Dependencies**: None
- **Action**: Add advisory lock records with owner, host/session, timestamp, expiration, and active-lock conflict output.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Active locks are respected and expired locks do not block sync.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add CLI and integrate writes

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**: Add `ito coordination sync --json`, `ito coordination doctor --json`, and `ito coordination lock <change-id> --json`, then integrate sync/retry into create, task, archive, and module writes.
- **Verify**: `make check`
- **Done When**: Coordination writes recover from retryable push conflicts and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
