# Tasks for: 003-02_crate-code-quality-audit

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Parallel per wave (crates are independent)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use tasks CLI to drive status updates
- **File Size Limit**: Split any file exceeding 1000 lines into logical modules

```bash
ito tasks status 003-02_crate-code-quality-audit
ito tasks next 003-02_crate-code-quality-audit
ito tasks start 003-02_crate-code-quality-audit 1.1
ito tasks complete 003-02_crate-code-quality-audit 1.1
```

---

## Wave 1: High-Coverage Crates (Maintain Quality)

- **Depends On**: None

### Task 1.1: Simplify ito-fs

- **Files**: `ito-rs/crates/ito-fs/src/lib.rs`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files
  - Apply rust-style guidelines
  - Verify 94%+ coverage maintained
- **Verify**: `cargo test -p ito-fs && cargo llvm-cov -p ito-fs --summary-only`
- **Done When**: Code simplified, coverage >= 94%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 1.2: Simplify ito-logging

- **Files**: `ito-rs/crates/ito-logging/src/lib.rs`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files
  - Apply rust-style guidelines
  - Verify 80%+ coverage maintained
- **Verify**: `cargo test -p ito-logging && cargo llvm-cov -p ito-logging --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 1.3: Simplify ito-test-support

- **Files**: `ito-rs/crates/ito-test-support/src/**/*.rs`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files
  - Apply rust-style guidelines
  - Verify 90%+ coverage maintained
- **Verify**: `cargo test -p ito-test-support && cargo llvm-cov -p ito-test-support --summary-only`
- **Done When**: Code simplified, coverage >= 90%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

---

## Wave 2: Medium-Coverage Crates (Boost to 80%)

- **Depends On**: Wave 1

### Task 2.1: Simplify and test ito-templates

- **Files**: `ito-rs/crates/ito-templates/src/lib.rs`, `ito-rs/crates/ito-templates/tests/`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files
  - Identify uncovered code paths (currently 72.9%)
  - Add tests to reach 80%+ coverage
  - Remove duplicate tests
- **Verify**: `cargo test -p ito-templates && cargo llvm-cov -p ito-templates --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 2.2: Simplify and test ito-workflow

- **Files**: `ito-rs/crates/ito-workflow/src/**/*.rs`, `ito-rs/crates/ito-workflow/tests/`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files (especially tasks.rs at 61%)
  - Identify uncovered code paths
  - Add tests for planning.rs, state.rs, workflow.rs
  - Target 80%+ overall coverage
  - Remove duplicate tests
- **Verify**: `cargo test -p ito-workflow && cargo llvm-cov -p ito-workflow --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

---

## Wave 3: Low-Coverage Crates (Major Test Addition)

- **Depends On**: Wave 2

### Task 3.1: Simplify and test ito-schemas

- **Files**: `ito-rs/crates/ito-schemas/src/**/*.rs`, `ito-rs/crates/ito-schemas/tests/`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files
  - Add comprehensive tests for schema parsing/validation
  - Target 80%+ coverage
- **Verify**: `cargo test -p ito-schemas && cargo llvm-cov -p ito-schemas --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

### Task 3.2: Simplify and test ito-harness

- **Files**: `ito-rs/crates/ito-harness/src/**/*.rs`, `ito-rs/crates/ito-harness/tests/`
- **Dependencies**: None
- **Action**:
  - Run @code-simplifier on all source files (currently 0% coverage)
  - Add tests for opencode.rs, stub.rs
  - Create mock harness for testing
  - Target 80%+ coverage
- **Verify**: `cargo test -p ito-harness && cargo llvm-cov -p ito-harness --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-01
- **Status**: [x] complete

---

## Wave 4: File Splitting (Prerequisite for Core Crates)

- **Depends On**: Wave 3

### Task 4.0: Split ito-cli main.rs into modules

- **Files**: `ito-rs/crates/ito-cli/src/main.rs` (4332 lines)
- **Dependencies**: None
- **Action**:
  - Create `ito-rs/crates/ito-cli/src/commands/` module directory
  - Extract command handlers into separate files:
    - `commands/mod.rs` - re-exports
    - `commands/init.rs` - handle_init
    - `commands/create.rs` - handle_create
    - `commands/list.rs` - handle_list
    - `commands/show.rs` - handle_show
    - `commands/validate.rs` - handle_validate
    - `commands/agent.rs` - handle_agent, handle_agent_instruction
    - `commands/tasks.rs` - handle_tasks_*
    - `commands/ralph.rs` - handle_ralph
    - `commands/archive.rs` - handle_archive
    - `commands/help.rs` - HELP constants, handle_help_all
  - Keep main.rs under 300 lines (entry point, arg parsing, dispatch)
  - Each command module should be <500 lines
- **Verify**: `cargo build -p ito-cli && cargo test -p ito-cli`
- **Done When**: main.rs < 300 lines, all commands in separate modules, all tests pass
- **Updated At**: 2026-02-01
- **Status**: [x] complete

---

## Wave 5: Core Crates (Largest Effort)

- **Depends On**: Wave 4

### Task 5.1: Simplify and test ito-core

- **Files**: `ito-rs/crates/ito-core/src/**/*.rs`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 4.0
- **Action**:
  - Run @code-simplifier on all source files
  - Split workflow/mod.rs (993 lines) if needed
  - Priority areas (0% coverage): show/mod.rs, validate/*.rs, repo_index.rs
  - Add tests for installers, config, distribution
  - Target 80%+ overall coverage
  - Remove duplicate tests
- **Verify**: `cargo test -p ito-core && cargo llvm-cov -p ito-core --summary-only`
- **Done When**: Code simplified, all files <1000 lines, coverage >= 80%
- **Updated At**: 2026-02-17
- **Status**: [x] complete

### Task 5.2: Simplify and test ito-cli

- **Files**: `ito-rs/crates/ito-cli/src/**/*.rs`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 4.0, Task 5.1
- **Action**:
  - Run @code-simplifier on each command module
  - Add integration tests for CLI commands
  - Test error handling paths
  - Target 80%+ coverage
  - Remove duplicate tests
- **Verify**: `cargo test -p ito-cli && cargo llvm-cov -p ito-cli --summary-only`
- **Done When**: Code simplified, coverage >= 80%
- **Updated At**: 2026-02-17
- **Status**: [x] complete

---

## Wave 6: Final Verification

- **Depends On**: Wave 5

### Task 6.1: Verify overall coverage target

- **Files**: All crates
- **Dependencies**: Task 5.1, Task 5.2
- **Action**:
  - Run full workspace coverage report
  - Verify overall coverage >= 80%
  - Verify all source files < 1000 lines
  - Document any exceptions with justification
- **Verify**: `cargo llvm-cov --workspace --summary-only && wc -l ito-rs/crates/*/src/**/*.rs | sort -rn | head -10`
- **Done When**: Overall coverage >= 80%, all files < 1000 lines, or exceptions documented
- **Updated At**: 2026-02-17
- **Status**: [x] complete

### Task 6.2: Review and checkpoint

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: Coverage report, simplified code
- **Dependencies**: Task 6.1
- **Action**:
  - Human review of coverage report
  - Verify code quality improvements
  - Verify file size compliance
  - Approve for archive
- **Done When**: Human approves changes
- **Updated At**: 2026-02-17
- **Status**: [x] complete

---

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
