# Change: Config-driven worktree workflow guidance

## Why

Different developers can reasonably prefer different worktree workflows (e.g., `checkout_subdir` vs `bare_control_siblings`). Baking a single strategy into committed templates (like `AGENTS.md` and worktree-related skills) creates churn, causes incorrect instructions on other machines, and makes it harder to keep guidance accurate.

## What Changes

- Add `ito agent instruction worktrees` (and `ito agent instruction workflow` as an alias) to print resolved worktree configuration and exact, strategy-specific commands.
- Extend cascading project configuration to support per-developer overrides via `.ito/config.local.json` and `.local/ito/config.json`.
- Persist interactive worktree wizard choices to the per-developer overlay by default, while continuing to read global config for backward compatibility.
- Update templates/skills to delegate worktree guidance to `ito agent instruction worktrees` instead of embedding a single strategy in committed files.

## Impact

- **Specs**: `agent-instructions`, `cascading-config`, `cli-init`, `cli-update`
- **Code**: `ito-config`, `ito-cli`, `ito-core`, template assets, installer gitignore behavior, tests/snapshots
- **Compatibility**: Existing global worktree config remains readable; guidance and persistence prefer per-project local overlays to avoid repo churn.
