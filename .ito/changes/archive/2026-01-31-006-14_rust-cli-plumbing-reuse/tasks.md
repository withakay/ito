# Tasks for: 006-14_rust-cli-plumbing-reuse

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Identify duplication hotspots and define shared helpers

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: None
- **Action**:
  - Enumerate duplicated patterns (fail/exit, diagnostics printing, tasks validation gating)
  - Decide the minimal helper surface (error type, diagnostic printing helpers)
- **Verify**: cargo test -p ito-cli
- **Done When**: Helper API is agreed and documented in-code
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 1.2: Implement shared error + diagnostics helpers

- **Files**: ito-rs/crates/ito-cli/src
- **Dependencies**: Task 1.1
- **Action**:
  - Add `CliError` and `CliResult`
  - Add diagnostic printing helper that supports `path:line` formatting
- **Verify**: cargo test -p ito-cli
- **Done When**: Helpers are used in at least one subcommand
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Refactor tasks subcommands to use shared plumbing

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 1.2
- **Action**:
  - Replace repeated tasks validation printing with shared helper
  - Replace ad-hoc exits with `CliResult` and a single exit path
- **Verify**: cargo test -p ito-cli
- **Done When**: tasks init/status/next/start/complete/shelve/unshelve behave the same with less repetition
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 2.2: Refactor validate subcommand to use shared plumbing

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 1.2
- **Action**:
  - Centralize validation issue printing
  - Ensure exit codes are consistent for bulk and single-item validation
- **Verify**: cargo test -p ito-cli
- **Done When**: validate command uses shared helpers and reduces duplication
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Run a clippy-driven cleanup pass on touched code

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 2.1
- **Action**:
  - Address obvious clippy warnings introduced/nearby while keeping changes scoped
- **Verify**: cargo clippy -p ito-cli --all-targets --all-features
- **Done When**: clippy warnings reduced for modified areas
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete
