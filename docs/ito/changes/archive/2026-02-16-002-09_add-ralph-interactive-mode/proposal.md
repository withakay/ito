## Why

Ralph currently requires `--change` (or a module with exactly one change) and errors out when interactive selection would be most useful. Adding interactive selection makes `ito ralph` faster to use and enables batch execution across multiple changes without re-invoking the command.

## What Changes

- Add interactive target selection to `ito ralph` when `--no-interactive` is not set and `--change` is omitted.
- Support selecting one OR multiple changes, then run Ralph sequentially for each selected change.
- When `--module <id>` is provided and the module contains multiple changes, prompt to select one or more changes from that module.
- When `--status`, `--add-context`, or `--clear-context` is used without `--change`, prompt to select exactly one change.
- Ensure selection is cancelable and produces a clear, non-zero exit on cancellation.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `cli-ralph`: add interactive selection behavior and multi-change execution.

## Impact

- Affected code: `ito-rs/crates/ito-cli/src/app/ralph.rs`, `ito-rs/crates/ito-core/src/ralph/runner.rs` (target resolution), CLI help/snapshots/tests.
- UX: introduces new interactive prompts in default mode (can be disabled with `--no-interactive`).
