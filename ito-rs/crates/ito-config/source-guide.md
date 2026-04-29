# Source Guide: ito-config

## Responsibility
`ito-config` resolves Ito configuration and invocation context. It reads cascading config files, normalizes the Ito directory name, derives output behavior, and exposes typed settings used by CLI/core/backend/web code.

## Entry Points
- `src/lib.rs`: public exports for config, context, output, and Ito dir helpers.
- `src/config/**`: config types, defaults, schema generation, memory/backend/worktree settings.
- `src/context.rs`: resolved invocation context.
- `src/ito_dir`: `.ito` directory normalization and path resolution.
- `src/output`: color/interactivity output settings.

## Design
- Keep this crate declarative: parse, merge, normalize, and expose settings.
- Runtime behavior belongs in `ito-core` or adapters after config is resolved.
- Schema generation must track public config structs.

## Flow
1. Callers request a resolved config/context for a project root.
2. Config files and defaults are merged in precedence order.
3. Typed values drive repository mode, worktree strategy, memory provider, backend, and output behavior.

## Integration
- Used by every adapter and much of `ito-core`.
- Backend and worktree settings are contract-sensitive because instruction templates render them.

## Gotchas
- Config changes often require schema/test updates.
- Keep path values portable in committed templates and prompts.

## Tests
- Targeted: `cargo test -p ito-config`.
- For schema changes, also run config-schema checks via `make check`.
