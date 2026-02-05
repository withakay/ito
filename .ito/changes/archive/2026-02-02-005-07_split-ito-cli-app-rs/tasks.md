# Tasks for: 005-07_split-ito-cli-app-rs

## Checklist

- [x] Task 1.1: Convert to standard `app/` module layout
- [x] Task 1.2: Move large help constants out of the module root
- [x] Task 1.3: Split entrypoint and dispatch logic
- [x] Task 2.1: Add a size regression check
- [x] Task 3.1: Confirm the 1000-limit definition (checkpoint)

## Wave 1: Split app.rs into modules

### Task 1.1: Convert to standard `app/` module layout

- **Action**:
  - Replace `ito-rs/crates/ito-cli/src/app.rs` with `ito-rs/crates/ito-cli/src/app/mod.rs`.
  - Remove `#[path = "app/<file>.rs"]` usage; use `mod <name>;` from `ito-rs/crates/ito-cli/src/app/mod.rs`.
  - Keep `ito-rs/crates/ito-cli/src/main.rs` working without changes to its public re-exports.
- **Verify**: `make test`
- **Done When**: build and tests pass; no behavior changes intended.

### Task 1.2: Move large help constants out of the module root

- **Action**:
  - Move top-level HELP and per-command help strings into `ito-rs/crates/ito-cli/src/app/help.rs`.
  - Re-export from `ito-rs/crates/ito-cli/src/app/mod.rs` so existing call sites are stable.
- **Verify**: `make test`
- **Done When**: `ito help` output unchanged (byte-for-byte if tests exist).

### Task 1.3: Split entrypoint and dispatch logic

- **Action**:
  - Move `pub(crate) fn main()` to `ito-rs/crates/ito-cli/src/app/entrypoint.rs`.
  - Move `run(...)` to `ito-rs/crates/ito-cli/src/app/run.rs`.
  - Keep shared helpers in small, purpose-named modules if needed (e.g., `dispatch.rs`).
- **Verify**: `make test`
- **Done When**: no resulting Rust file in `ito-rs/crates/ito-cli/src/` exceeds the 1000 SLOC target.

## Wave 2: Add regression guard

### Task 2.1: Add a size regression check

- **Action**: Add a test (preferred) or a pre-commit hook that fails if any `ito-rs/crates/ito-cli/src/**/*.rs` file exceeds the configured per-file limit.
- **Verify**: `make test` and `prek run --all-files`
- **Done When**: guard fails on an artificially oversized file and passes for the repo.

## Checkpoint

### Task 3.1: Confirm the 1000-limit definition

- **Type**: checkpoint
- **Question**: should the guard use physical lines (strict) or SLOC (ignoring blanks/comments)?
- **Default**: physical line limit (<= 1000) for deterministic enforcement.
