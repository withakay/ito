# Tasks for: 023-04_codex-audit-instructions

## Execution Notes

- **Tool**: Codex templates + Rust (installer/tests)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define Codex audit instruction assets

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.codex/instructions/`, `ito-rs/crates/ito-templates/assets/default/project/.codex/prompts/`
- **Dependencies**: None
- **Action**: Add a dedicated audit instruction file and reference it from existing prompts if needed.
- **Verify**: `make test -p ito-templates` (or workspace tests)
- **Done When**: Audit instructions exist and are installed for Codex.
- **Updated At**: 2026-02-17
- **Status**: [ ] pending

### Task 1.2: Add installer tests for `.codex/` assets

- **Files**: `ito-rs/crates/ito-cli/tests/update_smoke.rs` (or new tests)
- **Dependencies**: Task 1.1
- **Action**: Verify init installs and update refreshes the Codex instruction assets.
- **Verify**: `make test`
- **Done When**: Tests cover Codex asset install/update deterministically.
- **Updated At**: 2026-02-17
- **Status**: [ ] pending
