# Tasks for: 001-19_archive-completed-flag

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-19_archive-completed-flag
ito tasks next 001-19_archive-completed-flag
ito tasks start 001-19_archive-completed-flag 1.1
ito tasks complete 001-19_archive-completed-flag 1.1
ito tasks show 001-19_archive-completed-flag
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `--completed` flag to `ArchiveArgs` in CLI definition

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Add a `completed` boolean field to `ArchiveArgs` with `#[arg(long = "completed")]`.
  Add a clap `conflicts_with` attribute so `--completed` and the positional `CHANGE` argument are mutually exclusive.
- **Verify**: `cargo check --workspace`
- **Done When**: `ArchiveArgs` has the `completed` field, and `ito archive --completed some-change` is rejected by clap with a conflict error.
- **Updated At**: 2026-02-09
- **Status**: [x] complete

### Task 1.2: Write failing tests for batch archive behavior (RED)

- **Files**: `ito-rs/crates/ito-cli/tests/` (new or existing test file)
- **Dependencies**: Task 1.1
- **Action**:
  Write integration tests covering:
  1. `ito archive --completed` with no completed changes prints "No completed changes" and exits 0
  2. `ito archive --completed -y` archives all completed changes
  3. `ito archive --completed` with `--skip-specs` skips spec updates for all
  4. `ito archive some-change --completed` is rejected (mutual exclusivity)
  5. Partial failure: one change archive fails but others succeed, exit non-zero
  Tests should fail initially (RED phase).
- **Verify**: `cargo test --workspace -- archive_completed` (expect failures)
- **Done When**: Tests exist and fail because the batch logic is not yet implemented.
- **Updated At**: 2026-02-09
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement batch archive logic in `handle_archive` (GREEN)

- **Files**: `ito-rs/crates/ito-cli/src/app/archive.rs`
- **Dependencies**: None
- **Action**:
  Update `handle_archive_clap` to detect when `args.completed` is true and dispatch to a new batch archive path. The batch path should:
  1. Use `FsChangeRepository::list_complete()` to find completed changes
  2. If empty, print "No completed changes to archive." and return Ok
  3. Iterate over each completed change, calling the existing single-change archive logic
  4. Track successes and failures per change
  5. Print per-change progress (archived name or error)
  6. Print summary: "Archived N change(s)." or "Archived N change(s), M failed."
  7. Return error if any change failed
  Forward `--yes`, `--skip-specs`, and `--no-validate` flags to each per-change invocation.
- **Verify**: `cargo test --workspace -- archive_completed` (all tests pass)
- **Done When**: All tests from Task 1.2 pass (GREEN phase).
- **Updated At**: 2026-02-09
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Refactor and clean up (REFACTOR)

- **Files**: `ito-rs/crates/ito-cli/src/app/archive.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Review the implementation for:
  - Extract the single-change archive logic into a reusable `archive_single_change()` function if not already done
  - Ensure consistent error messages and output formatting
  - Add doc comments on the batch path
  - Run `make check` (clippy + fmt)
- **Verify**: `make check && cargo test --workspace`
- **Done When**: Code is clean, all tests pass, clippy and fmt are clean.
- **Updated At**: 2026-02-09
- **Status**: [x] complete

### Task 3.2: Verify test coverage meets target

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-cli/src/app/archive.rs`
- **Dependencies**: Task 3.1
- **Action**:
  Run `make test-coverage` and verify the archive module meets the project coverage target (100% target, 80% minimum).
  Add any missing edge case tests if coverage is below target.
- **Verify**: `make test-coverage`
- **Done When**: Coverage for archive-related code meets the target.
- **Updated At**: 2026-02-09
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
