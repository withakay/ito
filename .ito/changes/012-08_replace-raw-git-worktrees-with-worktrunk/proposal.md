<!-- ITO:START -->
## Why

Ito currently teaches and executes raw `git worktree` commands even though Worktrunk provides the higher-level worktree lifecycle that this project wants agents and developers to use. Replacing the raw git surface with Worktrunk reduces duplicated lifecycle logic while keeping Ito's existing `ito-worktrees` path convention stable.

## What Changes

- Replace direct change-worktree creation through `git worktree add` with Worktrunk `wt switch --create` integration.
- Preserve Ito's configured worktree location by ensuring Worktrunk runs with a local/project-specific worktree path configuration that maps change branches to the existing `ito-worktrees/<change-id>` layout.
- Update generated agent/worktree instructions to use Worktrunk commands instead of raw `git worktree` shell snippets.
- Update worktree discovery and lifecycle guidance so Ito treats Worktrunk as the canonical worktree management interface while still preserving Ito's path reporting and initialization semantics.
- Keep existing branch naming, setup, and `ito worktree ensure` stdout contracts intact for scripts and agents.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `worktree-lifecycle`: worktree creation switches from raw `git worktree add` to Worktrunk, including local Worktrunk configuration for Ito's path layout.
- `worktree-aware-template-rendering`: rendered instructions switch from raw git worktree commands to Worktrunk commands and configuration guidance.
- `ralph-worktree-awareness`: worktree detection prefers Worktrunk-managed structured listing rather than raw git porcelain parsing.

## Impact

- Affected code: `ito-rs/crates/ito-core/src/worktree_ensure.rs`, worktree ensure tests, Ralph worktree resolution, and process execution around worktree lifecycle.
- Affected templates: `ito-rs/crates/ito-templates/assets/instructions/agent/worktrees.md.j2` and related template rendering tests.
- Affected config: Ito must be able to create or invoke Worktrunk with a local configuration that preserves `ito-worktrees` as the default worktree root for this project.
- External dependency: Worktrunk CLI (`wt`) becomes the expected worktree lifecycle command when worktrees are enabled.
<!-- ITO:END -->
