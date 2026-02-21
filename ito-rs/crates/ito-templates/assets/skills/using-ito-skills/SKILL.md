---
name: using-ito-skills
description: "Use when discovering, finding, invoking, or loading skills. Ensures skills are invoked BEFORE responding."
---

# Using Ito Skills

If a skill applies to your task, you must invoke it before responding. Even a 1% chance means check.

## How to Access Skills

| Harness | Load Command | Skill Locations |
|---------|-------------|-----------------|
| OpenCode | `skill load <name>` | `.opencode/skills/`, `~/.config/opencode/skills/` |
| Claude Code | `mcp_skill` with `name="<name>"` | `.claude/skills/` |
| Codex | Read directly: `cat .codex/skills/<name>/SKILL.md` | `.codex/skills/`, `~/.codex/skills/` |

**Detecting your harness:** OpenCode has the `skill` tool, Claude Code has `mcp_skill`, Codex has `.codex/` directory.

## Red Flags (you're rationalizing)

- "This is just a simple question" — questions are tasks, check for skills
- "I need more context first" — skill check comes BEFORE exploration
- "The skill is overkill" — simple things become complex, use it
- "I remember this skill" — skills evolve, read the current version

## Priority

When multiple skills apply:
1. **Process skills first** (brainstorming, debugging) — determine HOW to approach
2. **Implementation skills second** — guide execution

## Skill Types

**Rigid** (TDD, debugging): Follow exactly. Don't adapt away discipline.
**Flexible** (patterns): Adapt principles to context. The skill itself tells you which.
