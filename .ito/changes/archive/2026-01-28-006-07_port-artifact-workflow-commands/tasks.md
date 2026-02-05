# Tasks for: 006-07_port-artifact-workflow-commands

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: `create module` and `create change`

### Task 1.1: Port module creation

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-core/src/module/*`
- **Dependencies**: Change `006-04_port-id-and-config-foundations`
- **Action**:
  - Implement module creation rules, IDs, and filesystem layout
  - Match TS error messages on conflicts
- **Verify**: integration + parity tests
- **Done When**: file writes match TS
- **Status**: \[x\] completed

### Task 1.2: Port change creation

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-core/src/change/*`
- **Dependencies**: Task 1.1
- **Action**:
  - Implement change creation with module-first layout
  - Create `.ito.yaml` and directory structure
- **Verify**: integration + parity tests
- **Done When**: scaffolding matches TS
- **Status**: \[x\] completed

______________________________________________________________________

## Wave 2: `status`, `instructions`, `templates`

### Task 2.1: Port `ito status`

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-core/src/status/*`
- **Dependencies**: Task 1.2
- **Action**:
  - Render artifact completion state
  - Match TS output ordering and wording
- **Verify**: parity tests
- **Done When**: status parity passes
- **Status**: \[x\] completed

### Task 2.2: Port `ito agent instruction` / `ito instructions`

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-templates/src/*`
- **Dependencies**: Task 2.1
- **Action**:
  - Emit canonical instructions for proposal/specs/design/tasks
  - Match TS output content exactly
- **Verify**: snapshot parity tests
- **Done When**: outputs match TS
- **Status**: \[x\] completed

### Task 2.3: Port `ito templates`

- **Files**: `ito-rs/crates/ito-cli/src/*`, `ito-rs/crates/ito-templates/src/*`
- **Dependencies**: Task 2.2
- **Action**:
  - Implement listing/showing templates as per TS
- **Verify**: parity tests
- **Done When**: templates command matches TS
- **Status**: \[x\] completed

______________________________________________________________________

## Wave 3: Coverage + Validation

### Task 3.1: Coverage target

- **Files**: `ito-rs/README.md`
- **Dependencies**: None
- **Action**:
  - Target >= 80% coverage for `ito-core` create/status logic
- **Verify**: `cargo llvm-cov --workspace`
- **Done When**: coverage target met or tracked
- **Status**: \[x\] completed

### Task 3.2: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run `ito validate 006-07_port-artifact-workflow-commands --strict` and fix any issues
- **Verify**: Validation passes
- **Done When**: `ito validate --strict` is clean
- **Status**: \[x\] completed

## Verify

```bash
ito validate 006-07_port-artifact-workflow-commands --strict
cd ito-rs
cargo test --workspace
cargo llvm-cov --workspace
```
