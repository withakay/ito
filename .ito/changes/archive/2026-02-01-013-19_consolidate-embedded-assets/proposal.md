## Why

The `ito-skills/` directory and per-harness template directories contained duplicated skill, command, and adapter files. Skills were stored multiple times (once per harness), commands/prompts had inconsistent frontmatter formats, and runtime downloading added unnecessary complexity. This made maintenance difficult and led to harnesses getting out of sync.

## What Changes

### Consolidated Asset Structure

Moved all distributable assets to `ito-rs/crates/ito-templates/assets/`:

| New Location | Contents | Notes |
|--------------|----------|-------|
| `assets/skills/` | All skills (general + workflow) | Single source of truth |
| `assets/adapters/` | Harness-specific adapters | session-start.sh, plugins, etc. |
| `assets/commands/` | All commands/prompts | Single source of truth |

### Removed Duplication

- Deleted `ito-skills/` directory entirely (skills moved to assets/skills/)
- Removed per-harness skill directories from templates
- Removed per-harness command directories from templates
- Consolidated workflow skills (ito, ito-apply, etc.) into shared assets/skills/

### Updated Distribution Logic

Modified `distribution.rs` to:
- Read skills, adapters, and commands from embedded assets
- Copy to correct harness paths at install time:
  - Claude: `.claude/skills/`, `.claude/commands/`
  - OpenCode: `.opencode/skills/`, `.opencode/commands/`
  - Codex: `.codex/skills/`, `.codex/prompts/`
  - GitHub: `.github/skills/`, `.github/prompts/` (with `.prompt.md` suffix)
- Add `ito-` prefix only to skills that don't already have it

### Fixed Frontmatter

Standardized YAML frontmatter for all commands:
```yaml
---
name: ito-apply
description: Implement an approved Ito change and keep tasks in sync.
category: Ito
tags: [ito, apply]
---
```

### Updated Documentation

- Updated `ito-rs/crates/ito-templates/AGENTS.md` with:
  - New assets structure documentation
  - Guidance on keeping harness files in sync
  - Frontmatter format requirements per harness
- Updated root `AGENTS.md` to reference templates documentation

## Capabilities

### New Capabilities

None - this is a consolidation/cleanup change.

### Modified Capabilities

- **ito-init**: Now installs all assets from embedded binary (no runtime downloads)
- **ito-update**: Same behavior, using consolidated assets

## Impact

- **Code**: `ito-templates/src/lib.rs`, `distribution.rs`, `installers/mod.rs`
- **Assets**: Complete restructure of `ito-rs/crates/ito-templates/assets/`
- **Removed**: `ito-skills/` directory
- **Risk**: Low - all tests pass, functionality preserved
- **Dependencies**: None
