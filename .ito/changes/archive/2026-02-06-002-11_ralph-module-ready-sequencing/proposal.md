# Change: Ralph module-ready sequencing and continuous module mode

## Why

Ralph previously treated `--module` as ambiguous when multiple changes existed, which forced users to manually select a change and made module-level automation brittle.

We also need module execution to tolerate drift: if another process updates task state while Ralph is running, Ralph should re-validate module readiness and reorient to the correct next change.

## What Changes

- Update `ito ralph --module <id>` to list ready changes and automatically select the first ready change by lowest change ID
- Add `--continue-module` to keep processing ready module changes until the module is complete
- Add start/end loop validation in module-continuation mode so Ralph can detect module-state drift and reorient before each run
- Preserve clear failure behavior when no ready changes remain but non-complete changes still exist
- Update CLI help and tests for module selection and continuation behavior

## Retrospective Note

This proposal is retrospective: implementation and tests have already been completed in the Rust CLI/core codepaths, and this change records that behavior in Ito artifacts.

## Impact

- **Affected specs**: `cli-ralph`
- **Affected code**:
  - `ito-rs/crates/ito-cli/src/cli.rs`
  - `ito-rs/crates/ito-cli/src/app/ralph.rs`
  - `ito-rs/crates/ito-core/src/ralph/runner.rs`
  - `ito-rs/crates/ito-core/tests/ralph.rs`
  - `ito-rs/crates/ito-cli/tests/snapshots/cli_snapshots__ito_ralph_help.snap`
- **User behavior change**:
  - `--module` now chooses the first ready change automatically
  - `--continue-module` enables full module progression with drift-aware revalidation
