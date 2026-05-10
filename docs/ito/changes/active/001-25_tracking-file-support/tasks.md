# Tasks for: 001-25_tracking-file-support

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-25_tracking-file-support
ito tasks next 001-25_tracking-file-support
ito tasks start 001-25_tracking-file-support 1.1
ito tasks complete 001-25_tracking-file-support 1.1
ito tasks shelve 001-25_tracking-file-support 1.1
ito tasks unshelve 001-25_tracking-file-support 1.1
ito tasks show 001-25_tracking-file-support
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define tracking file resolution + safety

- **Files**: `ito-rs/crates/ito-schemas/src/**`, `ito-rs/crates/ito-domain/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**: Implement a shared helper to resolve the tracking file for a change (schema `apply.tracks` or fallback `tasks.md`) and validate that the configured value is a safe file name (no separators/traversal).
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Resolution is used by callers via a single helper and unsafe `apply.tracks` values are rejected with an actionable error.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.2: Add tests and fixtures for non-default tracking file names

- **Files**: `ito-rs/crates/ito-core/tests/**`, `ito-rs/crates/ito-domain/tests/**`
- **Dependencies**: None
- **Action**: Add tests that set up a change with `apply.tracks: todo.md` and confirm the resolved tracking path is `changes/<id>/todo.md` (and not `tasks.md`).
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests cover `apply.tracks` override, fallback behavior, and unsafe values.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.3: (Removed) Placeholder task (merge reconciliation)

- **Files**: (none)
- **Dependencies**: None
- **Action**: No-op. This task id exists to preserve audit/task numbering after a merge.
- **Verify**: (none)
- **Done When**: Task is shelved.
- **Updated At**: 2026-02-25
- **Status**: [-] shelved

### Task 1.4: (Removed) Placeholder task (merge reconciliation)

- **Files**: (none)
- **Dependencies**: None
- **Action**: No-op. This task id exists to preserve audit/task numbering after a merge.
- **Verify**: (none)
- **Done When**: Task is shelved.
- **Updated At**: 2026-02-25
- **Status**: [-] shelved

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Validate schema-selected tracking file

- **Files**: `ito-rs/crates/ito-core/src/validate/**`
- **Dependencies**: None
- **Action**: Update `ito validate` to validate the resolved tracking file (and not `tasks.md` when schema tracks a different file).
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Validation output references the resolved tracking file and skips `tasks.md` when overridden.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.2: Empty tasks-tracking file is warning vs strict error

- **Files**: `ito-rs/crates/ito-domain/src/tasks/**`, `ito-rs/crates/ito-core/src/validate/**`
- **Dependencies**: None
- **Action**: If a file is validated as `ito.tasks-tracking.v1` but contains zero recognizable tasks, emit a warning (or an error in strict mode).
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert warning in non-strict and error in strict for empty tracking files.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: `ito tasks` operates on resolved tracking file

- **Files**: `ito-rs/crates/ito-cli/src/app/tasks/**`, `ito-rs/crates/ito-core/src/tasks/**`, `ito-rs/crates/ito-domain/src/tasks/**`
- **Dependencies**: None
- **Action**: Update `ito tasks status|next|start|complete` to read and update the resolved tracking file for the change.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: A change with `apply.tracks: todo.md` is fully operable via `ito tasks`.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

### Task 3.2: Helpful error for non-tasks-tracking tracking format

- **Files**: `ito-rs/crates/ito-cli/src/app/tasks/**`
- **Dependencies**: None
- **Action**: If the resolved tracking file is not an Ito tasks-tracking file (schema chooses a different format), `ito tasks` exits with a helpful error pointing to the schema's tracking format.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert the error message is actionable and mentions the schema-selected tracking format.
- **Updated At**: 2026-02-27
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-02-27
- **Status**: [-] shelved
