<!-- ITO:START -->
## Why

Ito workflows (proposal viewer, ralph loop, etc.) can surface tmux-specific options — but not every user runs tmux. Without an explicit preference in config, Ito has no way to know whether to suggest tmux, and suppressing or showing those options requires per-workflow guesswork. A `tools.tmux.enabled` config key, set once during `ito init`, gives Ito a single canonical gate to consult. Skills that use tmux install unconditionally (avoiding install-state complexity), but they read this flag at invocation time and self-govern accordingly.

## What Changes

- Add `tools.tmux.enabled` (bool, default `true`) to the Ito configuration schema.
- Add a "Do you use tmux?" yes/no prompt to `ito init` interactive flow; write the result to project config regardless of answer.
- `--no-tmux` flag for non-interactive `ito init` to suppress the prompt and write `false`.
- The `ito-tmux-skill` (installed unconditionally) reads `tools.tmux.enabled` and omits tmux-specific guidance when the flag is false.
- The `proposal-viewer` (001-29) hides the `tmux-nvim` viewer option when `tools.tmux.enabled = false`; `--viewer tmux-nvim` is rejected with a clear message.
- All other Ito-generated workflow instructions (ralph loop, apply, etc.) MUST consult `tools.tmux.enabled` before surfacing any tmux suggestions — `tools.tmux.enabled` is the canonical workflow-wide gate.
- Update `config-schema` (`schemas/ito-config.schema.json`) to include the new key.

## Capabilities

### New Capabilities

- `tools-config`: A `tools` namespace in the Ito config schema for per-tool preferences. Initially contains `tools.tmux.enabled`. Designed to be extended for other tools (e.g., `tools.bat.enabled`, `tools.glow.enabled`) without structural changes.

### Modified Capabilities

- `cli-init`: `ito init` interactive flow gains a tmux preference prompt; `--no-tmux` flag for non-interactive use.
- `global-config`: Documents `tools.tmux.enabled` as a supported config key alongside existing `worktrees.*` keys.
- `config-schema`: JSON schema artifact updated to include `tools.tmux.enabled`.

## Impact

- `ito-rs/crates/ito-config/src/` — new `tools` struct with `tmux.enabled` field; serde defaults
- `ito-rs/crates/ito-cli/src/commands/init.rs` — tmux prompt + `--no-tmux` flag
- `schemas/ito-config.schema.json` — schema regenerated to include `tools`
- `ito-rs/crates/ito-templates/assets/skills/tmux/SKILL.md` — self-governance note referencing `tools.tmux.enabled`
- Downstream: `proposal-viewer` (001-29) and any workflow instruction generators that surface tmux suggestions
<!-- ITO:END -->
