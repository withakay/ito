# Tasks for: 022-01_separate-tests-into-foo-tests

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 022-01_separate-tests-into-foo-tests
ito tasks next 022-01_separate-tests-into-foo-tests
ito tasks start 022-01_separate-tests-into-foo-tests 1.1
ito tasks complete 022-01_separate-tests-into-foo-tests 1.1
ito tasks shelve 022-01_separate-tests-into-foo-tests 1.1
ito tasks unshelve 022-01_separate-tests-into-foo-tests 1.1
ito tasks show 022-01_separate-tests-into-foo-tests
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Document the `*_tests.rs` convention

- **Files**: `AGENTS.md`, `ito-rs/AGENTS.md`, `.ito/user-rust-style.md`
- **Dependencies**: None
- **Action**:
  Define the repository standard for Rust unit tests living in sibling `*_tests.rs` files, including:
  - How to name/locate test files for both `foo.rs` and `foo/mod.rs`
  - How to include the test module (`#[cfg(test)] mod foo_tests;`)
  - Clear guidance on what is in-scope (unit tests) vs out-of-scope (integration tests under `tests/`)
  - Any allowed exceptions (if any)
- **Verify**: `make check`
- **Done When**: The convention is documented with at least one concrete example and is easy to find from contributor guidance
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 1.2: Decide enforcement approach

- **Files**: `.pre-commit-config.yaml`, `Makefile`, `scripts/`
- **Dependencies**: Task 1.1
- **Action**:
  Decide whether the convention is enforced (pre-commit/CI) or guidance-only. If enforced, define what is checked (e.g., disallow new inline `#[cfg(test)] mod tests { ... }` blocks) and what is exempt.
- **Verify**: `make check`
- **Done When**: Enforcement decision is recorded (and, if enforced, the intended check behavior is clearly described)
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 1.3: Review convention + enforcement

- **Type**: checkpoint (requires human approval)
- **Files**: `AGENTS.md`, `ito-rs/AGENTS.md`, `.ito/user-rust-style.md`, `.pre-commit-config.yaml`, `Makefile`, `scripts/`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: Confirm the convention details and (if applicable) enforcement approach before proceeding with broad refactors
- **Done When**: User confirms the convention and enforcement decision
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Migrate existing inline unit tests to sibling files

- **Files**: `ito-rs/crates/**/src/**/*.rs`
- **Dependencies**: Wave 1
- **Action**:
  Move existing unit tests out of production modules and into the corresponding sibling `*_tests.rs` files, updating module declarations and imports so behavior stays identical.
- **Verify**: `cargo test --workspace`
- **Done When**: All moved tests compile and pass, and production modules no longer embed large inline unit test blocks where a sibling `*_tests.rs` file is expected
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 2.2: Run repository quality gates

- **Files**: (none)
- **Dependencies**: Task 2.1
- **Action**:
  Run the project quality gates and fix any breakage introduced by the refactor.
- **Verify**: `make check`
- **Done When**: All checks pass locally
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

______________________________________________________________________
