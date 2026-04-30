# ito-cli — L3 Adapter

Thin CLI adapter: arg parsing (clap), command dispatch, output formatting, interactive prompts, diagnostic error reporting (miette). Produces the `ito` binary.
See [`ito-rs/AGENTS.md`](../../AGENTS.md) for workspace guidance. See [`.ito/architecture.md`](../../../.ito/architecture.md) for arch context.

## Purpose
Translate CLI args → ito-core calls → format results for terminal. **Thin adapter — all logic in ito-core.**

## Structure
```
src/
├── main.rs          cli.rs  cli_error.rs  diagnostics.rs  runtime.rs  util.rs
├── app/             # Command implementations (archive, init, list, show, validate, validate_repo, etc.)
└── commands/        # Subcommand modules (audit, config, create, tasks, workflow, etc.)
```
`app/validate_repo.rs` → dispatches to ito-core::validate_repo engine (see `.ito/architecture.md#repository-validation-rules`)

## Dependencies
|required: ito-core (all business logic), ito-common, ito-config, ito-logging, ito-templates
|optional: ito-web (gated on `web` feature, default: enabled)

## Constraints
**MUST NOT:** depend on ito-domain (route through ito-core) | contain business logic | define domain types/repo impls
**MUST:** depend on ito-core | convert CoreError → miette diagnostics | own all presentation (colours, formatting, prompts, progress)

## Common Mistakes
|importing from ito-domain → re-export through ito-core instead
|logic in command handlers → belongs in ito-core (if/else chains or loops over data = wrong layer)
|forgetting --json support for list/show commands

## Quality
```bash
make check && make test && make arch-guardrails
```
|rust-quality-checker: style |codex-review: catch business logic that should be in ito-core
|rust-test-engineer: CLI integration tests via ito-test-support PTY helpers + snapshot normalization
