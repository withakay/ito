# Tasks

- [ ] Add/clarify a `ChangeRepository` port/interface in `ito-domain`.
- [ ] Implement a filesystem-backed repository in `ito-core`.
- [ ] Migrate call sites to use the port/interface (prefer wiring via core use-cases).
- [ ] Ensure `ito-domain` has no direct `std::fs` usage for change repository behavior.
- [ ] Add tests:
  - [ ] unit tests for domain models/computed properties
  - [ ] integration-style tests for filesystem repository behavior (temp dirs / fixtures)
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
