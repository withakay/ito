## Context

Claude Code supports loading skills from `.claude/skills/`. The goal is to keep Claude-side assets extremely small and delegate all canonical workflow content to `ito agent instruction` artifacts.

## Goals / Non-Goals

- Goals:
  - Provide a small Claude Code entrypoint that points to `ito agent instruction bootstrap --tool claude`.
  - Avoid hooks unless a fallback is truly needed.
- Non-Goals:
  - Duplicating long workflow docs in `.claude/skills/`.

## Implementation Status

- ✅ `.claude/skills/ito-workflow/SKILL.md` created in default project template
- ✅ Minimal `session-start.sh` shim created in `ito-skills/adapters/claude/`
- ✅ Deprecation documentation added for `ito-skills/hooks/`

## Assumptions

- The `ito agent instruction bootstrap --tool claude` command is implemented in change 013-04. Until then, the skill references the command string as a literal.
- Distribution/fetch mechanics for instruction artifacts will be implemented in 013-05.
- The SessionStart hook shim is minimal and only points to the bootstrap command - full workflow content is delegated to the CLI.

## Deprecation Path for `ito-skills/hooks/`

The old `ito-skills/hooks/` implementation:
- Embedded full workflow content in SessionStart hook
- Is being replaced by template-based approach and minimal shim
- Migration path documented in `ito-skills/adapters/claude/README.md`

Users should:
1. Prefer project templates (`.claude/skills/`) when working in a project
2. Use the minimal shim (`adapters/claude/session-start.sh`) for non-project contexts
3. Stop using the old `ito-skills/hooks/` directory

## Removal Timeline

- Old hooks remain functional for backward compatibility
- Users should migrate to the new approach
- Future version will remove `ito-skills/hooks/` entirely

## Contracts

### CLI Contract

Claude integration assumes:

`ito agent instruction bootstrap --tool claude`

returns a tool-specific preamble that includes how to fetch the rest of the workflows.

### Install Contract

Installer will embed/copy:

- `.claude/skills/ito-workflow/SKILL.md` (project template) ✅ Implemented
- Optional shim under `ito-skills/adapters/claude/` (if required) ✅ Implemented

## Decisions

- Prefer templates (`.claude/skills/`) over hooks.
- Any hook/shim should only print a pointer to the bootstrap artifact.

## Rust Style

If this change requires Rust updates (e.g., template embedding or installer plumbing), follow the `rust-style` skill.
