# Ito Templates Notes

This crate owns the files installed by the Rust CLI via `ito init` / `ito update`.

## Where to edit

- **Shared skills** (installed to all harnesses): `assets/skills/`
  - General development skills (e.g., `brainstorming/`, `systematic-debugging/`)
  - Ito workflow skills (e.g., `ito/`, `ito-apply-change-proposal/`)
  - Skills here are copied to each harness's skills directory at install time

- **Shared adapters**: `assets/adapters/`
  - Harness-specific bootstrap files (e.g., `claude/session-start.sh`, `opencode/ito-skills.js`)

- **Project templates**: `assets/default/project/`
  - Harness-specific commands/prompts: `.opencode/commands/`, `.claude/commands/`, `.github/prompts/`, `.codex/prompts/`
  - Ito project docs: `.ito/`

- **Home templates**: `assets/default/home/` (currently unused by default)

Do NOT edit the checked-in, repo-root `.opencode/`, `.claude/`, `.github/`, or `.ito/` files directly when changing what `ito init` installs; those are outputs.

## Keeping harness files in sync

**IMPORTANT**: Commands and prompts under each harness directory (`.claude/`, `.opencode/`, `.codex/`, `.github/`) must be kept functionally equivalent.

Each harness has its own frontmatter format:
- **Claude Code** (`.claude/`): YAML frontmatter with `name`, `description`, `category`, `tags`
- **OpenCode** (`.opencode/`): YAML frontmatter with `name`, `description`
- **GitHub Copilot** (`.github/`): YAML frontmatter (check GitHub docs for current format)
- **Codex** (`.codex/`): YAML frontmatter with `name`, `description`

When adding or modifying a command/prompt:
1. Update ALL harness versions to maintain feature parity
2. Use the correct frontmatter format for each harness
3. Keep the core instructions identical (only frontmatter differs)

Skills are shared from `assets/skills/` and don't need per-harness maintenance.

## How to verify changes

```bash
make install
ito init --force --tools all
```

Then inspect installed files (examples):

- `.opencode/skills/*/SKILL.md`
- `.claude/skills/*/SKILL.md`
- `.github/skills/*/SKILL.md`
- `.ito/AGENTS.md`

## Future harnesses
There may have been more harnesses added after this was written; follow the same pattern for those and self update this document if you come across them.
