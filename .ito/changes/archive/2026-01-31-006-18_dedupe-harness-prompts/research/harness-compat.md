# Harness Compatibility (Agent Skills First)

## Baseline: Agent Skills (agentskills.io)

Source of truth: `https://agentskills.io/specification`

- A skill is a directory containing `SKILL.md` (YAML frontmatter + Markdown body).
- Required frontmatter fields: `name`, `description`.
- Optional frontmatter fields: `license`, `compatibility`, `metadata`, `allowed-tools` (experimental).
- Progressive disclosure: load metadata at startup; load full body only when invoked; load supporting resources on demand.

## Claude Code

Source: `https://code.claude.com/docs/en/skills.md`

- Explicitly states Claude Code skills follow Agent Skills and that “Custom slash commands have been merged into skills.”
- Supports skills at project (`.claude/skills/<name>/SKILL.md`), personal (`~/.claude/skills/...`), enterprise-managed, and plugin scopes.
- Adds non-standard frontmatter extensions (e.g., invocation control fields and tool allowlisting) and dynamic context injection.

Compatibility note: Safe to generate baseline Agent Skills; add Claude-specific fields only when we need Claude-only behavior.

## OpenCode

Source: `https://opencode.ai/docs/skills/` and `https://opencode.ai/docs/commands/`

- Supports Agent Skills; recognizes only a subset of frontmatter keys and ignores unknown keys.
- Supports custom commands as Markdown templates.

Compatibility note: Keep `SKILL.md` frontmatter minimal (Agent Skills baseline fields) so nothing important is ignored.

## OpenAI Codex

Source: `https://developers.openai.com/codex/skills`

- Supports Agent Skills (explicitly references agentskills.io).
- Skill discovery locations include `.codex/skills` (repo), `~/.codex/skills` (user), and `/etc/codex/skills` (admin).
- Built-in slash commands are session controls; custom behaviors should be implemented as skills.

Compatibility note: Codex documentation lists different max lengths for `name`/`description` than agentskills.io; keep within both.

## GitHub Copilot

Sources:

- Agent Skills: `https://docs.github.com/en/copilot/concepts/agents/about-agent-skills`

- Prompt files: `https://docs.github.com/en/copilot/tutorials/customization-library/prompt-files/your-first-prompt-file`

- Supports Agent Skills in `.github/skills` (and `.claude/skills`).

- Also supports “prompt files” under `.github/prompts/*.prompt.md`, which are *not* Agent Skills and are an IDE-centric slash-command mechanism.

Compatibility note: For Copilot, keep using Agent Skills for portable behavior; prompt files (if shipped) should be thin shims that delegate to the skill/CLI.
