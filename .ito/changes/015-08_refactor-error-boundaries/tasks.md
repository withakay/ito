# Tasks

- [x] Audit existing error types and error construction sites across `ito-domain`, `ito-core`, and adapters.
- [x] Define/normalize domain error types (framework-agnostic).
- [x] Define/normalize core/use-case error types that:
  - [x] translate filesystem errors into contextual errors
  - [x] translate schema parsing errors into contextual errors
  - [x] translate process execution errors into contextual errors
- [x] Ensure adapters only format/present errors (no business logic).
- [x] Add guardrails for "no diagnostic/UI frameworks in domain".
- [x] Add tests for representative error paths.
- [x] Run `cargo test --workspace`.
- [x] Run `make arch-guardrails`.
