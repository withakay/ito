# Tasks for: 016-12_sort-id-lists-ascending

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 016-12_sort-id-lists-ascending
ito tasks next 016-12_sort-id-lists-ascending
ito tasks start 016-12_sort-id-lists-ascending 1.1
ito tasks complete 016-12_sort-id-lists-ascending 1.1
ito tasks shelve 016-12_sort-id-lists-ascending 1.1
ito tasks unshelve 016-12_sort-id-lists-ascending 1.1
ito tasks show 016-12_sort-id-lists-ascending
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add failing tests for ID-sorted list surfaces

- **Files**: `ito-rs/crates/ito-core/src/list.rs`, `ito-rs/crates/ito-core/src/tasks.rs`, `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**:
  Add or update tests that assert ascending ID ordering for module/change/spec/task list outputs and deterministic tie-break behavior where alternate sorting is supported.
- **Verify**: `cargo test -p ito-core list::tests -- --nocapture && cargo test -p ito-core tasks::tests -- --nocapture && cargo test -p ito-cli -- --nocapture`
- **Done When**: Tests fail before implementation and capture required ordering behavior.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

### Task 1.2: Add failing tests for allocation-state serialization stability

- **Files**: `ito-rs/crates/ito-core/src/create/mod.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add tests to assert `.ito/workflows/.state/change-allocations.json` module entries are emitted in ascending module ID order and remain deterministic across repeated writes.
- **Verify**: `cargo test -p ito-core create::tests -- --nocapture`
- **Done When**: Tests fail under current non-deterministic ordering and define expected canonical JSON behavior.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement canonical ordering in list and task outputs

- **Files**: `ito-rs/crates/ito-core/src/list.rs`, `ito-rs/crates/ito-core/src/tasks.rs`, `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/src/app/show.rs`
- **Dependencies**: None
- **Action**:
  Implement ordering helpers and apply them so ID-bearing list outputs are ascending by canonical ID in both human-readable and JSON output paths.
- **Verify**: `cargo test -p ito-core list::tests tasks::tests -- --nocapture && cargo test -p ito-cli -- --nocapture`
- **Done When**: All ordering assertions pass and outputs are deterministic for identical inputs.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

### Task 2.2: Implement canonical allocation-state and module checklist ordering

- **Files**: `ito-rs/crates/ito-core/src/create/mod.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Replace non-deterministic allocation-state map ordering with canonical sorted ordering and ensure module change checklist entries are written in ascending change ID order.
- **Verify**: `cargo test -p ito-core create::tests -- --nocapture`
- **Done When**: Allocation-state and module checklist order is stable and ascending by ID under repeated updates.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Run full quality gates and finalize behavior checks

- **Files**: `ito-rs/crates/ito-core/src/list.rs`, `ito-rs/crates/ito-core/src/tasks.rs`, `ito-rs/crates/ito-core/src/create/mod.rs`, `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/src/app/show.rs`
- **Dependencies**: None
- **Action**:
  Run formatting, linting, and tests to confirm no regressions and that ordering policy is enforced end-to-end.
- **Verify**: `make check && make test`
- **Done When**: All checks pass and command behavior matches spec deltas.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
