## Why

The ito-skills distribution mechanism violates agentskills.io specifications and explicit project constraints. Skills are currently placed in a subfolder (`skills/ito-skills/`) rather than flat under `skills/`, symlinks were documented despite being explicitly forbidden, and skills are only distributed to OpenCode - not Claude or Codex harnesses.

## What Changes

- **BREAKING**: Remove the `ito-skills` subfolder nesting - skills move from `.opencode/skills/ito-skills/<skill>/` to `.opencode/skills/ito-<skill>/`
- Add `ito-` prefix to all skill names (e.g., `brainstorming` → `ito-brainstorming`)
- Distribute skills to all three harnesses: OpenCode, Claude, and Codex
- Remove symlink documentation from `ito-skills/docs/README.opencode.md`
- Update embedded template assets to use flat, prefixed structure

## Capabilities

### New Capabilities

- `flat-skill-distribution`: Skills are copied (not symlinked) directly under the harness skills folder with `ito-` prefix, complying with agentskills.io spec
- `multi-harness-skill-support`: Skills are distributed to OpenCode, Claude (`.claude/skills/`), and Codex (`.codex/skills/`) harnesses

### Modified Capabilities

<!-- None - these are new capabilities addressing previously broken behavior -->

## Impact

- **distribution.rs**: Major refactor of `opencode_ito_skills_file_paths()` and `opencode_manifests()` to use flat paths with prefix
- **Embedded templates**: Move from `.opencode/skills/ito-skills/<skill>/` to `.opencode/skills/ito-<skill>/`
- **README.opencode.md**: Remove symlink instructions, document correct flat structure
- **Claude/Codex installers**: Add skill distribution (currently only install bootstrap files)
- **Existing users**: Will need to run `ito dist install` again; old nested paths will remain as orphans (document cleanup)
