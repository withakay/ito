## Why

Running `ito ralph` across multiple ready changes currently requires a manual loop (pick a change, run Ralph, repeat). This makes it harder to keep a backlog moving and turns “clear the ready queue” into repetitive CLI work.

## What Changes

- Add a new `ito ralph` flag to automatically select the next available ready change.
- After a change run completes, re-scan for ready changes and continue with the next one.
- Exit successfully once no further ready changes remain (or error if work remains but nothing is ready).
- Keep the existing `--change` and `--module/--continue-module` flows unchanged.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `cli-ralph`: Add a repo-wide “continue through ready changes” mode.

## Impact

- **Affected code** (expected):
  - `ito-rs/crates/ito-cli/src/cli.rs`
  - `ito-rs/crates/ito-cli/src/app/ralph.rs` (or `ito-rs/crates/ito-cli/src/commands/ralph.rs`, depending on the in-flight refactor)
  - `ito-rs/crates/ito-core/src/ralph/runner.rs`
  - `ito-rs/crates/ito-core/tests/ralph.rs`
  - `ito-rs/crates/ito-cli/tests/ralph_smoke.rs`
