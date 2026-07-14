# Tasks for: 006-04_port-id-and-config-foundations

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: ID + Path Foundations

### Task 1.1: Implement ID parsing and normalization

- **Files**: `ito-rs/crates/ito-core/src/id/*`
- **Dependencies**: Change `006-02_create-ito-rs-workspace`
- **Action**:
  - Define types for ModuleId, ChangeId, SpecId
  - Implement parse/format rules matching TS
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: unit tests cover accepted/rejected formats
- **Status**: \[x\] complete

### Task 1.2: Implement ito dir discovery

- **Files**: `ito-rs/crates/ito-core/src/ito_dir/*`
- **Dependencies**: Task 1.1
- **Action**:
  - Implement discovery of `.ito` and any TS-compatible overrides
  - Ensure behavior matches in nested directories
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: unit tests cover discovery cases
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Config + Output Controls

### Task 2.1: Implement config/env handling

- **Files**: `ito-rs/crates/ito-core/src/config/*`
- **Dependencies**: Task 1.2
- **Action**:
  - Parse global config formats used by TS
  - Implement env var behavior required for parity
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: unit tests cover config precedence rules
- **Status**: \[x\] complete

### Task 2.2: Implement `--no-color` and `NO_COLOR`

- **Files**: `ito-rs/crates/ito-cli/src/output/*`, `ito-rs/crates/ito-core/src/output/*`
- **Dependencies**: Task 2.1
- **Action**:
  - Match TS color enablement rules
- **Verify**: parity tests in harness
- **Done When**: outputs match TS under NO_COLOR
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3: Parity + Coverage

### Task 3.1: Add parity tests for foundations

- **Files**: `ito-rs/crates/ito-cli/tests/parity_foundations.rs`
- **Dependencies**: Change `006-03_parity-test-harness`
- **Action**:
  - Compare help output regarding global flags
  - Compare behavior of `--no-color` and any env-driven modes
- **Verify**: `cd ito-rs && cargo test --workspace`
- **Done When**: parity tests deterministic
- **Status**: \[x\] complete

### Task 3.2: Coverage target

- **Files**: `ito-rs/README.md`
- **Dependencies**: Task 3.1
- **Action**:
  - Target >= 85% coverage for `ito-core` foundation modules
- **Verify**: `cd ito-rs && cargo llvm-cov --workspace`
- **Done When**: coverage target met or tracked
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 4: Validate Artifacts

### Task 4.1: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run strict validation and fix any issues
- **Verify**: `ito validate 006-04_port-id-and-config-foundations --strict`
- **Done When**: validation passes
- **Status**: \[x\] complete

## Verify

```bash
ito validate 006-04_port-id-and-config-foundations --strict
cd ito-rs
cargo test --workspace
cargo llvm-cov --workspace
```
