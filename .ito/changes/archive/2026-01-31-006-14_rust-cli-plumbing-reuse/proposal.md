## Why

The Rust CLI implementation (`ito-rs/crates/ito-cli/src/main.rs`) contains a large amount of repeated logic and repeated patterns:

- ad-hoc `fail(...)` / `eprintln!(...)` / `exit(1)` flows
- repeated formatting of diagnostics (path + optional line) across multiple subcommands
- repeated "read tasks.md -> parse -> block on errors" gating
- repeated path formatting and `.ito/...` path construction

This duplication makes changes slower and riskier (one behavior is updated in one place but not another) and it increases the chance of inconsistent UX across commands.

## What Changes

- Introduce a small, reusable "CLI plumbing" layer for `ito-cli`:
  - a single error type and `Result` flow for command handlers
  - shared helpers for consistent diagnostics printing (including `path:line` when available)
  - shared helpers for consistent exit codes and user-facing error formatting
- Refactor existing command handlers to use the shared plumbing without changing core behavior.

## Capabilities

### New Capabilities

- `rust-cli-plumbing`

### Modified Capabilities

(none)

## Impact

- User-visible behavior should remain the same, but error messages become more consistent and actionable.
- Internal code becomes easier to extend (less copy/paste) and less error-prone.
- No changes to on-disk formats or `.ito/` layout.
