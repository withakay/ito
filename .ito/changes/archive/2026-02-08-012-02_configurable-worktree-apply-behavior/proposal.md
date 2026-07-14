## Why

The current worktree flow is mostly hard-coded, so teams cannot tune when worktrees are used, how local files are copied, or how branch integration should happen after implementation. We need a configurable worktree policy that lets projects opt in safely while preserving a deterministic `ito agent instruction apply` workflow.

## What Changes

- Add a structured `worktrees` configuration object in project/global config for enabling worktree mode and controlling apply-time behavior.
- Add a codified `worktrees.strategy` enum so users explicitly pick one supported workflow strategy, with no ad-hoc/custom topology modes.
- Extend apply instructions to conditionally inject worktree setup steps when configuration enables them, including copy patterns for uncommitted local files.
- Add configurable worktree bootstrap command hooks (for example `direnv allow`) that run in the change worktree before implementation starts.
- Add integration/cleanup guidance in apply instructions so agents can either prepare a commit+PR flow or a merge-into-parent flow, then provide deterministic cleanup steps after merge.
- Add interactive worktree setup prompts during `ito init` (always) and `ito update` (when worktree config is not yet set), so users are guided through enabling worktrees, choosing a strategy, and setting integration mode. Choices are auto-persisted to config and the user is informed of the config file location.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `global-config`: Add the nested worktree policy shape and defaults used by instruction generation.
- `cli-config`: Add CLI set/get support for nested worktree keys and list values used by policy configuration.
- `cli-artifact-workflow`: Make apply instructions configuration-aware for strategy-based layout setup, worktree setup, copy patterns, integration strategy prompts, and cleanup instructions.
- `cli-init`: Add interactive worktree setup prompts during `ito init` that ask whether to enable worktrees, which strategy to use, and what integration mode to prefer. Auto-persist answers to config and display the config file path.
- `cli-update`: Add interactive worktree setup prompts during `ito update` when worktree config is not yet set (first upgrade), with the same flow as init. Skip the prompt if config already exists.

## Impact

- Affected specs: `global-config`, `cli-config`, `cli-artifact-workflow`, `cli-init`, and `cli-update`.
- Affected code: config schema/defaults, config command path handling, apply instruction rendering, worktree path/layout helper utilities, init command interactive prompts, and update command upgrade-time prompts.
- Affected users: teams adopting worktree mode gain configurable behavior; existing users keep current behavior by default unless they opt in. Users running `ito init` or upgrading via `ito update` are prompted to configure worktrees interactively.
- **Key renames**: This change restructures worktree config keys from the flat camelCase names introduced in `012-01` to a nested snake_case layout. Existing config files using the old keys will need migration:
  - `worktrees.defaultBranch` → `worktrees.default_branch`
  - `worktrees.localFiles` → `worktrees.apply.copy_from_main`
  - Config loading SHALL silently accept the legacy keys as aliases during a deprecation window and emit a warning recommending the new names.
