## Context

Released Ito should fetch adapter files over HTTP; development should use the local `./ito-skills/` tree without symlinks. Both modes should share the same manifests and install destinations.

## Goals / Non-Goals

- Goals:
  - Fetch raw files via GitHub URLs with a version tag.
  - Cache downloads per-user.
  - Support local-dev copy fallback.
  - Install tool-specific files into their expected config locations.
- Non-Goals:
  - Packaging adapters inside the binary beyond the existing templates mechanism.

## Rust Style

All Rust implementation for this change follows the `rust-style` skill.

## Decisions

- URL scheme:
  - Primary: `https://raw.githubusercontent.com/withakay/ito/<tag>/ito-skills/<path>`
  - Fallback: `https://raw.githubusercontent.com/withakay/ito/main/ito-skills/<path>`
- Cache directory:
  - `~/.config/ito/cache/ito-skills/<tag>/<path>`

## File Manifests

### OpenCode

- Source: `ito-skills/adapters/opencode/ito-skills.js`
- Dest: `${OPENCODE_CONFIG_DIR}/plugins/ito-skills.js`
- Source: `ito-skills/skills/`
- Dest: `${OPENCODE_CONFIG_DIR}/skills/ito-skills/`

### Claude Code

- Source: `.claude/skills/ito-workflow.md`
- Dest: `<project>/.claude/skills/ito-workflow.md`
- Optional source: `ito-skills/adapters/claude/session-start.sh`

### Codex

- Source: `ito-skills/.codex/ito-skills-bootstrap.md`
- Dest: `~/.codex/instructions/ito-skills-bootstrap.md`

## Open Questions

- Should Codex bootstrap be installed per-project instead of globally (if Codex supports it reliably)?
