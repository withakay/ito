<!-- ITO:START -->
## Context

Ito already has a config-driven worktree model: `worktrees.strategy`, `worktrees.layout.dir_name`, `worktrees.default_branch`, and `ito path worktrees-root` define where change worktrees live. The current implementation and generated guidance still use raw `git worktree` commands, while Worktrunk (`wt`) provides the higher-level commands Ito wants users and agents to rely on.

The critical compatibility constraint is path stability. Existing Ito projects, including this one, expect change worktrees under the configured `ito-worktrees` root. Worktrunk has its own configurable `worktree-path` template, so Ito must not blindly inherit a user's global Worktrunk path.

## Goals / Non-Goals

**Goals:**

- Use Worktrunk for Ito-managed change worktree creation, switching, listing, and user-facing guidance.
- Preserve Ito's configured `ito-worktrees/<change-id>` path layout by running Worktrunk with a local/project-specific path template.
- Keep `ito worktree ensure --change <id>` script-friendly: stdout remains only the resolved absolute worktree path.
- Preserve Ito initialization after creation: include files, setup commands, and initialization markers still run under Ito control.
- Produce actionable diagnostics when `wt` is missing or fails.

**Non-Goals:**

- Replace git usage unrelated to change worktree lifecycle, such as normal commits or archive behavior.
- Require every user to change their global `~/.config/worktrunk/config.toml`.
- Change Ito change IDs, branch naming, or the default `ito-worktrees` directory convention.
- Adopt Worktrunk hooks as the only setup mechanism in this change.

## Decisions

- Decision: Use Worktrunk as the creation/switching interface for change worktrees.
  Alternatives considered: keep raw `git worktree add` and only update docs; this would leave two lifecycle models and would not satisfy the replacement goal.

- Decision: Preserve Ito's path model by deriving the Worktrunk path template from Ito's resolved worktree root.
  Alternatives considered: adopt Worktrunk's global default path; this would churn existing `ito-worktrees` paths and make scripts/docs inconsistent across developers.

- Decision: Do not require editing user global Worktrunk config for Ito operations.
  Alternatives considered: tell users to set `worktree-path` globally; this is fragile because one user's global preference can break Ito's configured project layout.

- Decision: Keep Ito worktree initialization after Worktrunk creates the working tree.
  Alternatives considered: move setup entirely into `.config/wt.toml` hooks; this would blur Ito's existing setup semantics and make `ito worktree setup` harder to reason about.

- Decision: Prefer Worktrunk structured listing for detection, with a git porcelain fallback for existing or partially migrated repositories.
  Alternatives considered: remove git porcelain fallback immediately; this would be a sharper migration and could break existing worktrees before Worktrunk config is available.

## Risks / Trade-offs

- Worktrunk CLI missing -> Return a targeted error with install/PATH guidance before any partial initialization occurs.
- Local Worktrunk config behavior differs from user expectations -> Document the precedence and ensure Ito-managed operations are deterministic even when global config differs.
- Worktrunk path template cannot express a strategy-specific layout exactly -> Use an operation-local config or environment override generated from Ito's resolved absolute worktree root.
- Existing tests assert raw `git worktree` snippets -> Update tests to assert Worktrunk commands and path-preservation guidance.

## Migration Plan

1. Add tests that fail because `ito worktree ensure` still invokes raw `git worktree add` and templates still render raw git commands.
2. Add a small Worktrunk invocation boundary around process execution so tests can assert the exact `wt` commands and config/env used.
3. Update worktree ensure creation to call Worktrunk, then run existing Ito initialization unchanged.
4. Update rendered instructions and template tests to use Worktrunk and local path configuration guidance.
5. Update Ralph worktree detection to prefer Worktrunk structured listing while retaining git porcelain fallback.

Rollback is straightforward: revert the Worktrunk invocation boundary and restore the previous raw git creation path. Existing worktree directories and branches remain normal git worktrees.

## Open Questions

- Should Ito commit a default `.config/wt.toml` during `ito init`, or should it generate an operation-local Worktrunk config only for Ito-managed commands?
- Should Worktrunk become a hard dependency whenever `worktrees.enabled=true`, or should only creation require it while detection keeps broader fallback behavior?
<!-- ITO:END -->
