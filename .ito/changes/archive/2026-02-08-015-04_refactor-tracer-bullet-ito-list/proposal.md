# Refactor: Tracer bullet onion layering via `ito list`

## Why

The onion refactor needs an end-to-end “proof” path that demonstrates the target dependency direction and testing approach without requiring a big-bang migration.

`ito list` is a high-traffic command and a good tracer bullet because it exercises discovery/status logic and currently risks coupling adapters and domain code to direct filesystem concerns.

## Depends on

- 015-01_refactor-arch-guardrails (guardrails entrypoint and enforcement)
- 015-02_refactor-cli-web-decouple (adapter independence)
- 015-03_update-rust-workspace-specs (spec alignment)

## What

- Introduce a core use-case for listing changes (the default `ito list` behavior) that:
  - owns orchestration and I/O
  - returns a stable, typed summary to adapters
- Update `ito-cli` to call the core use-case and keep presentation/formatting in the CLI.
- Ensure the refactor preserves existing behavior:
  - default output
  - filtering flags (`--ready`, `--pending`, `--partial`, `--completed`)
  - sorting (`--sort`)
  - JSON output (`--json`)

This change is explicitly a tracer bullet: it establishes the pattern and boundaries for subsequent migrations.

## Out of scope

- Refactoring `ito list --modules` and `ito list --specs` (can follow once the changes-path pattern is proven).
- Broad domain purification (handled in later changes).

## Verification

- `cargo test --workspace`
- `make arch-guardrails` (once 015-01 is implemented)
- CLI regression tests for `ito list` (text and JSON)
