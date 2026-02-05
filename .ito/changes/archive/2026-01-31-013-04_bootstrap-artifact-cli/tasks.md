# Tasks for: 013-04_bootstrap-artifact-cli

## Execution Notes

- **Tool**: Any
- **Mode**: Sequential
- **Created**: 2026-01-31
- **Rust**: Implementation MUST follow the `rust-style` skill

```bash
ito tasks status 013-04_bootstrap-artifact-cli
ito tasks next 013-04_bootstrap-artifact-cli
ito tasks start 013-04_bootstrap-artifact-cli 1.1
ito tasks complete 013-04_bootstrap-artifact-cli 1.1
ito tasks show 013-04_bootstrap-artifact-cli
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement `bootstrap` artifact for `ito agent instruction` with `--tool`

- **Files**: `ito-rs/crates/ito-core/src/ralph/`, `ito-rs/crates/ito-cli/`, `.ito/changes/013-04_bootstrap-artifact-cli/design.md`
- **Dependencies**: None
- **Action**:
  - Add a new instruction artifact: `bootstrap`.
  - Support `--tool opencode|claude|codex` (and validate values).
  - Output a short preamble that:
    - Mentions how to retrieve workflow bodies via other artifacts.
    - Includes tool-specific notes only where tools differ.
  - Apply the `rust-style` skill for all Rust changes (formatting, structure, naming).
- **Verify**: `make test`
- **Done When**: `ito agent instruction bootstrap --tool <tool>` produces stable content for all three tools
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Add tests for bootstrap output shape and tool switching

- **Files**: `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 1.1
- **Action**:
  - Add tests that assert:
    - Command succeeds for the three supported tools.
    - Output contains pointers to key artifacts (proposal/apply/review/archive).
    - Unknown tools error with a clear message.
- **Verify**: `make test`
- **Done When**: Tests fail without implementation and pass with it
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Files**: `.ito/changes/013-04_bootstrap-artifact-cli/proposal.md`, `.ito/changes/013-04_bootstrap-artifact-cli/design.md`
- **Dependencies**: None
- **Action**: Review that bootstrap content is short and keeps workflows centralized
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [x] completed
