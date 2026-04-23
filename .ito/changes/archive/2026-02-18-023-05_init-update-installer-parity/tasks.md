# Tasks for: 023-05_init-update-installer-parity

## Execution Notes

- **Tool**: Rust (ito-core + ito-cli)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Reproduce current init/update inconsistencies in tests

- **Files**: `ito-rs/crates/ito-cli/tests/update_smoke.rs`, `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: None
- **Action**: Add failing tests that demonstrate:
  - update clobbers a file that should be preserved
  - init without force blocks in a surprising way
- **Verify**: `make test`
- **Done When**: Tests fail on current behavior with clear assertions.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

### Task 1.2: Define installer ownership policy in code

- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: Task 1.1
- **Action**: Implement an explicit per-path policy (Ito-managed overwrite, marker-managed merge, user-owned preserve).
- **Verify**: `make test`
- **Done When**: The policy is encoded and readable, and tests are updated accordingly.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

### Task 1.3: Align CLI options for update

- **Files**: `ito-rs/crates/ito-cli/src/app/update.rs`
- **Dependencies**: Task 1.2
- **Action**: Ensure `ito update` passes installer options consistent with update semantics.
- **Verify**: `make test`
- **Done When**: CLI update flow no longer relies on force semantics and behaves deterministically.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Expand coverage across harness directories

- **Files**: Installer tests
- **Dependencies**: None
- **Action**: Add regression cases for `.opencode/`, `.claude/`, `.github/`, `.codex/` installation and update refresh.
- **Verify**: `make test`
- **Done When**: Coverage includes at least one representative file from each harness.
- **Updated At**: 2026-02-18
- **Status**: [x] complete

### Task 2.2: Validate change

- **Files**: N/A
- **Dependencies**: Task 2.1
- **Action**: Run strict validation for the change.
- **Verify**: `ito validate 023-05_init-update-installer-parity --strict`
- **Done When**: Validation passes.
- **Updated At**: 2026-02-18
- **Status**: [x] complete
