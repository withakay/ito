# Tasks

- [ ] Introduce a core process execution boundary (trait + default implementation).
- [ ] Migrate existing process execution call sites to use the boundary.
- [ ] Ensure `ito-domain` contains no process spawning.
- [ ] Add tests for:
  - [ ] capturing stdout/stderr
  - [ ] non-zero exit codes
  - [ ] missing executable / spawn failure
- [ ] Run `cargo test --workspace`.
- [ ] Run `make arch-guardrails`.
