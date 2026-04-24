<!-- ITO:START -->
## Why

When worktrees are enabled in the Ito config, applying a change requires the agent to be
working in the correct worktree, but today there is no guardrail that ensures that
worktree exists, is correctly initialized, or is the active working directory. Agents
also lack an Ito-native mechanism for copying non-committed files (`.env`, `.envrc`, etc.)
into a freshly-created worktree, which causes builds and tests to silently fail.

## What Changes

- **New**: `ito worktree check --change <id>` command (or sub-command of `ito worktree`)
  that verifies the correct worktree exists for a change; creates and initializes it if absent.
- **New**: Worktree initialization step that: (a) creates the coordination-branch symlinks
  and (b) copies non-committed include files into the new worktree.
- **New**: `worktrees.init.include` config field — list of globs specifying files/directories
  to copy into a new worktree (e.g. `.env`, `.envrc`, `*.local.toml`).
- **New**: `.worktree-include` file at the repo root — file-based alternative/complement to
  the config field, one glob per line (analogous to `.gitignore`; follows `.gitignore` pattern
  syntax). When both the config field and the file are present, the union of both is used.
- **New**: Agent instruction guidance injected into `apply` instructions when worktrees are
  enabled: the agent SHALL run `ito worktree ensure --change <id>` and then work from the
  returned worktree path rather than the main checkout.
- **Modified**: `WorktreesConfig` schema — adds `init: WorktreeInitConfig` sub-section.

## Capabilities

### New Capabilities

- `worktree-lifecycle`: Ensure the correct worktree for a change exists and is initialized
  before apply work begins. Covers existence check, creation, coordination-branch symlink
  setup, and reporting the resolved worktree path so agents can orient themselves.
- `worktree-init-files`: Configurable file copy-over when a new change worktree is created.
  Supports globs defined in `worktrees.init.include` (config) and/or `.worktree-include`
  (file), with union semantics when both are present. Prior art: no dominant standard found;
  the `.worktree-include` name mirrors `.gitignore` ergonomics and is distinct from existing
  tooling to avoid conflicts.

### Modified Capabilities

- `config`: `WorktreesConfig` gains a new `init: WorktreeInitConfig` sub-section containing
  `include: Vec<String>` (glob patterns). Existing fields and defaults are unchanged.

## Impact

- `ito-config`: new `WorktreeInitConfig` type; `WorktreesConfig` gains `init` field.
- `ito-core`: new `worktree_ensure` operation (check, create, init symlinks, copy files); used
  by the apply instruction path and exposed as a standalone CLI command.
- `ito-cli`: new `ito worktree ensure --change <id>` sub-command; prints the resolved worktree
  path on stdout so scripts/agents can capture it.
- `ito-templates`: agent instruction template updated to emit the `ito worktree ensure` step
  when `worktrees.enabled` is true.
- JSON config schema updated to include the new `init` section.
- No breaking changes to existing config keys or CLI commands.
<!-- ITO:END -->
