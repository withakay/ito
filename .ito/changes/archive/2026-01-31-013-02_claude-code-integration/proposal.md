# Change: Claude Code Integration for Ito Skills

## Why

The vendored `ito-skills` uses Claude Code's `SessionStart` hook to inject bootstrap context. However, Ito's preferred approach is to use project templates (`AGENTS.md`/`CLAUDE.md`) and `.claude/skills/*` that delegate to `ito agent instruction`. This minimizes hook complexity and keeps workflow content in a single source of truth.

## What Changes

- Document that Claude Code integration should prefer project templates over hooks
- Create minimal `.claude/skills/ito-workflow.md` skill file that:
  - Points to `ito agent instruction <artifact>` for workflow bodies
  - Avoids embedding long policy text
- Optional: Create a minimal `SessionStart` hook shim that only prints a pointer to an instruction artifact (for cases where project files are not loaded)
- Remove or deprecate the existing `ito-skills/hooks/` bash scripts in favor of the template-based approach

## Capabilities

### New Capabilities

- `claude-code-adapter`: Claude Code skill/hook integration for Ito workflows

### Modified Capabilities

None

## Impact

- Affected specs: `tool-adapters` (new)
- Affected code:
  - New: `.claude/skills/ito-workflow.md` (template)
  - Optional: `ito-skills/adapters/claude/session-start.sh` (minimal shim)
  - Deprecate: `ito-skills/hooks/`
- Embedded in: `ito-rs/crates/ito-templates/assets/`
- Parallelization: Can be developed in parallel with 013-01, 013-03

## Parallel Execution Notes

This change can be implemented in parallel with:
- 013-01 (OpenCode adapter) - no shared code paths
- 013-03 (Codex bootstrap) - no shared code paths

Soft dependency on:
- 013-04 (bootstrap artifact CLI) - for the `ito agent instruction bootstrap --tool claude` content
- 013-05 (distribution) - for install/fetch mechanics
