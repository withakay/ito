# Tasks

- [ ] Add/clarify a `TaskRepository` port/interface in `ito-domain`.
- [ ] Implement a filesystem-backed repository in `ito-core`.
- [ ] Migrate call sites (including list/show) to use the repository boundary.
- [ ] Ensure `ito-domain` has no direct `std::fs` usage for task repository behavior.
- [ ] Add tests:
  - [ ] checkbox format parsing
  - [ ] enhanced format parsing
  - [ ] missing tasks.md returns `(0, 0)`
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
