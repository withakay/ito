# Tasks

- [ ] Add/clarify a `ModuleRepository` port/interface in `ito-domain`.
- [ ] Implement a filesystem-backed repository in `ito-core`.
- [ ] Migrate call sites to use the port/interface.
- [ ] Ensure `ito-domain` has no direct `std::fs` usage for module repository behavior.
- [ ] Add tests for module metadata parsing and listing.
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
