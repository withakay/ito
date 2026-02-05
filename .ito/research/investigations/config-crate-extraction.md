# Extracting Config Into Its Own Crate (Go/No-Go)

## Context

Config is currently implemented inside `ito-core` (Rust) and is intentionally small today (e.g. `projectPath` resolution via `ito.json` + global `config.json`).

However, the direction of travel is that config will grow:

- project-level settings beyond `projectPath` (e.g. `serve` port/bind/token)
- AI-related settings (tools/models/budgets/preferences)
- cascading project config sources with merging (e.g. `ito.json`, `.ito.json`, `<itoDir>/config.json`, `$PROJECT_DIR/config.json`)

That pushes config toward being a shared foundation rather than an incidental helper module.

## Go/No-Go Recommendation

### No-Go (for now) if

- config remains limited to `projectPath` + a couple of small flags
- config is only used by one crate (`ito-core`) and does not need to be shared
- the team wants to minimize crate count and public API surface area

In this case, a simpler move is to keep config in `ito-core` but refactor it for clarity (submodules, stronger tests, clearer merge rules) without creating a new crate.

### Go (recommended) if

- we implement cascading config with merging across multiple project files
- multiple crates need config resolution consistently (CLI, workflows, agent tooling, future server/serve)
- we want configuration provenance ("this value came from X") and testability that is easiest to get from a small, focused crate

Given current plans (cascading merge + more settings), extracting config is likely a net win.

## Pros

- **Boundary clarity**: config parsing/merging/provenance becomes a distinct concern instead of being scattered.
- **Reuse**: `ito-cli`, `ito-core`, and future crates can share the same resolution and merge logic.
- **Testability**: pure functions for merge + deterministic path resolution are easy to unit test; filesystem reads can be isolated behind a trait.
- **Stability**: encourages a stable schema and explicit merge semantics (objects vs arrays).
- **Provenance**: easier to return "effective config + sources" for debugging (`ito config explain`).

## Cons

- **Crate sprawl**: more workspace crates means more overhead (Cargo wiring, versioning, compilation).
- **API surface**: even if "internal", a crate boundary tends to calcify types and behaviors.
- **Dependency traps**: config often wants helpers (IO, path discovery, repo root detection). If those live in `ito-core`, a naive extraction can create cycles.
- **Premature abstraction risk**: if config stays small, the crate is pure ceremony.

## Design Options

### Option A: Stay in `ito-core` (refactor only)

Keep `ito_core::config` but:

- add explicit merge semantics (deep-merge objects; scalars override; arrays replace)
- add config provenance tracking
- add tests for precedence and invalid input handling

This is the lowest-risk path.

### Option B: New crate `ito-config` (recommended if cascading merge ships)

Create `ito-rs/crates/ito-config/` with:

- types: `Config`, `ConfigSource`, `ConfigProvenance`
- resolution: `resolve_paths(ctx, project_root) -> ConfigPaths`
- loading: `load_sources(paths) -> Vec<ConfigSource>`
- merging: `merge(sources) -> (Config, ConfigProvenance)`

Keep dependencies minimal:

- prefer `std` + `serde` + `serde_json`
- avoid depending on `ito-core`
- accept an IO abstraction (trait or injected closures) so tests can run without touching disk

### Option C: Two crates (`ito-config` + `ito-config-schema`)

Only worth it if schemas become large, versioned, and shared externally. Likely YAGNI for now.

## Cascading Config + Merging (Proposed)

If we adopt cascading configs, make the behavior explicit and testable.

### Proposed project-level sources (low → high precedence)

1. `<repo-root>/ito.json`
1. `<repo-root>/.ito.json`
1. `<itoDir>/config.json`
1. If `PROJECT_DIR` is set: `$PROJECT_DIR/config.json`

Separately, user-level global config can still exist as the default baseline (e.g. per-user defaults), but it should be treated as a distinct source category.

### Merge rules (simple default)

- objects: recursively merge
- scalars: later source overrides earlier
- arrays: later source replaces earlier (no concatenation)

If we later need fancier behavior, add it explicitly (e.g. `mergeStrategy` per key), but do not implicitly concatenate arrays.

### Error handling (recommended)

- invalid JSON in a source:
  - for optional/non-critical sources: warn and ignore
  - for config explicitly requested/edited by the user (e.g. `agent-config set`): fail with a clear error

This avoids silent misconfiguration while still being resilient.

## Migration/Refactor Plan (Low-Risk)

1. Refactor current config code into a clearly testable shape (even if still in `ito-core`).
1. Implement cascading sources + merge rules + provenance.
1. If more than one crate needs config, extract into `ito-config` with a stable internal API.

## Decision

- If the next concrete feature requires cascading merge and more keys (serve/agent settings): **Go** on extracting config into a crate.
- If not: **No-Go** for now; do the refactor-only approach and revisit when config expands.
