# Tasks for: 006-16_rust-test-suite-decouple-ts-oracle

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add feature flag and gate parity tests

- **Files**: ito-rs/crates/ito-cli/Cargo.toml, ito-rs/crates/ito-cli/tests
- **Dependencies**: None
- **Action**:
  - Delete TS-oracle parity tests from `ito-cli/tests`
  - Remove TS-oracle feature flags (if any exist)
  - Remove TS-oracle helpers from `ito-test-support` if unused
- **Verify**: cargo test -p ito-cli
- **Done When**: ito-cli tests pass without node/bun
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

### Task 1.2: Move duplicated tree comparison helpers into ito-test-support

- **Files**: ito-rs/crates/ito-test-support/src
- **Dependencies**: Task 1.1
- **Action**:
  - Identify repeated code for collecting and comparing directory trees
  - Extract into reusable helpers
  - Update tests to use the shared helper
- **Verify**: cargo test -p ito-cli
- **Done When**: test code duplication is reduced and behavior is unchanged
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Remove outdated parity-test documentation (if present)

- **Files**: ito-rs/README.md
- **Dependencies**: Task 1.1
- **Action**:
  - Remove or update any docs that reference TS-oracle parity testing
- **Verify**: cargo test --workspace
- **Done When**: docs no longer mention parity tests and default tests remain node/bun-free
- **Updated At**: 2026-01-29
- **Status**: \[x\] complete
