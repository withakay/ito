## Why

Some environments standardize on the Codex CLI for agent execution. Supporting a `codex` harness in `ito ralph` enables the same iterative loop workflow across toolchains.

## What Changes

- Add a `codex` harness to `ito ralph` / `ito loop` via `--harness codex`.
- Support passing `--model` through to the Codex CLI when supported.
- Support `--allow-all` (and aliases) by mapping to the closest available non-interactive mode for Codex.

## Capabilities

### Modified Capabilities

- `cli-ralph`: add `codex` as a supported harness.

## Impact

- Adds a new external dependency path (the Codex CLI must be installed and available on PATH).
