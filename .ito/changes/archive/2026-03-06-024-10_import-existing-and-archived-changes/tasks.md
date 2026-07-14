# Tasks for: 024-10_import-existing-and-archived-changes

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-10_import-existing-and-archived-changes
ito tasks next 024-10_import-existing-and-archived-changes
ito tasks start 024-10_import-existing-and-archived-changes 1.1
ito tasks complete 024-10_import-existing-and-archived-changes 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement core local-to-backend import orchestration

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-domain/src/`
- **Dependencies**: None
- **Action**: Add import orchestration that enumerates local active and archived changes, builds backend upload payloads, and maps lifecycle state correctly.
- **Verify**: `cargo test -p ito-core import`
- **Done When**: Core exposes a tested import API returning imported/skipped/failed summaries.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

### Task 1.2: Add idempotent and resumable backend import behavior

- **Files**: `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-backend/src/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 1.1
- **Action**: Implement retry-safe upsert logic plus import parity validation and post-import local cleanup with backup safeguards.
- **Verify**: `cargo test -p ito-core import && cargo test -p ito-backend import`
- **Done When**: Repeated imports are safe, parity checks gate success, and local artifacts are only removed after verified import completion.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Wire `ito init` backend/local selection and import prompt

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`, `ito-rs/crates/ito-cli/src/runtime.rs`, `ito-rs/crates/ito-cli/src/app/`
- **Dependencies**: None
- **Action**: Add init flow that lets users choose backend or local, uses explicit prompt copy for storage/import decisions, supports non-interactive flags (`--backend`, `--local`, `--import-local-changes`, `--no-import-local-changes`), and blocks backend mode when import is declined.
- **Verify**: `cargo test -p ito-cli init`
- **Done When**: `ito init` enforces migration gating for backend mode, prompt text is deterministic, and conflicting flag combinations fail with actionable errors.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

### Task 2.2: Wire `ito backend import` command for direct and init-driven migration

- **Files**: `ito-rs/crates/ito-cli/src/app/`, `ito-rs/crates/ito-cli/src/runtime.rs`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 2.1
- **Action**: Add `ito backend import` with `--dry-run` and summary reporting, and reuse the same import orchestration from `ito init` when backend is selected.
- **Verify**: `cargo test -p ito-cli backend`
- **Done When**: Direct backend import and init-triggered import share behavior and output consistent migration summaries.
- **Updated At**: 2026-03-06
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add integration coverage and migration docs

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`, `docs/`
- **Dependencies**: None
- **Action**: Add end-to-end tests for init Yes/No prompt paths, active+archived import, idempotent rerun, partial-failure resume, backend-mode read parity, and post-import local cleanup; document cutover runbook.
- **Verify**: `make check`
- **Done When**: Integration tests pass and docs describe init-gated backend cutover, required import, parity checks, and local cleanup behavior.
- **Updated At**: 2026-03-06
- **Status**: [x] complete
