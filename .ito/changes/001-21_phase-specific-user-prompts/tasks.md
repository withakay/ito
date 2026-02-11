# Tasks for: 001-21_phase-specific-user-prompts

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use Ito tasks CLI for all status changes

```bash
ito tasks status 001-21_phase-specific-user-prompts
ito tasks next 001-21_phase-specific-user-prompts
ito tasks start 001-21_phase-specific-user-prompts 1.1
ito tasks complete 001-21_phase-specific-user-prompts 1.1
ito tasks show 001-21_phase-specific-user-prompts
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add scoped guidance file resolution

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs` (or split module), related helpers
- **Dependencies**: None
- **Action**:
  Add lookup support for `.ito/user-prompts/<artifact-id>.md` and expose it to instruction generation.
- **Verify**: `cargo test -p ito-core workflow`
- **Done When**: Scoped prompt lookup succeeds for proposal/apply and missing files are non-fatal.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.2: Compose shared + scoped guidance in instruction output

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`, formatter/output structs
- **Dependencies**: Task 1.1
- **Action**:
  Merge artifact-scoped guidance with shared `.ito/user-guidance.md` in deterministic output order while preserving schema authority.
- **Verify**: `cargo test -p ito-core workflow`
- **Done When**: Proposal/apply instructions include both guidance sources when available.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add CLI/integration tests for scoped guidance

- **Files**: `ito-rs/crates/ito-cli/tests/*`, `ito-rs/crates/ito-core/tests/*`
- **Dependencies**: None
- **Action**:
  Add tests for proposal/apply scoped guidance, fallback behavior, and additive composition with shared guidance.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-core`
- **Done When**: New tests fail before implementation and pass after implementation.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.2: Document `.ito/user-prompts` usage

- **Files**: `docs/` guidance docs, possibly `.ito/user-guidance.md` managed text
- **Dependencies**: Task 2.1
- **Action**:
  Document naming convention (`<artifact-id>.md`) and examples for proposal/apply files.
- **Verify**: `make check`
- **Done When**: Docs clearly explain scoped and shared guidance behavior.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Final validation

- **Files**: change artifacts and touched implementation files
- **Dependencies**: None
- **Action**:
  Run full validation and ensure change artifacts are strict-valid.
- **Verify**: `make check && make test && ito validate 001-21_phase-specific-user-prompts --strict`
- **Done When**: Quality gates pass and change validates cleanly.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
