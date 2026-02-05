# Tasks for: 006-03_parity-test-harness

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: Test Support

### Task 1.1: Add `ito-test-support` helpers

- **Files**: `ito-rs/crates/ito-test-support/src/*`
- **Dependencies**: Change `006-02_create-ito-rs-workspace`
- **Action**:
  - Implement helpers to run TS and Rust CLIs with captured stdout/stderr/exit code
  - Implement temp dir + fixture copy helpers
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: unit tests cover harness helpers
- **Status**: \[x\] complete

### Task 1.2: Add PTY test helpers

- **Files**: `ito-rs/crates/ito-test-support/src/pty/*`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a minimal PTY runner abstraction for interactive commands
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: PTY helper is exercised by a small test
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Parity Tests

### Task 2.1: Add initial parity tests

- **Files**: `ito-rs/crates/ito-cli/tests/parity_help_version.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Compare `--help` and `--version` outputs
  - Add one non-mutating command parity test (e.g., `list --json` once implemented)
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: snapshots are deterministic
- **Status**: \[x\] complete

### Task 2.2: Coverage target

- **Files**: `ito-rs/README.md`
- **Dependencies**: Task 2.1
- **Action**:
  - Target >= 80% coverage for harness helpers
- **Verify**: `cd ito-rs && cargo llvm-cov --workspace`
- **Done When**: coverage measured and tracked
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3: Validate Artifacts

### Task 3.1: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run strict validation and fix any issues
- **Verify**: `ito validate 006-03_parity-test-harness --strict`
- **Done When**: validation passes
- **Status**: \[x\] complete

## Verify

```bash
ito validate 006-03_parity-test-harness --strict
cd ito-rs
cargo test --workspace
cargo llvm-cov --workspace
```
