# 029-01_add-byterover-integration

Add ByteRover as the first concrete agent-memory provider: install the Claude Code connector, install ByteRover hub skills (review, plan, explore, audit) under .agents/skills/byterover/, and curate Ito's authoritative docs/specs/modules into the local ByteRover context tree. Local-only (no cloud sync). Shared skills live under .agents/skills/ except for Claude Code which uses its own connector.

## Reviewer notes

- Installs the Claude Code ByteRover connector under `.claude/`.
- Installs the `byterover-review`, `byterover-plan`, `byterover-explore`, and `byterover-audit` hub skills under `.agents/skills/byterover/`.
- Bootstraps local ByteRover knowledge with folder-pack curation for `.ito/specs/`, `docs/`, `.ito/changes/archive/`, and `.ito/modules/`, plus file-mode curation for `AGENTS.md`, `.ito/AGENTS.md`, and `.ito/architecture.md`.
- Detailed reproduction commands belong in `.agents/skills/byterover/README.md`. Cloud sync (`brv login`, `brv push`, `brv pull`, `brv vc …`) remains out of scope for this change.

