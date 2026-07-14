# Tasks for: 006-15_rust-ito-path-helpers

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Design the `ito-core` path helper API

- **Files**: ito-rs/crates/ito-core/src
- **Dependencies**: None
- **Action**:
  - Choose between `ItoPaths` struct vs free functions
  - Define the minimum API surface used by CLI/core
- **Verify**: cargo test -p ito-core
- **Done When**: API is implemented and covered by unit tests
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 1.2: Migrate ito-core call sites

- **Files**: ito-rs/crates/ito-core/src/create, ito-rs/crates/ito-core/src/list.rs
- **Dependencies**: Task 1.1
- **Action**:
  - Replace repeated `.join("changes")` / `.join("modules")` patterns
  - Replace string-based path formatting with `PathBuf::join`
- **Verify**: cargo test -p ito-core
- **Done When**: core code uses the helper and behavior is unchanged
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Migrate ito-cli to the shared path helper

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: Task 1.1
- **Action**:
  - Replace `.ito/` path construction with the `ito-core` helper
  - Remove duplicated path logic in validate and tasks
- **Verify**: cargo test -p ito-cli
- **Done When**: CLI uses shared path helpers; tests pass
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete
