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
- **New**: `worktrees.init.setup` config field — an optional command (string) or command list
  executed inside the new worktree after files are copied. Examples: `"make init"`,
  `"npm install"`, or a script that was brought over via the include list.
- **New**: `ito worktree setup --change <id>` CLI sub-command — runs the configured setup
  command(s) in the target worktree; called automatically by `ito worktree ensure` after
  initialization. Can also be called standalone to re-run setup without recreating the worktree.
- **New**: `ito agent instruction worktree-init --change <id>` instruction artifact — emits
  the setup steps as human/agent-readable text for harnesses where the CLI cannot directly
  execute the setup (e.g., agent needs to run `npm install` itself). When a setup command is
  configured, the output lists the commands to run and the working directory. When no command
  is configured, the output is a no-op placeholder.
- **New**: Agent instruction guidance injected into `apply` instructions when worktrees are
  enabled: the agent SHALL run `ito worktree ensure --change <id>` (which now covers ensure +
  file copy + setup) and then work from the returned worktree path.
- **Modified**: `WorktreesConfig` schema — adds `init: WorktreeInitConfig` sub-section.

## Capabilities

### New Capabilities

- `worktree-lifecycle`: Ensure the correct worktree for a change exists and is initialized
  before apply work begins. Covers existence check, creation, coordination-branch symlink
  setup, file copy-over, setup command execution, and reporting the resolved worktree path.
- `worktree-init-files`: Configurable file copy-over when a new change worktree is created.
  Supports globs defined in `worktrees.init.include` (config) and/or `.worktree-include`
  (file), with union semantics when both are present.
- `worktree-setup`: Configurable post-init command execution inside a new worktree. Supports
  a single command string or ordered list of commands via `worktrees.init.setup` in config.
  Exposed as `ito worktree setup --change <id>` (standalone re-run) and as an instruction
  artifact via `ito agent instruction worktree-init --change <id>`.

### Modified Capabilities

- `config`: `WorktreesConfig` gains a new `init: WorktreeInitConfig` sub-section containing
  `include: Vec<String>` (glob patterns) and `setup: WorktreeSetupConfig` (optional command
  or command list). Existing fields and defaults are unchanged.

## Impact

- `ito-config`: new `WorktreeInitConfig` and `WorktreeSetupConfig` types; `WorktreesConfig`
  gains `init` field.
- `ito-core`: `worktree_ensure` now also runs setup command after file copy; new
  `run_worktree_setup` operation exposed standalone.
- `ito-cli`: new `ito worktree ensure --change <id>` and `ito worktree setup --change <id>`
  sub-commands; new `ito agent instruction worktree-init --change <id>` instruction artifact.
- `ito-templates`: apply instruction updated; new `worktree-init` instruction template added.
- JSON config schema updated. No breaking changes to existing config keys or CLI commands.
<!-- ITO:END -->
