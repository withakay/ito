## Why

Concurrent agents can race when creating or updating change proposals, which increases the chance of duplicate IDs and merge conflicts in `.ito/changes/`. We need a deliberate, tool-owned coordination path that syncs early and often without requiring direct pushes to protected `main`.

## What Changes

- Add an internal coordination branch workflow that defaults to direct-push sync on `ito/internal/changes`.
- Introduce configuration to enable/disable coordination-branch usage and override the branch name when needed.
- Update change lifecycle operations to sync the coordination branch before proposal creation and before task execution entry points.
- Ensure synchronization behavior is transparent to users (no branch switching in their working tree, no unexpected workspace mutation).
- Define deterministic conflict/failure handling for non-fast-forward pushes, branch protection failures, and offline operation.

## Capabilities

### New Capabilities

- `change-coordination-branch`: Internal branch-based coordination for proposal and task lifecycle operations, including defaults, sync behavior, and failure handling.

### Modified Capabilities

- None.

## Impact

- **Config and defaults**: Adds coordination-branch settings and defaults in config schema/defaults.
- **Change creation flow**: `ito create change` integrates pre-sync and immediate reservation push behavior.
- **Instruction/task entry points**: `ito agent instruction apply` and task-start lifecycle operations consume the coordination branch as source-of-truth.
- **Git execution layer**: Adds robust fetch/rebase/push behavior with deterministic handling of conflicts and protected remotes.
