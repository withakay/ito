# Tasks

- [x] Identify adapter modules with non-trivial logic (beyond parsing/presentation).
- [x] Extract orchestration logic into `ito-core` use-cases.
- [x] Update CLI/Web handlers to call use-cases and render results.
- [x] Ensure `ito-core` has no adapter framework dependencies (`clap`, `crossterm`, `axum`).
- [x] Add regression coverage for refactored commands.
- [x] Run `cargo test --workspace`.
- [x] Run `make arch-guardrails`.
