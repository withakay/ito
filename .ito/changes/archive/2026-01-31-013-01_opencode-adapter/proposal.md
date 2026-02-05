# Change: OpenCode Adapter for Ito Skills

## Why

The vendored `ito-skills` OpenCode plugin assumes it lives inside the cloned repo and resolves skills via relative paths (`../../skills`). This breaks when the plugin is copy-installed to `${OPENCODE_CONFIG_DIR}/plugins/`. Ito needs a Ito-owned OpenCode plugin that is safe to copy-install and reads skills from a stable location.

## What Changes

- Create a Ito-owned OpenCode plugin (`ito-skills.js`) that:
  - Reads skills from `${OPENCODE_CONFIG_DIR}/skills/ito-skills/` (not relative to plugin path)
  - Uses `experimental.chat.system.transform` hook to inject bootstrap content
  - Injects a minimal preamble pointing to `ito agent instruction <artifact>`
  - Includes OpenCode-specific tool-mapping notes only where tools differ from Claude Code
- Define the skill copy destination as `${OPENCODE_CONFIG_DIR}/skills/ito-skills/`
- Plugin remains stateless (no tool interception, no lifecycle hooks beyond prompt transform)

## Capabilities

### New Capabilities

- `opencode-adapter`: OpenCode plugin integration for Ito skills bootstrap

### Modified Capabilities

None

## Impact

- Affected specs: `tool-adapters` (new)
- Affected code:
  - New: `ito-skills/adapters/opencode/ito-skills.js`
  - Embedded in: `ito-rs/crates/ito-templates/assets/`
- Dependencies: Requires 013-05 (distribution) for fetch/copy mechanics
- Parallelization: Can be developed in parallel with 013-02, 013-03; depends on 013-04 for bootstrap artifact

## Parallel Execution Notes

This change can be implemented in parallel with:
- 013-02 (Claude Code integration) - no shared code paths
- 013-03 (Codex bootstrap) - no shared code paths

Soft dependency on:
- 013-04 (bootstrap artifact CLI) - for the `ito agent instruction bootstrap --tool opencode` content
- 013-05 (distribution) - for install/fetch mechanics
