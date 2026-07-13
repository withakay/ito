<!-- ITO:START -->
## Why

Tmux support is unrelated to Ito's core job of turning reviewed specifications into implementation work, yet it adds a bundled skill with executable scripts, configuration schema, init prompts, viewer code, documentation, and cross-harness tests. The integration increases the default agent surface and maintenance burden without contributing to the spec-driven lifecycle.

Ito should stop owning terminal multiplexing policy. Users and agent environments can still use tmux independently; Ito will simply no longer install, configure, recommend, or invoke it.

## What Changes

- Remove the embedded `ito-tmux` skill and its helper scripts from template assets and every harness installation surface.
- Remove `tools.tmux.enabled`, its default, schema entries, config resolution, init/update prompts and flags, and tmux-specific diagnostics.
- Remove the `tmux-nvim` proposal viewer and viewer registry/probing code.
- Remove tmux references from current Ito prompts, commands, docs, tests, and generated assets while retaining archived change history and changelog entries as history.
- Add managed legacy cleanup entries so `ito init --upgrade` and update flows prune obsolete Ito-owned tmux files without deleting user-owned tmux configuration outside Ito-managed paths.
- Keep ordinary external editor/viewer support and all iteration/Ralph behavior unchanged.

## Change Shape
- **Type**: refactor
- **Risk**: medium
- **Stateful**: no
- **Public Contract**: cli, config, jsonschema
- **Design Needed**: yes
- **Design Reason**: Complete removal crosses public config, CLI flags/viewer identifiers, embedded assets, cleanup semantics, generated schemas, and multiple harness adapters.

## Capabilities
### New Capabilities

None.

### Modified Capabilities
- `ito-tmux-skill`: Remove the Ito-managed tmux skill, scripts, and installation/update contract.
- `tools-config`: Remove the tmux tool preference and the now-empty tmux-specific tools namespace contract.
- `global-config`: Remove `tools.tmux.enabled` from supported global configuration while retaining worktree defaults.
- `config-schema`: Regenerate the tracked schema without tmux configuration keys.

## Impact

- Rust configuration types/defaults/schema generation in `ito-config` and init/update CLI flows.
- Viewer implementation and registry in `ito-core` plus `ito view` argument validation.
- Embedded assets, manifest inventories, executable-bit handling, legacy cleanup, and harness installer tests in `ito-templates`/`ito-core`.
- Managed skills, commands, agent prompts, reference docs, wiki synthesis, config fixtures, and current specs.
- A breaking removal for scripts or configs that explicitly reference `tools.tmux.enabled`, `ito-tmux`, or `--viewer tmux-nvim`; external tmux use remains unaffected.
<!-- ITO:END -->
