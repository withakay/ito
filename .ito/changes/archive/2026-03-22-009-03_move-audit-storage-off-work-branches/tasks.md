# Tasks for: 009-03_move-audit-storage-off-work-branches

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 009-03_move-audit-storage-off-work-branches
ito tasks next 009-03_move-audit-storage-off-work-branches
ito tasks start 009-03_move-audit-storage-off-work-branches 1.1
ito tasks complete 009-03_move-audit-storage-off-work-branches 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add audit storage routing abstraction

- **Files**: `ito-rs/crates/ito-domain/src/audit/`, `ito-rs/crates/ito-core/src/audit/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**: Introduce an audit storage abstraction that can route reads/writes to backend-managed storage, internal-branch storage, or fallback local storage without assuming a tracked working-branch file.
- **Verify**: `cargo test -p ito-core audit`
- **Done When**: audit write/read entrypoints no longer hard-code `.ito/.state/audit/events.jsonl` as the only durable source.
- **Updated At**: 2026-03-16
- **Status**: [x] complete

### Task 1.2: Route backend mode to server-only audit storage

- **Files**: `ito-rs/crates/ito-backend/src/`, `ito-rs/crates/ito-core/src/`, `ito-rs/crates/ito-cli/src/`, `ito-rs/crates/ito-backend/tests/`
- **Dependencies**: Task 1.1
- **Action**: Make backend mode write and validate against backend-managed audit state only, removing the requirement for a tracked local audit JSONL file on working branches.
- **Verify**: `cargo test -p ito-backend events && cargo test -p ito-cli audit`
- **Done When**: backend mode persists audit state only on the server side and audit commands continue to work.
- **Updated At**: 2026-03-16
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Move local durable audit storage to an internal branch/repository

- **Files**: `ito-rs/crates/ito-core/src/audit/`, `ito-rs/crates/ito-cli/src/commands/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**: Implement local-mode durable audit storage on an internal Ito branch/repository, plus non-polluting fallback behavior when that branch cannot be used.
- **Verify**: `cargo test -p ito-core audit_mirror && cargo test -p ito-cli audit_more`
- **Done When**: normal working branches no longer receive tracked audit JSONL churn from routine audit writes.
- **Updated At**: 2026-03-16
- **Status**: [x] complete

### Task 2.2: Migrate validation, reconciliation, and docs to routed audit storage

- **Files**: `ito-rs/crates/ito-cli/src/commands/audit.rs`, `ito-rs/crates/ito-core/src/audit/`, `.ito/`, docs/tests
- **Dependencies**: Task 2.1
- **Action**: Update validation/reconciliation/streaming paths and operational guidance to use routed audit storage, migrate existing tracked audit files, and document the new source-of-truth rules.
- **Verify**: `make check && ito validate 009-03_move-audit-storage-off-work-branches --strict`
- **Done When**: audit tooling works end-to-end without tracked working-branch audit JSONL, and docs describe the new storage model clearly.
- **Updated At**: 2026-03-17
- **Status**: [x] complete
