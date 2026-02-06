# Tasks

- [x] Add/clarify a `ChangeRepository` port/interface in `ito-domain`.
- [x] Implement a filesystem-backed repository in `ito-core`.
- [x] Migrate call sites to use the port/interface (prefer wiring via core use-cases).
- [x] Ensure `ito-domain` has no direct `std::fs` usage for change repository behavior.
- [x] Add tests:
  - [x] unit tests for domain models/computed properties
  - [x] integration-style tests for filesystem repository behavior (temp dirs / fixtures)
- [x] Run `cargo test --workspace`.
- [x] Run `make arch-guardrails`.
