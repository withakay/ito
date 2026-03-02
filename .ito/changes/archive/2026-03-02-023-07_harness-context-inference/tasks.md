# Tasks for: 023-07_harness-context-inference

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 023-07_harness-context-inference
ito tasks next 023-07_harness-context-inference
ito tasks start 023-07_harness-context-inference 1.1
ito tasks complete 023-07_harness-context-inference 1.1
ito tasks shelve 023-07_harness-context-inference 1.1
ito tasks unshelve 023-07_harness-context-inference 1.1
ito tasks show 023-07_harness-context-inference
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Author proposal, delta spec, and tasks

- **Files**: `.ito/changes/023-07_harness-context-inference/**`
- **Dependencies**: None
- **Action**:
  - Write `proposal.md`, `design.md` (if needed), `tasks.md`.
  - Add at least one delta spec under `specs/` with valid requirement/scenario formatting.
- **Verify**: `ito validate 023-07_harness-context-inference --strict`
- **Done When**: Validation passes and the proposal is ready for review.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement core inference API

- **Files**: `ito-rs/crates/ito-core/src/**`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: None
- **Action**:
  - Add a core function that infers the current Ito target (change/module/none) from local signals.
  - Ensure inference is deterministic and conservative.
- **Verify**: `make test`
- **Done When**: Core tests cover the inference ordering and false-positive guardrails.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.2: Wire `ito agent instruction context` (+ JSON)

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: 2.1
- **Action**:
  - Add a new instruction artifact `context` that emits a short continuation snippet.
  - Support `--json` output for harness scripts/plugins.
- **Verify**: `make test`
- **Done When**: CLI integration tests validate both text and JSON output.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.3: Update harness adapters to consume the entrypoint

- **Files**: `.opencode/**`, `.claude/**`, `.github/**`, `ito-rs/crates/ito-templates/assets/**`
- **Dependencies**: 2.2
- **Action**:
  - OpenCode: toasts + compaction continuation injection via the Ito entrypoint.
  - Claude: SessionStart + PreCompact hook scripts that inject the continuation snippet.
  - GitHub Copilot CLI: add custom instructions referencing the entrypoint; hooks are best-effort only.
- **Verify**: `make test`
- **Done When**: Adapters remain thin and call Ito for inference.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review the proposal and spec delta.
- **Done When**: User approves the proposal.
- **Updated At**: 2026-02-25
- **Status**: [x] complete
