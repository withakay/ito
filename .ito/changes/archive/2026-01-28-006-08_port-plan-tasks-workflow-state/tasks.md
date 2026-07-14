# Tasks for: 006-08_port-plan-tasks-workflow-state

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: Schemas

### Task 1.1: Implement workflow/state schemas

- **Files**: `ito-rs/crates/ito-schemas/src/*`
- **Dependencies**: None
- **Action**:
  - Model YAML/JSON formats used by TS
  - Add serialization roundtrip tests
- **Verify**: unit tests
- **Done When**: schemas roundtrip and match fixtures
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Commands

### Task 2.1: Port `plan` and `tasks`

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-workflow/src/*`
- **Dependencies**: Task 1.1
- **Action**:
  - Implement commands and match TS output
- **Verify**: integration + parity tests
- **Done When**: parity passes
- **Status**: \[x\] complete

### Task 2.2: Port `workflow` and `state`

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-workflow/src/*`
- **Dependencies**: Task 2.1
- **Action**:
  - Implement commands and ensure state reads/writes are compatible
- **Verify**: integration + parity tests
- **Done When**: parity passes including on-disk state
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3: Coverage + Validation

### Task 3.1: Coverage target

- **Files**: `ito-rs/README.md`
- **Dependencies**: None
- **Action**:
  - Target >= 80% coverage in `ito-workflow` and `ito-schemas`
- **Verify**: `cargo llvm-cov --workspace`
- **Done When**: coverage target met or tracked
- **Status**: \[x\] complete

### Task 3.2: Validate change artifacts

- **Files**: N/A
- **Dependencies**: Task 1.1, Task 2.1, Task 2.2, Task 3.1
- **Action**:
  - Run `ito validate 006-08_port-plan-tasks-workflow-state --strict` and fix any issues
- **Verify**: Validation passes
- **Done When**: `ito validate --strict` is clean
- **Status**: \[x\] complete

## Verify

```bash
ito validate 006-08_port-plan-tasks-workflow-state --strict
cd ito-rs
cargo test --workspace
cargo llvm-cov --workspace
```
