---
name: ito
description: Unified entry point for ito commands with intelligent skill-first routing and CLI fallback.
---

Route ito commands to the best handler.

## Goal

Users may type requests like `ito archive 001-03_add-ito-skill` or `ito dashboard`.

This skill MUST:

1. Prefer matching ito-* skills (skill-first precedence)
2. Fall back to the ito CLI when no matching skill is installed
3. Preserve argument order and content

## Input

The requested command is provided either:

- As plain text following the word "ito" in the user request
- In prompt arguments (if your harness provides them)
- In a `<ItoCommand>` block

## Steps

1. **Parse** the command:
   - Extract the primary command (first token) and the remaining args
   - If no command is provided, output a concise error: "Command is required" and show a one-line usage example

2. **Resolve skill target**:
   - Build candidate skill id: `ito-${command}`
   - Determine if that skill is installed/available in this harness
     - OpenCode: check for a directory under `.opencode/skills/`
     - Claude: check for a directory under `.claude/skills/`
     - GitHub Copilot: check for a directory under `.github/skills/`
     - Codex: skills are global; if unsure, assume not installed and use CLI fallback

3. **Execute**:
   - If matching skill exists: follow that skill's instructions, passing along the original args
   - Otherwise: invoke the CLI using Bash: `ito <command> <args...>`

4. **Error handling**:
   - If the invoked skill fails: prefix with `[ito-* skill error]` and preserve the original error
   - If the CLI fails: prefix with `[ito CLI error]` and preserve the original error
