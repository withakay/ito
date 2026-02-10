# Change: Add `--completed` flag to `ito archive`

## Why

Archiving completed changes is a frequent end-of-sprint activity. Currently each change must be archived individually with `ito archive <change-id>`, which is tedious when multiple changes have reached completion. A batch mode reduces manual repetition and aligns with the existing `ito list --completed` filter.

## What Changes

- Add `--completed` flag to `ito archive` that discovers and archives all changes with `ChangeStatus::Complete`
- Each change is archived sequentially using the existing single-change archive logic (spec updates, module marking, move to archive)
- The flag is mutually exclusive with a positional `CHANGE` argument
- Respects existing flags (`--yes`, `--skip-specs`, `--no-validate`)
- Reports per-change progress and a summary on completion

## Capabilities

### New Capabilities

_(none -- this extends the existing `cli-archive` capability)_

### Modified Capabilities

- `cli-archive`: Add batch archive mode via `--completed` flag

## Impact

- **Affected specs**: `cli-archive`
- **Affected code**:
  - `ito-rs/crates/ito-cli/src/cli.rs` (add `--completed` to `ArchiveArgs`)
  - `ito-rs/crates/ito-cli/src/app/archive.rs` (batch dispatch loop, per-change error handling)
  - `ito-rs/crates/ito-core/src/change_repository.rs` (already provides `list_complete()`)
- **Dependencies**: None new -- leverages `ChangeRepository::list_complete()` which already exists
