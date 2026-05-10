# Change: Add archived filter to `ito list`

## Why
Archived changes are currently listed through the separate `ito list-archive` command, while active change filters live under `ito list`. Adding `ito list --archived` makes archived-change discovery consistent with the rest of the list filters.

## What Changes
- Add an `--archived` filter to `ito list`.
- Make `ito list --archived` list archived changes and exclude active changes.
- Preserve `ito list-archive` as an existing command unless a later change explicitly removes it.

## Impact
- Affected specs: `cli-list`
- Affected code: `ito-rs` CLI argument parsing, list command dispatch, and list/archive output tests
