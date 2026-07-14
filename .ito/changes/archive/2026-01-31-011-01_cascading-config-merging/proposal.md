## Why

Ito configuration is starting to span multiple concerns (project layout, agent/tool settings, future `serve` settings). We need a predictable, testable cascading config system that can merge multiple project config files without hardcoding a single location.

## What Changes

- Support cascading project configuration with deterministic precedence across multiple files:
  - `<repo-root>/ito.json`
  - `<repo-root>/.ito.json`
  - `<itoDir>/config.json`
  - `$PROJECT_DIR/config.json` (when `PROJECT_DIR` is set)
- Implement deep-merge semantics for JSON objects (later sources override earlier; arrays replaced).
- Extend ito directory selection to consider `.ito.json` in addition to `ito.json`.
- Expose an API in `ito-core` to load the merged effective project config.
- Add tests for precedence, merging, and path resolution.
- Update documentation/specs to match the implemented behavior.

## Capabilities

### New Capabilities

- `cascading-project-config`: Load and merge multiple project config sources with clear precedence.

### Modified Capabilities

- `global-config`: continues to exist as a user-level baseline but project config resolution becomes richer.

## Impact

- `ito-rs/crates/ito-core/src/config/mod.rs`: add merged project config loader + merge semantics.
- `ito-rs/crates/ito-core/src/ito_dir/mod.rs`: incorporate `.ito.json` for `projectPath` resolution.
- Docs/specs: align config documentation and agent-config expectations with JSON project config.
