# Source Guide: ito-cli

## Responsibility
`ito-cli` owns the `ito` command-line surface: argument parsing, command routing, terminal output, prompts, and integration-test coverage for user-visible behavior. It should delegate domain work to `ito-core` rather than reimplementing workflow semantics.

## Entry Points
- `src/main.rs`: minimal binary entrypoint that calls `app::main()`.
- `src/cli.rs` and `src/cli/**`: Clap argument definitions and subcommand shapes.
- `src/app/mod.rs`, `src/app/entrypoint.rs`, `src/app/run.rs`: application dispatch and runtime setup.
- `src/commands/**`: command-specific handlers and UI glue.
- `tests/**`: end-to-end CLI behavior using the compiled candidate binary.

## Design
- Keep parsing and display here; keep state transitions and repository behavior in `ito-core`.
- CLI handlers usually resolve config/context, call a core use-case, and format output/errors.
- Integration tests are the primary regression guard for installed files, prompts, and command output.

## Flow
1. `main` calls `app::main`.
2. CLI args are parsed into command structs.
3. App/runtime code builds context from `ito-config` and dispatches to `commands` or `ito-core`.
4. Results are rendered to stdout/stderr, often with JSON mode variants.

## Integration
- Depends on `ito-core`, `ito-config`, `ito-domain`, `ito-templates`, `ito-logging`, and test support.
- Many tests use `ito-test-support::run_rust_candidate` for deterministic HOME/XDG/git environment setup.

## Gotchas
- Avoid embedding business rules in CLI handlers; that makes backend/web parity harder.
- Large integration test files can trip the max-lines guardrail when edited.
- Non-interactive command paths must avoid prompts and should expose flags for required choices.

## Tests
- Targeted: `cargo test -p ito-cli --test <test_file> <filter>`.
- Broad: `cargo test -p ito-cli`.
- Full repo: `make check`.
