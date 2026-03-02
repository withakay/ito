## Why

`ito state` has low demonstrated usage and unclear product value, while adding maintenance and spec drift cost. Removing it simplifies the CLI surface and avoids carrying a partially adopted command group.

## What Changes

- Remove the top-level `ito state` command and all subcommands (`show`, `decision`, `blocker`, `note`, `focus`, `question`)
- Remove command routing and command help text for `state`
- Remove `ito stats` built-in counters for `ito.state.*` command IDs
- Remove `state` command tests and update snapshots/help expectations
- Update specs to explicitly remove the state-management requirement and clarify that state tracking is no longer a CLI command
- **BREAKING**: `ito state ...` invocations will stop working

## Capabilities

### New Capabilities

- None

### Modified Capabilities

- `cli-plan`: remove CLI state-management requirement and command exposure
- `rust-planning-and-state`: remove command-parity expectations for `state` commands

## Impact

- Affected code:
  - `ito-rs/crates/ito-cli/src/cli.rs`
  - `ito-rs/crates/ito-cli/src/app/run.rs`
  - `ito-rs/crates/ito-cli/src/commands/state.rs` (deleted)
  - `ito-rs/crates/ito-cli/src/commands/stats.rs`
  - `ito-rs/crates/ito-domain/src/state.rs` (deleted if unused)
  - related tests/snapshots under `ito-rs/crates/ito-cli/tests/`
- Affected behavior:
  - users must update any automation that calls `ito state ...`
  - state notes/decisions/focus are no longer managed via CLI command
- Compatibility:
  - breaking CLI change; release notes and migration guidance are required
