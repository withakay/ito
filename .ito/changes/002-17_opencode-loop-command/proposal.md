<!-- ITO:START -->
## Why

Running `ito ralph` from within an OpenCode session is currently clunky: you have to remember the right flags, copy/paste a change id, and if the loop exits unexpectedly you have to manually restart and re-seed context.

We want a simple OpenCode slash command that starts a Ralph loop for a change without leaving the session, using stable defaults and adding restart context automatically when the loop is restarted.

## What Changes

- Add an OpenCode slash command `/loop` that starts an Ito Ralph run for a change id.
- Provide a thin, centralized workflow wrapper (skill + command) that:
  - Uses the OpenCode harness defaults (`ito ralph --harness opencode`).
  - Supports non-interactive runs (OpenCode `Bash` tool is non-interactive).
  - Restarts Ralph when it exits early (non-zero / unexpected termination), appending a restart note into the Ralph context.
  - Optionally passes a model id; when not provided, it relies on OpenCode defaults.

Notes:

- Ito already supports adding extra loop context via `ito ralph --add-context` and inspecting progress via `ito ralph --status`.
- Ito already supports “gummed up” iteration restarts via `ito ralph --timeout` (inactivity timeout).

## Capabilities

### New Capabilities

- `opencode-loop-command`: Provide `/loop` in OpenCode to run Ralph on a change with restart-context support.

### Modified Capabilities

<!-- None -->

## Impact

- **Templates**: add a new OpenCode command definition (`loop.md`) installed by `ito init`.
- **Skills**: add a new `ito-loop` skill that standardizes the wrapper behavior and works in any harness.
- **CLI**: no new flags required; wrapper composes `ito ralph` with existing flags (`--timeout`, `--add-context`, `--status`, `--model`, `--max-iterations`).
<!-- ITO:END -->
