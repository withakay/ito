## Why

Some environments prefer GitHub Copilot CLI as the default agent interface. Adding a `github-copilot` harness to `ito ralph` allows iterative loops without switching tools.

## What Changes

- Add a `github-copilot` harness to `ito ralph` / `ito loop` via `--harness github-copilot`.
- Implement the harness using the GitHub CLI Copilot subcommands (e.g., `gh copilot ...`) when available.
- Support `--model` and `--allow-all` only when the underlying tool supports them; otherwise warn and proceed with best-effort defaults.

## Capabilities

### Modified Capabilities

- `cli-ralph`: add `github-copilot` as a supported harness.

## Impact

- Adds a new external dependency path (GitHub CLI with Copilot enabled).
- Some Copilot flows may be interactive; the harness will document non-interactive limitations.
