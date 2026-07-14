## Why

Some teams standardize on Claude Code as their primary agent runner. Adding a `claude-code` harness to `ito ralph` lets users reuse the same Ralph loop workflow without requiring OpenCode.

## What Changes

- Add a `claude-code` harness to `ito ralph` / `ito loop` via `--harness claude-code`.
- Support passing `--model` through to the Claude Code CLI when supported.
- Support `--allow-all` (and aliases) by mapping to the closest non-interactive/auto-approve mode available for the harness.

## Capabilities

### Modified Capabilities

- `cli-ralph`: add `claude-code` as a supported harness.

## Impact

- Adds a new external dependency path (the Claude Code CLI must be installed and available on PATH).
- Harness flag mapping may vary across CLI versions; we will document the supported flags.
