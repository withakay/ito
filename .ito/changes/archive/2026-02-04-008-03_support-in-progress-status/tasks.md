# Tasks for: 008-03_support-in-progress-status

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 008-03_support-in-progress-status
ito tasks next 008-03_support-in-progress-status
ito tasks start 008-03_support-in-progress-status 1.1
ito tasks complete 008-03_support-in-progress-status 1.1
```

______________________________________________________________________

## Wave 1

Parser Changes

- **Depends On**: None

### Task 1.1: Add in-progress checkbox constant and parsing

- **Files**: `ito-rs/crates/ito-domain/src/tasks/parse.rs`
- **Dependencies**: None
- **Action**:
  - Ensure checkbox parsing recognizes `- [~]` as `TaskStatus::InProgress`
- **Verify**: `cargo test -p ito-domain`
- **Done When**: Parser correctly identifies `- [~]` as in-progress status
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 1.2: Add checkbox format serialization for in-progress

- **Files**: `ito-rs/crates/ito-domain/src/tasks/update.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Ensure `update_checkbox_task_status(...)` writes `- [~]` when setting `InProgress`
- **Verify**: `cargo test -p ito-domain`
- **Done When**: Writing a task with in-progress status produces `- [~]` marker
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

Unit Tests for Parser

- **Depends On**: Wave 1

### Task 2.1: Write failing test for in-progress checkbox parsing

- **Files**: `ito-rs/crates/ito-domain/tests/tasks_parsing.rs`
- **Dependencies**: None
- **Action**:
  - Write test case: `test_parse_checkbox_in_progress`
  - Input: `- [~] Task in progress`
  - Expected: Task with status `InProgress` and description "Task in progress"
- **Verify**: `cargo test -p ito-domain test_parse_checkbox`
- **Done When**: Test passes after Wave 1 implementation
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 2.2: Write test for mixed checkbox status parsing

- **Files**: `ito-rs/crates/ito-domain/tests/tasks_parsing.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Write test case: `test_parse_checkbox_mixed_statuses`
  - Input: Multi-line with `- [ ]`, `- [~]`, `- [x]` tasks
  - Expected: Correct status for each task
- **Verify**: `cargo test -p ito-domain test_parse_checkbox`
- **Done When**: Test passes with all three statuses correctly identified
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 2.3: Write test for checkbox serialization round-trip

- **Files**: `ito-rs/crates/ito-domain/tests/tasks_parsing.rs`
- **Dependencies**: Task 2.1
- **Action**:
  - Write test case: `test_checkbox_roundtrip_in_progress`
  - Parse tasks, modify one to in-progress, serialize, re-parse
  - Verify status is preserved through round-trip
- **Verify**: `cargo test -p ito-domain test_checkbox`
- **Done When**: Round-trip preserves in-progress status
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

CLI Command Updates

- **Depends On**: Wave 2

### Task 3.1: Remove error for checkbox format in start command

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: None
- **Action**:
  - Locate the error message at line ~227: "Checkbox-only tasks.md does not support in-progress"
  - Remove or replace the error branch
  - Allow the start command to proceed for checkbox format
- **Verify**: `cargo build -p ito-cli`
- **Done When**: `ito tasks start` no longer errors on checkbox format
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 3.2: Implement single in-progress validation for checkbox

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: Task 3.1
- **Action**:
  - Before starting a checkbox task, check if any task already has `- [~]` marker
  - If found, return error: "Task N is already in-progress. Complete it first or use a different task."
  - Include the in-progress task's description in the error message
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Starting a task when another is in-progress returns clear error
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 3.3: Update start command to write in-progress marker

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: Task 3.2
- **Action**:
  - When starting a checkbox task, update `- [ ]` to `- [~]`
  - Use the existing task update/write logic
  - Print confirmation: "Started task N: <description>"
- **Verify**: `ito tasks start <change-id> <task-id>` on a test checkbox file
- **Done When**: Task marker changes from `- [ ]` to `- [~]` in file
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 3.4: Update next command for checkbox in-progress awareness

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: Task 3.3
- **Action**:
  - When finding next task in checkbox format, check for existing in-progress
  - If in-progress task exists, display: "Current task: <description>"
  - If no in-progress, find first pending and offer to start it
- **Verify**: `ito tasks next <change-id>` on checkbox file with in-progress task
- **Done When**: Next command shows current in-progress task or offers to start next pending
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

Integration Tests

- **Depends On**: Wave 3

### Task 4.1: Write integration test for checkbox start workflow

- **Files**: `ito-rs/crates/ito-cli/tests/tasks_more.rs`
- **Dependencies**: None
- **Action**:
  - Create a test checkbox tasks.md file
  - Run `ito tasks start` via CLI
  - Verify file is updated with `- [~]` marker
  - Verify output message
- **Verify**: `cargo test -p ito-cli tasks_more`
- **Done When**: Integration test passes for start workflow
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 4.2: Write integration test for single in-progress constraint

- **Files**: `ito-rs/crates/ito-cli/tests/tasks_more.rs`
- **Dependencies**: Task 4.1
- **Action**:
  - Create checkbox file with one `- [~]` task
  - Attempt to start another task
  - Verify error is returned
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Test confirms constraint is enforced
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

Final Verification

- **Depends On**: Wave 4

### Task 5.1: Run full test suite

- **Files**: All
- **Dependencies**: None
- **Action**:
  - Run `cargo test --workspace`
  - Run `cargo clippy --workspace`
  - Ensure no regressions
- **Verify**: `cargo test --workspace && cargo clippy --workspace`
- **Done When**: All tests pass, no clippy warnings
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 5.2: Manual end-to-end verification

- **Files**: None (manual test)
- **Dependencies**: Task 5.1
- **Action**:
  - Create a test change with checkbox tasks.md
  - Run `ito tasks start` - verify marker changes to `- [~]`
  - Run `ito tasks next` - verify shows current in-progress
  - Run `ito tasks complete` - verify marker changes to `- [x]`
  - Try starting another task while one is in-progress - verify error
- **Verify**: Manual verification
- **Done When**: All manual test scenarios pass
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
