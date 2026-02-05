## Why

When running `ito list`, changes with no tasks.md file or empty tasks show as "no-tasks" status, making it impossible to distinguish between changes that are genuinely completed vs. those that were never properly planned. This makes archiving workflows confusing - users can't easily identify which changes are ready to archive. Additionally, the `/ito-archive` skill requires a change ID but doesn't help users discover which changes are archivable.

## What Changes

- Add a "completed" status for changes where all tasks are done (currently only shows task count)
- Add a clear visual indicator for completed changes in `ito list` output
- Introduce interactive selection when `/ito-archive` is called without a change ID
- Show only completed/archivable changes when prompting for archive selection
- Consider adding a `--completed` filter flag to `ito list`

## Capabilities

### New Capabilities

- `completed-status-display`: Add explicit "completed" status indicator in `ito list` output when all tasks in a change are marked done, distinguishing from "no-tasks" and "in-progress" states.

- `interactive-archive-selection`: When `ito archive` (or `/ito-archive`) is invoked without a change ID, prompt the user with a list of completed changes to select from rather than failing or requiring a specific ID upfront.

### Modified Capabilities

None - this change adds new functionality without modifying existing spec-level behavior.

## Impact

- **CLI**: `ito list` output format will show "completed" status
- **ito-archive skill**: Needs update to support interactive selection flow
- **User workflows**: Clearer path from completion to archiving
- **Code affected**:
  - `ito-rs/crates/ito-core/src/list.rs` (status logic)
  - `ito-rs/crates/ito-cli/src/commands/list.rs` (display)
  - `.claude/skills/ito-archive/` (skill instructions)
