# Refactor: Thin adapters (CLI/Web) and keep core framework-free

## Why

- Onion layering is easiest to maintain when adapters are thin and `ito-core` owns orchestration.
- Framework dependencies (CLI/web) leaking into core makes testing harder and blurs boundaries.
- Establishing a clear pattern in one place reduces repeated refactors in every new command.

## What

- Move non-trivial command logic out of adapters and into `ito-core` use-cases.
- Ensure adapters are responsible for:
  - parsing input (CLI flags / HTTP requests)
  - presentation (table output / JSON / HTTP responses)
  - composition (wiring implementations)
- Ensure `ito-core` remains framework-free (no clap/crossterm/axum dependencies).

## Scope

- Refactor patterns and boundaries; behavior should remain consistent.

## Depends on

- 015-01_refactor-arch-guardrails
- 015-02_refactor-cli-web-decouple

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
