## Approach

`ito-cli` currently exposes `mod app;` from `ito-rs/crates/ito-cli/src/main.rs`, and `app.rs` includes submodules via `#[path = "app/<name>.rs"]`. We will convert to a standard Rust module layout and split large sections into focused modules.

### Proposed module layout

- `ito-rs/crates/ito-cli/src/app/mod.rs`
  - Re-exports the stable entrypoints and help constants currently referenced by `ito-rs/crates/ito-cli/src/main.rs`.
  - Declares submodules (`list`, `status`, etc.) using standard `mod` statements.
- `ito-rs/crates/ito-cli/src/app/entrypoint.rs`
  - `pub(crate) fn main()` and any early process setup (e.g., NO_COLOR handling).
- `ito-rs/crates/ito-cli/src/app/run.rs`
  - `pub(crate) fn run(args: &[String]) -> CliResult<()>`.
- `ito-rs/crates/ito-cli/src/app/help.rs`
  - The large help strings/consts (top-level HELP and per-command help), re-exported from `mod.rs`.
- Existing files remain as-is:
  - `ito-rs/crates/ito-cli/src/app/list.rs`
  - `ito-rs/crates/ito-cli/src/app/status.rs`
  - `ito-rs/crates/ito-cli/src/app/common.rs`
  - `ito-rs/crates/ito-cli/src/app/archive.rs`
  - `ito-rs/crates/ito-cli/src/app/instructions.rs`
  - `ito-rs/crates/ito-cli/src/app/templates.rs`
  - `ito-rs/crates/ito-cli/src/app/show.rs`
  - `ito-rs/crates/ito-cli/src/app/validate.rs`
  - `ito-rs/crates/ito-cli/src/app/ralph.rs`

### Guardrail: 1000 SLOC target

We treat 1000 SLOC per Rust file as the target. To keep this objective and automated, we will implement a regression check that runs in tests or hooks.

Implementation note: SLOC is ambiguous (comments/blank lines). For v1 we can use a strict physical line limit (<= 1000) or a simple “source line” counter that ignores blank lines and comment-only lines. Either approach is acceptable if documented and consistently enforced.

## Rollout

- Refactor is done in small, verifiable moves: introduce `app/mod.rs` first, then migrate `main()`/`run()`/help constants, then remove `app.rs`.
- Verify behavior via existing CLI tests and `make test`.
