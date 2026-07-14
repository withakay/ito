# Tasks for: 006-09_port-ralph-loop

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: Runner + State

### Task 1.1: Port ralph runner core

- **Files**: `ito-rs/crates/ito-harness/src/*`, `ito-rs/crates/ito-core/src/ralph/*`
- **Dependencies**: Change `006-08_port-plan-tasks-workflow-state`
- **Action**:
  - Implement iteration loop, min/max iterations, status output
  - Implement completion promise detection
- **Verify**: unit tests
- **Done When**: runner matches TS semantics
- **Status**: \[x\] completed

### Task 1.2: Implement on-disk ralph state layout

- **Files**: `ito-rs/crates/ito-core/src/ralph/state/*`
- **Dependencies**: Task 1.1
- **Action**:
  - Write state under `.ito/.state/ralph/<change-id>/` matching TS layout
- **Verify**: integration tests
- **Done When**: file layout matches TS
- **Status**: \[x\] completed

______________________________________________________________________

## Wave 2: CLI + Harnesses

### Task 2.1: Implement CLI commands `ralph` and `loop`

- **Files**: `ito-rs/crates/ito-cli/src/*`
- **Dependencies**: Task 1.2
- **Action**:
  - Implement argument parsing and pass-through to harness
  - Match TS flags and error messages
- **Verify**: integration tests
- **Done When**: CLI matches TS in non-networked mode
- **Status**: \[x\] completed

### Task 2.2: Add stub harness for tests

- **Files**: `ito-rs/crates/ito-harness/src/stub/*`
- **Dependencies**: Task 2.1
- **Action**:
  - Implement a harness that returns scripted outputs and exit codes
- **Verify**: parity tests can run without network
- **Done When**: tests do not require external services
- **Status**: \[x\] completed

______________________________________________________________________

## Wave 3: Parity + Coverage + Validation

### Task 3.1: Add parity tests for loop semantics

- **Files**: `ito-rs/crates/ito-cli/tests/parity_*`
- **Dependencies**: Change `006-03_parity-test-harness`
- **Action**:
  - Compare TS vs Rust loop behavior in controlled fixtures
  - Validate state files and history outputs
- **Verify**: `cargo test --workspace`
- **Done When**: parity tests are deterministic
- **Status**: \[x\] completed

### Task 3.2: Coverage target

- **Files**: `ito-rs/README.md`
- **Dependencies**: None
- **Action**:
  - Target >= 80% coverage for runner and state logic
- **Verify**: `cargo llvm-cov --workspace`
- **Done When**: coverage target met or tracked
- **Status**: \[x\] completed

### Task 3.3: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run `ito validate 006-09_port-ralph-loop --strict` and fix any issues
- **Verify**: Validation passes
- **Done When**: `ito validate --strict` is clean
- **Status**: \[x\] completed

## Verify

```bash
ito validate 006-09_port-ralph-loop --strict
cd ito-rs
cargo test --workspace
cargo llvm-cov --workspace
```
