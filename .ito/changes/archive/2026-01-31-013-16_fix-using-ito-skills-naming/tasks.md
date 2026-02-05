# Tasks for: 013-16_fix-using-ito-skills-naming

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

```bash
ito tasks status 013-16_fix-using-ito-skills-naming
ito tasks next 013-16_fix-using-ito-skills-naming
```

______________________________________________________________________

## Wave 1: Fix frontmatter

### Task 1.1: Update frontmatter name and description

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Change `name: using-superpowers` to `name: using-ito-skills`
  - Update description to: "Use when discovering, finding, invoking, or loading skills. Ensures skills are invoked BEFORE responding. Establishes skill priority and usage patterns for OpenCode, Claude Code, and Codex."
- **Done When**: Frontmatter updated
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Add multi-harness support

### Task 2.1: Add OpenCode skill instructions

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Add section: "## Using Skills in OpenCode"
  - Document: `skill list` to discover skills, `skill load <name>` to invoke
  - Note skill locations: `.opencode/skills/` (project), `~/.config/opencode/skills/` (user)
- **Done When**: OpenCode instructions added
- **Status**: [x] completed

### Task 2.2: Add Claude Code skill instructions

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Add section: "## Using Skills in Claude Code"
  - Document: `mcp_skill` function with `name` parameter
  - Note skill locations: `.claude/skills/` (project)
- **Done When**: Claude Code instructions added
- **Status**: [x] completed

### Task 2.3: Add Codex skill instructions

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Add section: "## Using Skills in Codex"
  - Document: read skill files from `.codex/skills/ito-<name>/SKILL.md`
  - Note how Codex discovers and uses skill content
- **Done When**: Codex instructions added
- **Status**: [x] completed

### Task 2.4: Add harness detection guidance

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Add section: "## Detecting Your Harness"
  - Document hints: available tools, environment markers, directory structure
- **Done When**: Detection guidance added
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Clean content

### Task 3.1: Remove superpowers references

- **Files**: `ito-skills/skills/using-ito-skills/SKILL.md`
- **Action**:
  - Search for any `superpowers` references
  - Replace with `ito-skills` or remove as appropriate
- **Verify**: `grep -i superpowers ito-skills/skills/using-ito-skills/SKILL.md` returns no results
- **Done When**: No superpowers references
- **Status**: [x] completed

______________________________________________________________________

## Wave 4: Update embedded template

### Task 4.1: Sync embedded template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-using-ito-skills/SKILL.md`
- **Action**:
  - Copy updated skill from `ito-skills/skills/using-ito-skills/SKILL.md`
- **Done When**: Embedded template updated
- **Status**: [x] completed

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started
- `[ ] in-progress` - Currently working
- `[x] complete` - Finished and verified
- `[-] shelved` - Deferred
