## Why

There is no `/ito-list` slash command for AI assistants to list project changes, specs, or modules. Users have to manually instruct the agent to run `ito list` with the right flags. Adding a dedicated command and skill enables a discoverable, self-documenting workflow where the agent can also interpret results and suggest next actions.

## What Changes

- Adds `ito-list` skill to embedded assets (`assets/skills/ito-list/SKILL.md`)
- Adds `ito-list` command template to embedded assets (`assets/commands/ito-list.md`)
- Both are auto-discovered by `include_dir!` — no code changes to distribution.rs
- After `ito init`/`ito update`, the command and skill are installed to all harnesses (Claude, OpenCode, Codex, GitHub Copilot)
- The skill instructs the agent to run `ito list` with appropriate flags, interpret results, and suggest next actions (e.g., ready changes → suggest `/ito-apply`, completed → suggest `/ito-archive`)

## Capabilities

### New Capabilities

- None (the command delegates to the existing `ito list` CLI; no new spec-level capabilities)

### Modified Capabilities

- None (no spec-level behavior changes)

## Impact

- Two new embedded asset files added to `ito-templates`
- Installed to all four harnesses via existing distribution pipeline
- No Rust source code changes required
