[Codemap: ito-rs/crates]
|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning for crate selection

[Crates]|root: ./
|ito-cli: CLI parsing, dispatch, terminal behavior → ito-cli/codemap.md
|ito-core: use-cases, adapters, validation, installers, orchestration → ito-core/codemap.md
|ito-domain: domain entities, repo traits, status/traceability → ito-domain/codemap.md
|ito-config: cascading config, invocation context → ito-config/codemap.md
|ito-common: fs, path, ID, matching, URL utils (leaf, no workspace deps) → ito-common/codemap.md
|ito-templates: embedded templates, skills, commands, schemas, rendering → ito-templates/codemap.md
|ito-backend: multi-tenant HTTP adapter → ito-backend/codemap.md
|ito-web: browser UI + terminal adapter → ito-web/codemap.md
|ito-logging: append-only JSONL telemetry → ito-logging/codemap.md
|ito-test-support: test helpers, fake repos (dev-dep only) → ito-test-support/codemap.md

[Design]|domain concepts → ito-domain |executable behavior → ito-core |surface wiring → adapter crates
|template content embedded via ito-templates; installers in ito-core |test helpers in ito-test-support only

[Gotchas]|ito-cli tests exercise ito-core end-to-end via compiled binary
|ito-core is broad; prefer subsystem modules over catch-all files

[Tests]|crate-scoped: cargo test -p <crate> |e2e CLI: cargo test -p ito-cli --test <integration_test>
