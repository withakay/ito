# Tasks for: 001-08_allow-change-number-overflow

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Update specs to remove the 99 cap

- **Files**: .ito/changes/001-08_allow-change-number-overflow/specs/ito-rs-change-id-overflow/spec.md
- **Dependencies**: None
- **Action**:
  - Ensure the canonical change ID format is described as `NNN-<change>_name`
  - Add scenarios/examples for 3+ digit change numbers (`1-100_bar` -> `001-100_bar`)
- **Verify**: ito validate "001-08_allow-change-number-overflow" --strict
- **Done When**: Specs validate and reflect unbounded change numbers
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 1.2: Audit ito-rs callers for 2-digit assumptions

- **Files**: ito-rs/crates/ito-core/src/id/change_id.rs, ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 1.1
- **Action**:
  - Search `ito-rs` for regexes or formatting that assume `NNN-NN_` where `NN` is exactly 2 digits
  - Update any such code to allow 2+ digits for the change number
- **Verify**: cargo test
- **Done When**: No ito-rs code assumes change numbers are capped at 2 digits
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Remove the hard cap in the Rust parser

- **Files**: ito-rs/crates/ito-core/src/id/change_id.rs
- **Dependencies**: Task 1.2
- **Action**:
  - Remove/relax the `change_num > 99` validation
  - Add unit tests for 3+ digit change numbers and excessive padding (`1-000100_bar`)
  - Ensure canonical formatting uses minimum 2 digits but allows overflow (e.g. `100` stays `100`)
- **Verify**: cargo test
- **Done When**: Rust parser accepts 3+ digit change numbers and all tests pass
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 2.2: Verify end-to-end behavior in the Rust CLI

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 2.1
- **Action**:
  - Ensure Rust CLI commands that accept a change ID (validate/show/etc.) do not reject `NNN-100_name`
  - Add a minimal fixture/test that includes a `001-100_example` change directory (Rust-only)
- **Verify**: cargo test
- **Done When**: Rust CLI commands work with 3+ digit change numbers
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Confirm sorting expectations and doc wording

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: .ito/changes/001-08_allow-change-number-overflow/proposal.md, .ito/changes/001-08_allow-change-number-overflow/design.md
- **Dependencies**: Task 2.2
- **Action**:
  - Confirm the intended behavior is "minimum padding, allow overflow" (no renames)
  - Confirm wording explicitly states ordering is best-effort past 99
- **Done When**: Human reviewer approves wording and semantics
- **Updated At**: 2026-01-29
- **Status**: [x] completed
