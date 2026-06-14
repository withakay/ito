# Tasks for: 030-05_managed-file-ownership

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-05_managed-file-ownership
ito tasks next 030-05_managed-file-ownership
ito tasks start 030-05_managed-file-ownership 1.1
ito tasks complete 030-05_managed-file-ownership 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define managed status tests

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito managed status --json` classifying generated, marker-managed, user-owned, and unknown files.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert ownership type, owning template, and durable guidance location fields.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Define dry-run and diff tests

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito managed diff --json` and `ito update --dry-run --json` that prove no files are modified.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests cover creates, updates, skips, conflicts, and overwrites.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement managed-file registry

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-templates/src/**`
- **Dependencies**: None
- **Action**: Add a registry populated from installer/template metadata with ownership classifications and durable guidance recommendations.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Registry output matches installer-planned files and marker-managed blocks.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Implement update planner

- **Files**: `ito-rs/crates/ito-core/src/installers/**`, `ito-rs/crates/ito-templates/src/**`
- **Dependencies**: None
- **Action**: Add a no-write update planner that computes creates, updates, skips, conflicts, overwrites, and text diffs where useful.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Planner can back both managed diff and update dry-run.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add managed CLI commands and guidance

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-templates/assets/**`
- **Dependencies**: None
- **Action**: Add `ito managed status --json`, `ito managed diff --json`, `ito update --dry-run --json`, and guidance to inspect managed status before editing managed paths.
- **Verify**: `make check`
- **Done When**: Commands work, guidance is updated, and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
