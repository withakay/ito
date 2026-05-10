<!-- ITO:START -->
## Why

`ito init` and `ito update` have fallen behind the current project configuration surface, so new settings can be missed during setup or refresh. The init wizard also behaves surprisingly when rerun because it does not consistently preselect values from the existing config.

## What Changes

- Audit the current Ito config model and identify settings that should be handled by `ito init`, `ito init --upgrade`, and/or `ito update` flags.
- Add missing setup/update handling for relevant config settings so the CLI can keep generated project configuration current.
- Make interactive `ito init` read existing project config before prompting and use configured values as the default selections in the TUI.
- Update generated Ito instructions so any worktree-enabled repo tells agents to create/use a dedicated change worktree before any write operation, rather than doing proposal, code, docs, generated-asset, or commit work from the main/control checkout.
- Preserve existing explicit config values unless the user changes them in the wizard or passes an overriding non-interactive flag.
- Add tests that cover rerunning init against an existing config with tmux, worktrees, and bare sibling strategy already selected.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cli-init`: `ito init` must use existing config values as wizard defaults and cover missing setup settings.
- `cli-update`: `ito update` must expose/refresh the same relevant config settings without surprising overwrites.
- `config-schema`: config coverage must be checked against the init/update setup surface so missing settings are visible and testable.
- `worktree-aware-template-rendering`: generated instructions for worktree-enabled repos must make change worktrees mandatory for file-changing work.

## Impact

- Affected code: `ito-rs/crates/ito-cli` init/update command parsing and TUI prompt defaults; `ito-rs/crates/ito-core` project initialization/update orchestration; `ito-rs/crates/ito-config` config loading/default resolution as needed.
- Affected templates: generated `AGENTS.md` and worktree skill instructions rendered by Ito init/update.
- Affected tests: CLI integration tests for interactive init defaults, non-interactive flags, and config-schema/init-update parity.
- No breaking changes are intended; existing configs should be preserved and become more reliably honored.
<!-- ITO:END -->
