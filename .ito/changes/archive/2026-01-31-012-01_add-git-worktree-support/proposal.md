## Why

Ito currently assumes a single working directory, which makes it easy to mix unrelated edits and accidentally commit local environment files. Git worktrees can isolate each change into its own checkout while keeping a small, stable entry directory for the agent.

## What Changes

- Add an optional Git worktree-based workspace layout that uses a small repo root checkout for orchestration and separate worktrees for `main` and per-change branches.
- Generate deterministic, copy/pasteable shell scripts in apply-time agent instructions for creating/selecting the right worktree and paths.
- Copy configurable local environment files (initially `.env`, `.envrc`, and Mise local config) into newly created change worktrees.
- Provide configuration to control default branch assumptions (default: `main`, with fallback to `master`) and which local files to copy.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `cli-init`: Add opt-in workspace layout initialization for Git worktrees.
- `cli-artifact-workflow`: Emit worktree-aware apply instructions (including scripts and working directory guidance).
- `global-config`: Add user-level defaults for worktree behavior and local file copy patterns.
- `cli-config`: Expose config keys needed to control the new worktree behavior.

## Impact

- Touches init/apply workflows and how Ito guides agents on where to run commands.
- Introduces Git worktree and sparse-checkout assumptions that must be clearly documented and strictly opt-in.
- Adds handling for local environment files; requires care to avoid accidentally committing secrets.
