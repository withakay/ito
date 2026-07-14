<!-- ITO:START -->
## Why

OpenCode Ito agents such as `ito-general` and `ito-orchestrator` are intended to be top-level agents, but existing installed agent files can retain stale `mode: subagent` or `subagent:` frontmatter across `ito init --update` / `ito update`. That makes the generated agent surface inconsistent with the current embedded templates and can leave the OpenCode harness treating these agents as subagents even after reinstall or update.

## What Changes

- Strip stale OpenCode-only subagent frontmatter from existing `.opencode/agents/*.md` files during agent template refresh.
- Add focused installer and init regression tests covering fresh OpenCode agent installs and updates from legacy frontmatter.
- Keep the existing non-destructive body-preservation behavior for markerless and partially marked agent files while still normalizing the stale subagent metadata.

## Change Shape

- **Type**: fix
- **Risk**: low
- **Stateful**: no
- **Public Contract**: none
- **Design Needed**: no
- **Design Reason**: The change is a small frontmatter-normalization fix in the installer/update path with focused regression coverage.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `rust-installers`: OpenCode agent template installs and updates should not leave stale subagent metadata on top-level Ito agent files.

## Impact

- Agent template update logic in `ito-rs/crates/ito-core/src/installers/mod.rs`.
- OpenCode installer regressions in `ito-rs/crates/ito-cli/tests/init_more.rs`.
- No change to the embedded OpenCode agent markdown bodies themselves; the fix is in how existing installed frontmatter is normalized.
<!-- ITO:END -->
