# Tasks for: 030-02_deterministic-completion-gate

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-02_deterministic-completion-gate
ito tasks next 030-02_deterministic-completion-gate
ito tasks start 030-02_deterministic-completion-gate 1.1
ito tasks complete 030-02_deterministic-completion-gate 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define verifier verdict contract

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for complete, incomplete, and blocked verifier verdicts with JSON blocking reasons and evidence.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests fail before implementation and cover checkbox-only false positives.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Cover integration expectations

- **Files**: `ito-rs/crates/ito-core/src/ralph/**`, `ito-rs/crates/ito-core/src/archive.rs`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add regression tests proving Ralph and archive reject completion when the verifier returns incomplete.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Ralph and archive tests assert verifier reasons are surfaced.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement core completion verifier

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-domain/src/**`
- **Dependencies**: None
- **Action**: Implement verifier checks for required artifacts, parsed task state, artifact validation, archive eligibility, and relevant git/worktree state.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Core tests return deterministic verdicts and structured blocking reasons.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Add verifier CLI surface

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add `ito change verify-complete <change-id> --json` and human-readable fallback output.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: The command returns valid JSON and actionable text output.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Integrate verifier into workflows

- **Files**: `ito-rs/crates/ito-core/src/ralph/**`, `ito-rs/crates/ito-core/src/archive.rs`, `ito-rs/crates/ito-templates/assets/**`
- **Dependencies**: None
- **Action**: Route Ralph completion handling and archive default behavior through the verifier, then update guidance to avoid checkbox-only completion decisions.
- **Verify**: `make check`
- **Done When**: Integrations use the verifier and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
