# Tasks

- [x] Add/clarify a `TaskRepository` port/interface in `ito-domain`.
- [x] Implement a filesystem-backed repository in `ito-core`.
- [x] Migrate call sites (including list/show) to use the repository boundary.
- [x] Ensure `ito-domain` has no direct `std::fs` usage for task repository behavior.
- [x] Add tests:
  - [x] checkbox format parsing
  - [x] enhanced format parsing
  - [x] missing tasks.md returns `(0, 0)`
- [x] Run `cargo test --workspace`.
- [x] Run `make arch-guardrails`.
