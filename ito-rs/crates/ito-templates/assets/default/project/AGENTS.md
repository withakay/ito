<!-- ITO:START -->

# Ito Instructions

These instructions are for AI assistants working in this project.

Always open `@/.ito/AGENTS.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/.ito/AGENTS.md` to learn:

- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Note: Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.
Add project-specific guidance in `.ito/user-guidance.md` (injected into agent instruction outputs) and/or below this managed block.

Keep this managed block so 'ito update' can refresh the instructions.

## Worktrees

Worktree workflow is **config-driven** so different developers can use different strategies without changing committed files.

Source of truth (prints the exact strategy + commands for *this machine*):

```bash
ito agent instruction worktrees
```

Per-developer overrides should go in `.ito/config.local.json` (gitignored).

Skill hint (if your harness supports skills): `ito-workflow`, `using-git-worktrees`.

<!-- ITO:END -->

## Project Guidance

(Add any project-specific assistant guidance here. Prefer `.ito/user-guidance.md` for instructions you want applied consistently to Ito change workflows.)
