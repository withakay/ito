# Tasks

- [ ] Identify adapter modules with non-trivial logic (beyond parsing/presentation).
- [ ] Extract orchestration logic into `ito-core` use-cases.
- [ ] Update CLI/Web handlers to call use-cases and render results.
- [ ] Ensure `ito-core` has no adapter framework dependencies (`clap`, `crossterm`, `axum`).
- [ ] Add regression coverage for refactored commands.
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
