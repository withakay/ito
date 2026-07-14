# Change: Distribution and Fetch Mechanics for Ito Skills

## Why

Normal Ito use (released version) should fetch required adapter files over HTTP from GitHub. Development mode should support copying from `./ito-skills/` without symlinks. This ensures adapters can be installed/updated consistently across both scenarios.

## What Changes

- Implement GitHub URL scheme for fetching raw files:
  - Tagged: `https://raw.githubusercontent.com/withakay/ito/<ito-version-tag>/ito-skills/<path>`
  - Fallback: `https://raw.githubusercontent.com/withakay/ito/main/ito-skills/<path>`
- Implement per-user cache to avoid repeated downloads:
  - Cache location: `~/.config/ito/cache/ito-skills/<ito-version-tag>/<path>`
- Update `ito init` to:
  - Accept `--tools opencode,claude,codex` flag (or similar)
  - Fetch/copy required adapter files for selected tools
- Update `ito update` to refresh managed adapter files for the current version
- Implement development local source mode:
  - Detect `./ito-skills/` in repo root
  - Copy files (no symlinks) into cache/install locations
  - Use `main`-equivalent semantics (latest local working tree)

## Capabilities

### New Capabilities

- `distribution`: Fetch, cache, and install mechanics for Ito skill adapters

### Modified Capabilities

- `ito-init`: Extended to install tool-specific adapters
- `ito-update`: Extended to refresh adapter files

## Impact

- Affected specs: `distribution` (new), `ito-init` (modified), `ito-update` (modified)
- Affected code:
  - `ito-rs/crates/ito-core/src/installers/`
  - `ito-rs/crates/ito-cli/` (init/update commands)
  - New: HTTP fetch utilities, cache management
- This is a dependency for 013-01, 013-02, 013-03 (they need install mechanics)
- Parallelization: Can be developed in parallel; adapters can be tested with manual file copies

## Parallel Execution Notes

This change provides infrastructure for:
- 013-01 (OpenCode adapter) - installs plugin and skills
- 013-02 (Claude Code integration) - installs skills/hooks
- 013-03 (Codex bootstrap) - installs bootstrap files

All adapter tracks can proceed in parallel by:
1. Defining their required file sets
2. Implementing the adapter logic
3. This change delivers the install/fetch plumbing

For testing during parallel development, adapters can be manually copied to their destinations.

## File Manifest (Per Tool)

### OpenCode
- Plugin: `ito-skills/adapters/opencode/ito-skills.js` -> `${OPENCODE_CONFIG_DIR}/plugins/ito-skills.js`
- Skills: `ito-skills/skills/` -> `${OPENCODE_CONFIG_DIR}/skills/ito-skills/`

### Claude Code
- Skill: `.claude/skills/ito-workflow.md` (via templates)
- Optional hook: `ito-skills/adapters/claude/session-start.sh`

### Codex
- Bootstrap: `ito-skills/.codex/ito-skills-bootstrap.md` -> `~/.codex/instructions/ito-skills-bootstrap.md`
