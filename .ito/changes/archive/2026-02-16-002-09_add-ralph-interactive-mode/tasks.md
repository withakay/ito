# Tasks for: 002-09_add-ralph-interactive-mode

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 002-09_add-ralph-interactive-mode
ito tasks next 002-09_add-ralph-interactive-mode
ito tasks start 002-09_add-ralph-interactive-mode 1.1
ito tasks complete 002-09_add-ralph-interactive-mode 1.1
ito tasks shelve 002-09_add-ralph-interactive-mode 1.1
ito tasks unshelve 002-09_add-ralph-interactive-mode 1.1
ito tasks show 002-09_add-ralph-interactive-mode
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Implement interactive change picker in CLI

- **Files**: `ito-rs/crates/ito-cli/src/app/ralph.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Add interactive selection when `--change` is omitted and `--no-interactive` is not set.
  - If `--module <id>` is provided and it contains multiple changes, prompt to select one or more.
  - If neither `--change` nor `--module` is provided, prompt to select one or more changes from the repo.
  - If `--status`, `--add-context`, or `--clear-context` is used without `--change`, prompt to select exactly one change.
  - Exclude archived changes (anything under `.ito/changes/archive/`).
  - Present changes in a stable order (sorted by change id) and execute in that presented order.
  - Execute Ralph sequentially for each selected change.
  - Cancellation exits non-zero with a clear cancellation message.
- **Verify**: `make check`
- **Done When**: Manual run confirms:
  - `ito ralph` prompts and runs selected change(s)
  - `ito ralph --module 002` prompts when multiple changes exist
  - `ito ralph --no-interactive` without an explicit target fails with a clear error
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 1.2: Improve core error messaging for missing interactive selection

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  Replace placeholder "Interactive selection is not yet implemented" errors with messages that:
  - clearly explain how to resolve (use `--change` / `--module` or run via `ito ralph` with interactive selection)
  - do not imply a missing feature once the CLI implements selection
- **Verify**: `make check`
- **Done When**: error output is actionable and no longer claims the feature is unimplemented
- **Updated At**: 2026-02-16
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-02-16
- **Status**: [x] complete
