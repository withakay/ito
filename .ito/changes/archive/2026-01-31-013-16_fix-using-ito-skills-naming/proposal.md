## Why

The `using-ito-skills` skill has multiple issues:
1. **Naming mismatch**: Directory is `using-ito-skills/` but frontmatter says `name: using-superpowers`
2. **Single-harness focus**: Only references Claude Code's `Skill` tool, but should support OpenCode, Claude Code, and Codex
3. **Outdated references**: Contains `superpowers` references

## What Changes

- Update frontmatter `name` field from `using-superpowers` to `using-ito-skills`
- Update description to be keyword-rich for discoverability
- Add harness-specific instructions for:
  - **OpenCode**: Use native `skill` tool to list/load skills
  - **Claude Code**: Use `Skill` tool with `mcp_skill` function
  - **Codex**: Reference skill files in `.codex/skills/`
- Remove all `superpowers` references

## Capabilities

### Modified Capabilities

- `using-ito-skills`: Fixed naming, added multi-harness support (OpenCode, Claude Code, Codex)

## Impact

- **ito-skills/skills/using-ito-skills/SKILL.md**: Major update for multi-harness
- **Embedded templates**: Update `ito-using-ito-skills`
- Skill becomes useful across all supported AI coding assistants
