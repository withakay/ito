# Tasks

- [ ] Audit existing error types and error construction sites across `ito-domain`, `ito-core`, and adapters.
- [ ] Define/normalize domain error types (framework-agnostic).
- [ ] Define/normalize core/use-case error types that:
  - [ ] translate filesystem errors into contextual errors
  - [ ] translate schema parsing errors into contextual errors
  - [ ] translate process execution errors into contextual errors
- [ ] Ensure adapters only format/present errors (no business logic).
- [ ] Add guardrails for "no diagnostic/UI frameworks in domain".
- [ ] Add tests for representative error paths.
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
