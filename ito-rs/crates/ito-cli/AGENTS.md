# ito-cli — Layer 3 (Adapter)

Thin CLI adapter. Owns argument parsing (`clap`), command dispatch, output formatting, interactive prompts, and diagnostic error reporting (`miette`). Produces the `ito` binary.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Translate user intent from CLI arguments into `ito-core` function calls, and format the results for terminal output. This crate must be a **thin adapter** — all logic lives in `ito-core`.

## Structure

```
src/
├── main.rs                 # Entry point
├── cli.rs                  # Cli struct, Commands enum (clap derives)
├── cli_error.rs            # CoreError → miette conversion
├── diagnostics.rs          # Diagnostic formatting helpers
├── runtime.rs              # Tokio runtime setup
├── util.rs                 # CLI utility functions
├── app/                    # Command implementations (archive, init, list, show, validate, etc.)
└── commands/               # Subcommand modules (audit, config, create, tasks, workflow, etc.)
```

## Workspace Dependencies

- `ito-core` — **required edge** (all business logic)
- `ito-common` — path utilities
- `ito-config` — context resolution
- `ito-logging` — telemetry
- `ito-templates` — template access for init/update
- `ito-web` — **optional**, gated on `web` feature (default: enabled)

## Feature Flags

- `web` (default: enabled) — enables the `serve` subcommand and `ito-web` dependency

## Architectural Constraints

### MUST NOT

- Depend on `ito-domain` directly (must route through `ito-core` — enforced by `make arch-guardrails`)
- Contain business logic — all logic delegates to `ito-core`
- Define domain types or repository implementations

### MUST

- Depend on `ito-core` (required edge, enforced by guardrails)
- Convert `CoreError` into `miette` diagnostic reports
- Own all presentation concerns: colours, formatting, interactive prompts, progress bars
- Remain a thin adapter — if a function is getting complex, the logic probably belongs in `ito-core`

## Common Mistakes to Watch For

1. **Importing from `ito-domain`** — if you need a domain type, re-export it through `ito-core` instead.
2. **Putting logic in command handlers** — command handlers should parse args, call core, format output. If you're writing `if/else` chains or loops over data, that logic belongs in `ito-core`.
3. **Forgetting `--json` support** — commands that list or show data should support `--json` for machine-readable output.

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests (including CLI integration tests)
make arch-guardrails    # verify no ito-domain dependency
```

Use the `rust-quality-checker` subagent for style compliance. Use the `codex-review` subagent before committing to catch any business logic that should be in `ito-core`. Use the `rust-test-engineer` subagent when adding CLI integration tests — `ito-test-support` provides PTY helpers and snapshot normalization.
