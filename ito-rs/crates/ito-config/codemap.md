[Codemap: ito-config]|L0: cascading config load + invocation context; only depends on ito-common; no domain ops

[Entry Points]|src/lib.rs: config/context/output/ito-dir exports |src/config/**: types, defaults, schema, memory/backend/worktree settings
|src/context.rs: ItoContext (resolved per invocation) |src/ito_dir/: .ito dir normalization |src/output/: color/interactivity settings

[Design]|declarative: parse→merge→normalize→expose; runtime behavior downstream in ito-core/adapters; schema must track public config structs

[Gotchas]|config changes need schema+test updates |backend/worktree settings are contract-sensitive (instruction templates render them) |keep path values portable in templates

[Tests]|targeted: cargo test -p ito-config |schema changes: also run make check
