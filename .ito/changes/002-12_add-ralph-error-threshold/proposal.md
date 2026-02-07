# Change: Add Ralph harness error threshold

## Why

Ralph now continues iterating after non-zero harness exits so it can repair build and test failures, but repeated harness failures can still lead to long, unproductive loops. We need a configurable safety bound that preserves self-healing behavior while surfacing persistently failing runs.

## What Changes

- Add a configurable non-zero harness exit threshold to Ralph loop options.
- Default the threshold to 10 non-zero harness exits before Ralph fails the run.
- Add a new `--error-threshold <n>` CLI flag to override the default threshold.
- Keep existing `--exit-on-error` fail-fast behavior for users who want immediate exit on first non-zero harness run.
- Update Ralph tests and CLI help snapshots to cover the new option and semantics.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cli-ralph`: Ralph continues on harness failures by default but now enforces a configurable non-zero exit threshold with a default of 10.

## Impact

- **Affected specs**: `cli-ralph`
- **Affected code**:
  - `ito-rs/crates/ito-core/src/ralph/runner.rs`
  - `ito-rs/crates/ito-core/src/ralph/mod.rs`
  - `ito-rs/crates/ito-cli/src/cli.rs`
  - `ito-rs/crates/ito-cli/src/app/ralph.rs`
  - `ito-rs/crates/ito-core/tests/ralph.rs`
  - `ito-rs/crates/ito-cli/tests/snapshots/`
- **User impact**: Ralph remains resilient to transient build failures, but reliably exits when failures persist past the configured threshold.
