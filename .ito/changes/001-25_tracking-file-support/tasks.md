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

### Task 1.1: Resolve and sanitize schema tracking filename

- **Files**: `ito-rs/crates/ito-core/**`, `ito-rs/crates/ito-schemas/**`, `ito-rs/crates/ito-domain/**`
- **Dependencies**: None
- **Action**:
  - Implement tracking file resolution: schema `apply.tracks` if present, else default `tasks.md`.
  - Add defensive validation: reject traversal and path separators in `apply.tracks`.
  - Add unit tests for resolution and rejection cases.
- **Verify**: `make check`
- **Done When**: Tracking file resolution is available as a reusable helper and tests cover safe/unsafe inputs.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.2: Teach TaskRepository to use resolved tracking file

- **Files**: `ito-rs/crates/ito-domain/**`, `ito-rs/crates/ito-core/**`
- **Dependencies**: Task 1.1
- **Action**:
  - Thread the resolved tracking file path through task loading/counting.
  - Keep backward compatibility for schemas that omit `apply.tracks` (still uses `tasks.md`).
  - Add/update tests covering both enhanced and checkbox formats through the resolved path.
- **Verify**: `make check`
- **Done When**: Task counts and task loading operate on the resolved tracking file path.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.3: Update `ito tasks` to read/write the resolved tracking file

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**:
  - Make all `ito tasks` subcommands operate on the resolved tracking file.
  - If schema tracking format is not `ito.tasks-tracking.v1`, exit with a helpful error that explains the mismatch and points to schema-specific workflows/docs.
  - Add CLI tests for both success and mismatch error cases.
- **Verify**: `make check`
- **Done When**: `ito tasks status|next|start|complete` work against a schema-tracked file (e.g. `todo.md`) and refuse non-ito tasks tracking schemas.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 1.4: Validate resolved tracking file and warn on empty tasks

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `ito-rs/crates/ito-schemas/**`
- **Dependencies**: Task 1.1
- **Action**:
  - In schema-driven validation, validate the tracking file resolved from `apply.tracks` (not hard-coded `tasks.md`).
  - Ensure `tasks.md` is not validated when the schema tracks a different file.
  - Add an `ito.tasks-tracking.v1` validation rule: if zero recognizable tasks, warn by default and fail in `--strict`.
  - Add tests covering validate default vs `--strict` output/exit code for the empty-tasks case.
- **Verify**: `make check`
- **Done When**: `ito validate <change-id>` validates the resolved tracking file and emits the empty-tasks warning/error per strictness.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update audit reconcile to parse the resolved tracking file

- **Files**: `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`
- **Dependencies**: None
- **Action**:
  - Update audit reconcile logic that reads task tracking to use the resolved tracking file.
  - Add tests covering reconcile behavior for both `tasks.md` default and schema-tracked filenames.
- **Verify**: `make check`
- **Done When**: `ito audit reconcile` reflects task state based on the resolved tracking file.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

### Task 2.2: Documentation and examples for apply.tracks

- **Files**: `docs/**`, `schemas/**`, `.ito/specs/**` (if needed)
- **Dependencies**: None
- **Action**:
  - Add/adjust docs that explain `apply.tracks` behavior, safety constraints, and how it interacts with `ito validate` and `ito tasks`.
  - Add a minimal example schema using `apply.tracks: todo.md`.
- **Verify**: `make check`
- **Done When**: Docs describe the new behavior and include an example that matches acceptance criteria.
- **Updated At**: 2026-02-25
- **Status**: [ ] pending

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-02-25
- **Status**: [ ] pending
