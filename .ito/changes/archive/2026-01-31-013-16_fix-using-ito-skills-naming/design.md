## Context

The `using-ito-skills` skill was originally called `using-superpowers` and only referenced Claude Code's `Skill` tool. The project now supports three AI coding assistant harnesses:
- **OpenCode**: Has native `skill` tool for discovery and loading
- **Claude Code**: Uses `mcp_skill` MCP function
- **Codex**: Reads skill files directly from `.codex/skills/`

The skill needs to work across all three.

## Goals / Non-Goals

**Goals:**
- Fix frontmatter name to match directory
- Add harness-specific skill invocation instructions
- Update description for discoverability
- Remove superpowers references

**Non-Goals:**
- Changing the core message (invoke skills BEFORE responding)
- Making the skill harness-specific (one skill works for all)

## Decisions

### 1. Single skill with multi-harness instructions

**Decision**: Keep one skill file with sections for each harness.

**Rationale**: The core guidance (invoke skills first) is universal. Only the mechanics differ.

### 2. Harness-specific sections

**Decision**: Add clearly labeled sections for OpenCode, Claude Code, and Codex.

**Rationale**: Each harness has different skill invocation mechanisms. Clear sections prevent confusion.

### 3. Detection guidance

**Decision**: Include hints for detecting which harness is running.

**Rationale**: The skill content itself may be used across harnesses; knowing which one helps apply the right instructions.

## Risks / Trade-offs

**[Trade-off] Longer skill** → More content to cover all harnesses. Acceptable for universal applicability.

## Migration Plan

1. Update frontmatter name and description
2. Add OpenCode skill instructions section
3. Add Claude Code skill instructions section
4. Add Codex skill instructions section
5. Add harness detection guidance
6. Remove superpowers references
7. Update embedded template
