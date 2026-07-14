# Tasks for: 024-18_import-local-state-to-backend

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-18_import-local-state-to-backend
ito tasks next 024-18_import-local-state-to-backend
ito tasks start 024-18_import-local-state-to-backend 1.1
ito tasks complete 024-18_import-local-state-to-backend 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add core local-to-backend import orchestration

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-domain/src/`
- **Dependencies**: None
- **Action**: Implement core import orchestration that enumerates local active and archived changes, converts them into backend artifact bundles, and preserves lifecycle status.
- **Verify**: `cargo test -p ito-core import`
- **Done When**: Core exposes a reusable import API with imported/skipped/failed summary output for both lifecycle states.
- **Updated At**: 2026-03-10
- **Status**: [x] complete

### Task 1.2: Make import rerunnable and dry-run aware

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-core/tests/`, `ito-rs/crates/ito-backend/src/`
- **Dependencies**: Task 1.1
- **Action**: Add idempotent backend upsert behavior, dry-run preview reporting, and partial-failure resume semantics for import.
- **Verify**: `cargo test -p ito-core import && cargo test -p ito-backend import`
- **Done When**: Repeated imports are safe, dry-run performs no writes, and partial imports can be rerun without duplication.
- **Updated At**: 2026-03-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Wire `ito backend import` CLI command

- **Files**: `ito-rs/crates/ito-cli/src/cli/backend.rs`, `ito-rs/crates/ito-cli/src/commands/backend.rs`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Add the `ito backend import` CLI surface with backend-mode gating, optional `--dry-run`, and user-facing summary output backed by the shared core import orchestration.
- **Verify**: `cargo test -p ito-cli backend`
- **Done When**: Users can run direct imports from the CLI and see consistent preview and completion summaries.
- **Updated At**: 2026-03-10
- **Status**: [x] complete

### Task 2.2: Add backend read-parity integration coverage

- **Files**: `ito-rs/crates/ito-core/tests/`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 2.1
- **Action**: Add end-to-end tests covering active and archived import, rerun idempotency, dry-run no-op behavior, and backend-backed change reads after migration.
- **Verify**: `make check`
- **Done When**: The new import workflow is covered by integration tests that fail if lifecycle parity regresses.
- **Updated At**: 2026-03-10
- **Status**: [x] complete
