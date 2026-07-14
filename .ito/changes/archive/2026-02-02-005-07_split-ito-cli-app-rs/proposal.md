## Why

`ito-rs/crates/ito-cli/src/app.rs` has grown to ~2327 lines, making it hard to navigate, review, and safely change. We want a maintainability standard of at most 1000 SLOC per code file, and this file is the only current offender.

## What Changes

- Refactor `ito-rs/crates/ito-cli/src/app.rs` into a module tree under `ito-rs/crates/ito-cli/src/app/` so no resulting Rust file exceeds the 1000 SLOC target.
- Keep the existing public surface stable (the `app::main()` entrypoint and the help constants re-exported by `ito-rs/crates/ito-cli/src/main.rs`).
- Add a lightweight regression guard (test or lint hook) to prevent the 1000 SLOC limit from regressing for `ito-rs/crates/ito-cli/src/**.rs`.

## Capabilities

### Modified Capabilities

- `repo-precommit-quality-gates`: add a documented quality gate for maximum per-file SLOC.
- `rust-cli-plumbing` (implementation-only): reorganize CLI app code into smaller modules.

## Impact

- Pure refactor (no behavior changes intended), but touches CLI routing/entrypoint files.
- Small build/test impact: module paths change; compile failures are the main risk.
