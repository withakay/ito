## Why

`ito init` currently re-prompts for interactive setup choices even when a project already has configuration, which is repetitive and slows down re-initialization and upgrades. We want init to respect existing config so users can keep current settings by simply pressing Enter/Next.

## What Changes

- Make the interactive `ito init` worktree wizard pre-fill defaults from already-configured values (for example `worktrees.strategy`).
- Ensure users can accept existing settings without re-selecting options (Enter/Next keeps the current value).
- Keep non-interactive init unchanged.
- Keep existing filesystem/config behavior as the default; no behavior changes when the wizard is not invoked.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `ito-init`: Interactive init wizard defaults are derived from resolved config so previously chosen settings do not require re-selection.

## Impact

- **Affected code**: `ito-rs/crates/ito-cli/src/app/init.rs` and init wizard test coverage.
- **User experience**: Re-running `ito init` becomes "next-next-next" when config is already set.
- **Compatibility**: Existing config keys and precedence rules are preserved.
