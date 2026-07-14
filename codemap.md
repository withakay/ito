[Codemap: Ito Workspace]
|purpose: Rust workspace for spec-driven design (reviewed proposals, specs, tasks, validation, implementation iteration, templates, and adapters)
|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning for any Ito architecture tasks

[Entry Points]
|Cargo.toml: workspace membership + shared dep versions |Makefile: make check/test/release
|ito-rs/crates/ito-cli/src/main.rs: CLI binary |ito-rs/crates/ito-web/src/main.rs: web binary
|ito-rs/crates/*/codemap.md: crate-level atlas pages
|default product: ito-cli with web; backend + coordination-branch are independent opt-in features

[Layer Design]
|L0: ito-common (utils, leaf), ito-config (config loading)
|L1: ito-domain (data shapes, repo traits), ito-templates (embedded assets), ito-logging (telemetry)
|L2: ito-core (use-cases, adapters, validation, orchestration)
|L3 adapters: ito-cli, ito-web, ito-backend → all via ito-core
|Support: ito-test-support (dev-dep only)
|Flow: CLI args → ito-cli → ito-config ctx + ito-core use-cases → domain repo traits → fs/backend adapters

[Directories]|root: ./
|ito-rs/: Rust workspace → ito-rs/codemap.md
|ito-rs/crates/: crate index → ito-rs/crates/codemap.md
|.ito/: tracked authority for workflow specs, changes, modules, prompts, project config, wiki, and audit evidence → Ito instructions

[Gotchas]
|main/control checkout is read-only; use dedicated worktree for all edits
|docs/ito is retired; do not recreate a published Ito mirror outside tracked .ito authority
|.ito/ .opencode/ .github/ .codex/ are Ito-managed; may be overwritten by ito init/update
|codemap.md = orientation only; verify behavior in source before editing
|refresh codemap.json after codemap updates so agents detect drift

[Tests]|prefer: make check |targeted: cargo test -p <crate> |max-lines guardrail: large-file edits can fail make check
