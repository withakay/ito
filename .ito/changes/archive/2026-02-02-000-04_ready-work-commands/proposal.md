# Proposal: Ready Work Commands

## Why

Currently there's no quick way to discover what work is ready to be implemented. Users must manually inspect changes and tasks to determine what can be worked on next. Adding "ready work" commands will streamline the workflow for both humans and AI agents to identify actionable items.

## What Changes

- Add `--ready` flag to `ito list` command to filter changes that are ready for implementation (proposal + specs complete, has pending tasks)
- Add `ito tasks ready [CHANGE]` subcommand to show ready tasks:
  - Without change argument: shows all ready tasks across all changes
  - With change argument: shows ready tasks for that specific change
- A "ready" task is defined as a pending task in the earliest incomplete wave

## Capabilities

### New Capabilities

- `ready-work-filter`: Filtering and display of work items (changes and tasks) that are ready for implementation

### Modified Capabilities

<!-- No existing spec-level behavior changes required -->

## Impact

- **Code**: `ito-cli` crate - `cli.rs` (args), `list.rs` (ready filter), tasks handler (new subcommand)
- **APIs**: New CLI flags and subcommands (additive, non-breaking)
- **Dependencies**: May need to expose additional query methods from `ito-domain` if not already available
